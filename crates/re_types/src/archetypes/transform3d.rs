// NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.

#![allow(trivial_numeric_casts)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::iter_on_single_items)]
#![allow(clippy::map_flatten)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unnecessary_cast)]

/// A 3D transform.
///
/// ## Example
///
/// ```ignore
/// //! Log some transforms.
///
/// use rerun::{
///    experimental::{
///        archetypes::Transform3D,
///        datatypes::{
///            Angle, Mat3x3, RotationAxisAngle, Scale3D, TranslationAndMat3x3,
///            TranslationRotationScale3D,
///        },
///    },
///    MsgSender, RecordingStreamBuilder,
/// };
/// use std::f32::consts::PI;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///    let (rec_stream, storage) = RecordingStreamBuilder::new("transform").memory()?;
///
///    let arrow = rerun::components::Arrow3D {
///        origin: rerun::components::Vec3D::from([0.0, 0.0, 0.0]),
///        vector: rerun::components::Vec3D::from([0.0, 1.0, 0.0]),
///    };
///
///    MsgSender::new("base")
///        .with_component(&[arrow])?
///        .send(&rec_stream)?;
///
///    MsgSender::from_archetype(
///        "base/translated",
///        &Transform3D::new(TranslationAndMat3x3::new([1.0, 0.0, 0.0], Mat3x3::IDENTITY)),
///    )?
///    .send(&rec_stream)?;
///
///    MsgSender::new("base/translated")
///        .with_component(&[arrow])?
///        .send(&rec_stream)?;
///
///    MsgSender::from_archetype(
///        "base/rotated_scaled",
///        &Transform3D::new(TranslationRotationScale3D {
///            rotation: Some(RotationAxisAngle::new([0.0, 0.0, 1.0], Angle::Radians(PI / 4.)).into()),
///            scale: Some(Scale3D::from(2.0)),
///            ..Default::default()
///        }),
///    )?
///    .send(&rec_stream)?;
///
///    MsgSender::new("base/rotated_scaled")
///        .with_component(&[arrow])?
///        .send(&rec_stream)?;
///
///    rerun::native_viewer::show(storage.take())?;
///    Ok(())
/// }
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Transform3D {
    /// The transform
    pub transform: crate::components::Transform3D,
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[crate::ComponentName; 1usize]> =
    once_cell::sync::Lazy::new(|| ["rerun.components.Transform3D".into()]);
static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[crate::ComponentName; 0usize]> =
    once_cell::sync::Lazy::new(|| []);
static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[crate::ComponentName; 0usize]> =
    once_cell::sync::Lazy::new(|| []);
static ALL_COMPONENTS: once_cell::sync::Lazy<[crate::ComponentName; 1usize]> =
    once_cell::sync::Lazy::new(|| ["rerun.components.Transform3D".into()]);

impl Transform3D {
    pub const NUM_COMPONENTS: usize = 1usize;
}

impl crate::Archetype for Transform3D {
    #[inline]
    fn name() -> crate::ArchetypeName {
        crate::ArchetypeName::Borrowed("rerun.archetypes.Transform3D")
    }

    #[inline]
    fn required_components() -> &'static [crate::ComponentName] {
        REQUIRED_COMPONENTS.as_slice()
    }

    #[inline]
    fn recommended_components() -> &'static [crate::ComponentName] {
        RECOMMENDED_COMPONENTS.as_slice()
    }

    #[inline]
    fn optional_components() -> &'static [crate::ComponentName] {
        OPTIONAL_COMPONENTS.as_slice()
    }

    #[inline]
    fn all_components() -> &'static [crate::ComponentName] {
        ALL_COMPONENTS.as_slice()
    }

    #[inline]
    fn try_to_arrow(
        &self,
    ) -> crate::SerializationResult<
        Vec<(::arrow2::datatypes::Field, Box<dyn ::arrow2::array::Array>)>,
    > {
        use crate::Loggable as _;
        Ok([{
            Some({
                let array = <crate::components::Transform3D>::try_to_arrow([&self.transform], None);
                array.map(|array| {
                    let datatype = ::arrow2::datatypes::DataType::Extension(
                        "rerun.components.Transform3D".into(),
                        Box::new(array.data_type().clone()),
                        Some("rerun.components.Transform3D".into()),
                    );
                    (
                        ::arrow2::datatypes::Field::new("transform", datatype, false),
                        array,
                    )
                })
            })
            .transpose()
            .map_err(|err| crate::SerializationError::Context {
                location: "rerun.archetypes.Transform3D#transform".into(),
                source: Box::new(err),
            })?
        }]
        .into_iter()
        .flatten()
        .collect())
    }

    #[inline]
    fn try_from_arrow(
        data: impl IntoIterator<Item = (::arrow2::datatypes::Field, Box<dyn ::arrow2::array::Array>)>,
    ) -> crate::DeserializationResult<Self> {
        use crate::Loggable as _;
        let arrays_by_name: ::std::collections::HashMap<_, _> = data
            .into_iter()
            .map(|(field, array)| (field.name, array))
            .collect();
        let transform = {
            let array = arrays_by_name
                .get("transform")
                .ok_or_else(|| crate::DeserializationError::MissingData {
                    backtrace: ::backtrace::Backtrace::new_unresolved(),
                })
                .map_err(|err| crate::DeserializationError::Context {
                    location: "rerun.archetypes.Transform3D#transform".into(),
                    source: Box::new(err),
                })?;
            <crate::components::Transform3D>::try_from_arrow_opt(&**array)
                .map_err(|err| crate::DeserializationError::Context {
                    location: "rerun.archetypes.Transform3D#transform".into(),
                    source: Box::new(err),
                })?
                .into_iter()
                .next()
                .flatten()
                .ok_or_else(|| crate::DeserializationError::MissingData {
                    backtrace: ::backtrace::Backtrace::new_unresolved(),
                })
                .map_err(|err| crate::DeserializationError::Context {
                    location: "rerun.archetypes.Transform3D#transform".into(),
                    source: Box::new(err),
                })?
        };
        Ok(Self { transform })
    }
}

impl Transform3D {
    pub fn new(transform: impl Into<crate::components::Transform3D>) -> Self {
        Self {
            transform: transform.into(),
        }
    }
}
