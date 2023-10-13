use re_arrow_store::TimeRange;
use re_query::{range_archetype, QueryError};
use re_types::{
    archetypes::TimeSeriesScalar,
    components::{Color, Radius, Scalar, ScalarScattering, Text},
    Archetype, ComponentNameSet,
};
use re_viewer_context::{
    AnnotationMap, DefaultColor, NamedViewSystem, SpaceViewSystemExecutionError, ViewPartSystem,
    ViewQuery, ViewerContext,
};

#[derive(Clone, Debug)]
pub struct PlotPointAttrs {
    pub label: Option<String>,
    pub color: egui::Color32,
    pub radius: f32,
    pub scattered: bool,
}

impl PartialEq for PlotPointAttrs {
    fn eq(&self, rhs: &Self) -> bool {
        let Self {
            label,
            color,
            radius,
            scattered,
        } = self;
        label.eq(&rhs.label)
            && color.eq(&rhs.color)
            && radius.total_cmp(&rhs.radius).is_eq()
            && scattered.eq(&rhs.scattered)
    }
}

impl Eq for PlotPointAttrs {}

#[derive(Clone, Debug)]
struct PlotPoint {
    time: i64,
    value: f64,
    attrs: PlotPointAttrs,
}

#[derive(Clone, Copy, Debug)]
pub enum PlotSeriesKind {
    Continuous,
    Scatter,
}

#[derive(Clone, Debug)]
pub struct PlotSeries {
    pub label: String,
    pub color: egui::Color32,
    pub width: f32,
    pub kind: PlotSeriesKind,
    pub points: Vec<(i64, f64)>,
}

/// A scene for a time series plot, with everything needed to render it.
#[derive(Default, Debug)]
pub struct TimeSeriesSystem {
    pub annotation_map: AnnotationMap,
    pub lines: Vec<PlotSeries>,
}

impl NamedViewSystem for TimeSeriesSystem {
    fn name() -> re_viewer_context::ViewSystemName {
        "TimeSeries".into()
    }
}

impl ViewPartSystem for TimeSeriesSystem {
    fn required_components(&self) -> ComponentNameSet {
        TimeSeriesScalar::required_components()
            .iter()
            .map(ToOwned::to_owned)
            .collect()
    }

    fn indicator_components(&self) -> ComponentNameSet {
        std::iter::once(TimeSeriesScalar::indicator().name()).collect()
    }

    fn execute(
        &mut self,
        ctx: &mut ViewerContext<'_>,
        query: &ViewQuery<'_>,
        _context: &re_viewer_context::ViewContextCollection,
    ) -> Result<Vec<re_renderer::QueueableDrawData>, SpaceViewSystemExecutionError> {
        re_tracing::profile_function!();

        self.annotation_map.load(
            ctx,
            &query.latest_at_query(),
            query.iter_entities_for_system(Self::name()).map(|(p, _)| p),
        );

        match self.load_scalars(ctx, query) {
            Ok(_) | Err(QueryError::PrimaryNotFound(_)) => Ok(Vec::new()),
            Err(err) => Err(err.into()),
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl TimeSeriesSystem {
    fn load_scalars(
        &mut self,
        ctx: &ViewerContext<'_>,
        query: &ViewQuery<'_>,
    ) -> Result<(), QueryError> {
        re_tracing::profile_function!();

        let store = &ctx.store_db.entity_db.data_store;

        for (ent_path, _ent_props) in query.iter_entities_for_system(Self::name()) {
            let mut points = Vec::new();
            let annotations = self.annotation_map.find(ent_path);
            let annotation_info = annotations
                .resolved_class_description(None)
                .annotation_info();
            let default_color = DefaultColor::EntityPath(ent_path);

            let query = re_arrow_store::RangeQuery::new(
                query.timeline,
                TimeRange::new(i64::MIN.into(), i64::MAX.into()),
            );

            let arch_views: Vec<_> = {
                let now = std::time::Instant::now();
                re_tracing::profile_scope!("store_walk");
                let x = range_archetype::<TimeSeriesScalar, { TimeSeriesScalar::NUM_COMPONENTS }>(
                    store, &query, ent_path,
                )
                .collect();
                eprintln!("store_walk in {:?}", now.elapsed());
                x
            };

            {
                let now = std::time::Instant::now();
                re_tracing::profile_scope!("joins");
                for (time, arch_view) in arch_views {
                    let Some(time) = time else {
                        continue;
                    }; // scalars cannot be timeless

                    for (scalar, scattered, color, radius, label) in itertools::izip!(
                        arch_view.iter_required_component::<Scalar>()?,
                        arch_view.iter_optional_component::<ScalarScattering>()?,
                        arch_view.iter_optional_component::<Color>()?,
                        arch_view.iter_optional_component::<Radius>()?,
                        arch_view.iter_optional_component::<Text>()?,
                    ) {
                        let color =
                            annotation_info.color(color.map(|c| c.to_array()), default_color);
                        let label = annotation_info.label(label.as_ref().map(|l| l.as_str()));

                        const DEFAULT_RADIUS: f32 = 0.75;

                        points.push(PlotPoint {
                            time: time.as_i64(),
                            value: scalar.0,
                            attrs: PlotPointAttrs {
                                label,
                                color,
                                radius: radius.map_or(DEFAULT_RADIUS, |r| r.0),
                                scattered: scattered.map_or(false, |s| s.0),
                            },
                        });
                    }
                }
                eprintln!("joins in {:?}", now.elapsed());
            }

            {
                let now = std::time::Instant::now();
                re_tracing::profile_scope!("sort");
                points.sort_by_key(|s| s.time);
                eprintln!("sort in {:?}", now.elapsed());
            }

            if points.is_empty() {
                continue;
            }

            // If all points within a line share the label (and it isn't `None`), then we use it
            // as the whole line label for the plot legend.
            // Otherwise, we just use the entity path as-is.
            let same_label = |points: &[PlotPoint]| -> Option<String> {
                let label = points[0].attrs.label.as_ref()?;
                (points.iter().all(|p| p.attrs.label.as_ref() == Some(label)))
                    .then(|| label.clone())
            };
            let line_label = same_label(&points).unwrap_or_else(|| ent_path.to_string());

            self.add_line_segments(&line_label, points);
        }

        Ok(())
    }

    // We have a bunch of raw points, and now we need to group them into actual line
    // segments.
    // A line segment is a continuous run of points with identical attributes: each time
    // we notice a change in attributes, we need a new line segment.
    #[inline(never)] // Better callstacks on crashes
    fn add_line_segments(&mut self, line_label: &str, points: Vec<PlotPoint>) {
        re_tracing::profile_function!();

        let num_points = points.len();
        let mut attrs = points[0].attrs.clone();
        let mut line: PlotSeries = PlotSeries {
            label: line_label.to_owned(),
            color: attrs.color,
            width: 2.0 * attrs.radius,
            kind: if attrs.scattered {
                PlotSeriesKind::Scatter
            } else {
                PlotSeriesKind::Continuous
            },
            points: Vec::with_capacity(num_points),
        };

        for (i, p) in points.into_iter().enumerate() {
            if p.attrs == attrs {
                // Same attributes, just add to the current line segment.

                line.points.push((p.time, p.value));
            } else {
                // Attributes changed since last point, break up the current run into a
                // line segment, and start the next one.

                attrs = p.attrs.clone();
                let kind = if attrs.scattered {
                    PlotSeriesKind::Scatter
                } else {
                    PlotSeriesKind::Continuous
                };

                let prev_line = std::mem::replace(
                    &mut line,
                    PlotSeries {
                        label: line_label.to_owned(),
                        color: attrs.color,
                        width: 2.0 * attrs.radius,
                        kind,
                        points: Vec::with_capacity(num_points - i),
                    },
                );
                let prev_point = *prev_line.points.last().unwrap();
                self.lines.push(prev_line);

                // If the previous point was continuous and the current point is continuous
                // too, then we want the 2 segments to appear continuous even though they
                // are actually split from a data standpoint.
                let cur_continuous = matches!(kind, PlotSeriesKind::Continuous);
                let prev_continuous = matches!(kind, PlotSeriesKind::Continuous);
                if cur_continuous && prev_continuous {
                    line.points.push(prev_point);
                }

                // Add the point that triggered the split to the new segment.
                line.points.push((p.time, p.value));
            }
        }

        if !line.points.is_empty() {
            self.lines.push(line);
        }
    }
}
