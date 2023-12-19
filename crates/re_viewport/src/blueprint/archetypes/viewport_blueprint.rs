// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/re_types/definitions/rerun/blueprint/archetypes/viewport_blueprint.fbs".

#![allow(trivial_numeric_casts)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::iter_on_single_items)]
#![allow(clippy::map_flatten)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unnecessary_cast)]

use ::re_types_core::external::arrow2;
use ::re_types_core::ComponentName;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch, MaybeOwnedComponentBatch};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Archetype**: The top-level description of the Viewport.
#[derive(Clone, Debug, Default)]
pub struct ViewportBlueprint {
    /// All of the space-views that belong to the viewport.
    pub space_views: crate::blueprint::components::IncludedSpaceViews,

    /// The layout of the space-views
    pub layout: Option<crate::blueprint::components::ViewportLayout>,

    /// The layout of the space-views
    pub root_container: Option<crate::blueprint::components::RootContainer>,

    /// Show one tab as maximized?
    pub maximized: Option<crate::blueprint::components::SpaceViewMaximized>,

    /// Whether the viewport layout is determined automatically.
    ///
    /// Set to `false` the first time the user messes around with the viewport blueprint.
    pub auto_layout: Option<crate::blueprint::components::AutoLayout>,

    /// Whether or not space views should be created automatically.
    pub auto_space_views: Option<crate::blueprint::components::AutoSpaceViews>,
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 1usize]> =
    once_cell::sync::Lazy::new(|| ["rerun.blueprint.components.IncludedSpaceViews".into()]);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 1usize]> =
    once_cell::sync::Lazy::new(|| ["rerun.blueprint.components.ViewportBlueprintIndicator".into()]);

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 6usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.blueprint.components.AutoLayout".into(),
            "rerun.blueprint.components.AutoSpaceViews".into(),
            "rerun.blueprint.components.RootContainer".into(),
            "rerun.blueprint.components.SpaceViewMaximized".into(),
            "rerun.blueprint.components.ViewportLayout".into(),
            "rerun.components.InstanceKey".into(),
        ]
    });

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 8usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.blueprint.components.IncludedSpaceViews".into(),
            "rerun.blueprint.components.ViewportBlueprintIndicator".into(),
            "rerun.blueprint.components.AutoLayout".into(),
            "rerun.blueprint.components.AutoSpaceViews".into(),
            "rerun.blueprint.components.RootContainer".into(),
            "rerun.blueprint.components.SpaceViewMaximized".into(),
            "rerun.blueprint.components.ViewportLayout".into(),
            "rerun.components.InstanceKey".into(),
        ]
    });

impl ViewportBlueprint {
    pub const NUM_COMPONENTS: usize = 8usize;
}

/// Indicator component for the [`ViewportBlueprint`] [`::re_types_core::Archetype`]
pub type ViewportBlueprintIndicator = ::re_types_core::GenericIndicatorComponent<ViewportBlueprint>;

impl ::re_types_core::Archetype for ViewportBlueprint {
    type Indicator = ViewportBlueprintIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.blueprint.archetypes.ViewportBlueprint".into()
    }

    #[inline]
    fn indicator() -> MaybeOwnedComponentBatch<'static> {
        static INDICATOR: ViewportBlueprintIndicator = ViewportBlueprintIndicator::DEFAULT;
        MaybeOwnedComponentBatch::Ref(&INDICATOR)
    }

    #[inline]
    fn required_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        REQUIRED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn recommended_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        RECOMMENDED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn optional_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        OPTIONAL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn all_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        ALL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn from_arrow_components(
        arrow_data: impl IntoIterator<Item = (ComponentName, Box<dyn arrow2::array::Array>)>,
    ) -> DeserializationResult<Self> {
        re_tracing::profile_function!();
        use ::re_types_core::{Loggable as _, ResultExt as _};
        let arrays_by_name: ::std::collections::HashMap<_, _> = arrow_data
            .into_iter()
            .map(|(name, array)| (name.full_name(), array))
            .collect();
        let space_views = {
            let array = arrays_by_name
                .get("rerun.blueprint.components.IncludedSpaceViews")
                .ok_or_else(DeserializationError::missing_data)
                .with_context("rerun.blueprint.archetypes.ViewportBlueprint#space_views")?;
            <crate::blueprint::components::IncludedSpaceViews>::from_arrow_opt(&**array)
                .with_context("rerun.blueprint.archetypes.ViewportBlueprint#space_views")?
                .into_iter()
                .next()
                .flatten()
                .ok_or_else(DeserializationError::missing_data)
                .with_context("rerun.blueprint.archetypes.ViewportBlueprint#space_views")?
        };
        let layout =
            if let Some(array) = arrays_by_name.get("rerun.blueprint.components.ViewportLayout") {
                <crate::blueprint::components::ViewportLayout>::from_arrow_opt(&**array)
                    .with_context("rerun.blueprint.archetypes.ViewportBlueprint#layout")?
                    .into_iter()
                    .next()
                    .flatten()
            } else {
                None
            };
        let root_container =
            if let Some(array) = arrays_by_name.get("rerun.blueprint.components.RootContainer") {
                <crate::blueprint::components::RootContainer>::from_arrow_opt(&**array)
                    .with_context("rerun.blueprint.archetypes.ViewportBlueprint#root_container")?
                    .into_iter()
                    .next()
                    .flatten()
            } else {
                None
            };
        let maximized = if let Some(array) =
            arrays_by_name.get("rerun.blueprint.components.SpaceViewMaximized")
        {
            <crate::blueprint::components::SpaceViewMaximized>::from_arrow_opt(&**array)
                .with_context("rerun.blueprint.archetypes.ViewportBlueprint#maximized")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        let auto_layout =
            if let Some(array) = arrays_by_name.get("rerun.blueprint.components.AutoLayout") {
                <crate::blueprint::components::AutoLayout>::from_arrow_opt(&**array)
                    .with_context("rerun.blueprint.archetypes.ViewportBlueprint#auto_layout")?
                    .into_iter()
                    .next()
                    .flatten()
            } else {
                None
            };
        let auto_space_views =
            if let Some(array) = arrays_by_name.get("rerun.blueprint.components.AutoSpaceViews") {
                <crate::blueprint::components::AutoSpaceViews>::from_arrow_opt(&**array)
                    .with_context("rerun.blueprint.archetypes.ViewportBlueprint#auto_space_views")?
                    .into_iter()
                    .next()
                    .flatten()
            } else {
                None
            };
        Ok(Self {
            space_views,
            layout,
            root_container,
            maximized,
            auto_layout,
            auto_space_views,
        })
    }
}

impl ::re_types_core::AsComponents for ViewportBlueprint {
    fn as_component_batches(&self) -> Vec<MaybeOwnedComponentBatch<'_>> {
        re_tracing::profile_function!();
        use ::re_types_core::Archetype as _;
        [
            Some(Self::indicator()),
            Some((&self.space_views as &dyn ComponentBatch).into()),
            self.layout
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
            self.root_container
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
            self.maximized
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
            self.auto_layout
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
            self.auto_space_views
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch).into()),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    #[inline]
    fn num_instances(&self) -> usize {
        1
    }
}

impl ViewportBlueprint {
    pub fn new(space_views: impl Into<crate::blueprint::components::IncludedSpaceViews>) -> Self {
        Self {
            space_views: space_views.into(),
            layout: None,
            root_container: None,
            maximized: None,
            auto_layout: None,
            auto_space_views: None,
        }
    }

    #[inline]
    pub fn with_layout(
        mut self,
        layout: impl Into<crate::blueprint::components::ViewportLayout>,
    ) -> Self {
        self.layout = Some(layout.into());
        self
    }

    #[inline]
    pub fn with_root_container(
        mut self,
        root_container: impl Into<crate::blueprint::components::RootContainer>,
    ) -> Self {
        self.root_container = Some(root_container.into());
        self
    }

    #[inline]
    pub fn with_maximized(
        mut self,
        maximized: impl Into<crate::blueprint::components::SpaceViewMaximized>,
    ) -> Self {
        self.maximized = Some(maximized.into());
        self
    }

    #[inline]
    pub fn with_auto_layout(
        mut self,
        auto_layout: impl Into<crate::blueprint::components::AutoLayout>,
    ) -> Self {
        self.auto_layout = Some(auto_layout.into());
        self
    }

    #[inline]
    pub fn with_auto_space_views(
        mut self,
        auto_space_views: impl Into<crate::blueprint::components::AutoSpaceViews>,
    ) -> Self {
        self.auto_space_views = Some(auto_space_views.into());
        self
    }
}
