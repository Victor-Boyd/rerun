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

/// A 3D transform
#[derive(Clone, Debug)]
pub struct Transform3D {
    /// The transform
    pub transform: crate::components::Transform3D,
}

impl Transform3D {
    pub const REQUIRED_COMPONENTS: [crate::ComponentName; 1usize] =
        [crate::ComponentName::Borrowed(
            "rerun.components.Transform3D",
        )];

    pub const RECOMMENDED_COMPONENTS: [crate::ComponentName; 0usize] = [];

    pub const OPTIONAL_COMPONENTS: [crate::ComponentName; 0usize] = [];

    pub const ALL_COMPONENTS: [crate::ComponentName; 1usize] = [crate::ComponentName::Borrowed(
        "rerun.components.Transform3D",
    )];

    pub const NUM_COMPONENTS: usize = 1usize;
}

impl crate::Archetype for Transform3D {
    #[inline]
    fn name() -> crate::ArchetypeName {
        crate::ArchetypeName::Borrowed("rerun.archetypes.Transform3D")
    }

    #[inline]
    fn required_components() -> Vec<crate::ComponentName> {
        Self::REQUIRED_COMPONENTS.to_vec()
    }

    #[inline]
    fn recommended_components() -> Vec<crate::ComponentName> {
        Self::RECOMMENDED_COMPONENTS.to_vec()
    }

    #[inline]
    fn optional_components() -> Vec<crate::ComponentName> {
        Self::OPTIONAL_COMPONENTS.to_vec()
    }

    #[inline]
    fn all_components() -> Vec<crate::ComponentName> {
        Self::ALL_COMPONENTS.to_vec()
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
            .transpose()?
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
            let array = arrays_by_name.get("transform").ok_or_else(|| {
                crate::DeserializationError::MissingData {
                    datatype: ::arrow2::datatypes::DataType::Null,
                }
            })?;
            <crate::components::Transform3D>::try_from_arrow_opt(&**array)?
                .into_iter()
                .next()
                .flatten()
                .ok_or_else(|| crate::DeserializationError::MissingData {
                    datatype: ::arrow2::datatypes::DataType::Null,
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
