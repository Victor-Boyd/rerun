// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/re_types/definitions/rerun/datatypes/tensor_dimension.fbs".

#![allow(trivial_numeric_casts)]
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

/// **Datatype**: A single dimension within a multi-dimensional tensor.
#[derive(Clone, Default, Eq, PartialEq)]
pub struct TensorDimension {
    /// The length of this dimension.
    pub size: u64,

    /// The name of this dimension, e.g. "width", "height", "channel", "batch', ….
    pub name: Option<crate::ArrowString>,
}

impl<'a> From<TensorDimension> for ::std::borrow::Cow<'a, TensorDimension> {
    #[inline]
    fn from(value: TensorDimension) -> Self {
        std::borrow::Cow::Owned(value)
    }
}

impl<'a> From<&'a TensorDimension> for ::std::borrow::Cow<'a, TensorDimension> {
    #[inline]
    fn from(value: &'a TensorDimension) -> Self {
        std::borrow::Cow::Borrowed(value)
    }
}

impl crate::Loggable for TensorDimension {
    type Name = crate::DatatypeName;

    #[inline]
    fn name() -> Self::Name {
        "rerun.datatypes.TensorDimension".into()
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    #[inline]
    fn arrow_datatype() -> arrow2::datatypes::DataType {
        use ::arrow2::datatypes::*;
        DataType::Struct(vec![
            Field {
                name: "size".to_owned(),
                data_type: DataType::UInt64,
                is_nullable: false,
                metadata: [].into(),
            },
            Field {
                name: "name".to_owned(),
                data_type: DataType::Utf8,
                is_nullable: true,
                metadata: [].into(),
            },
        ])
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
    ) -> crate::SerializationResult<Box<dyn ::arrow2::array::Array>>
    where
        Self: Clone + 'a,
    {
        re_tracing::profile_function!();
        use crate::{Loggable as _, ResultExt as _};
        use ::arrow2::{array::*, datatypes::*};
        Ok({
            let (somes, data): (Vec<_>, Vec<_>) = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    (datum.is_some(), datum)
                })
                .unzip();
            let bitmap: Option<::arrow2::bitmap::Bitmap> = {
                let any_nones = somes.iter().any(|some| !*some);
                any_nones.then(|| somes.into())
            };
            StructArray::new(
                <crate::datatypes::TensorDimension>::arrow_datatype(),
                vec![
                    {
                        let (somes, size): (Vec<_>, Vec<_>) = data
                            .iter()
                            .map(|datum| {
                                let datum = datum.as_ref().map(|datum| {
                                    let Self { size, .. } = &**datum;
                                    size.clone()
                                });
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let size_bitmap: Option<::arrow2::bitmap::Bitmap> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        PrimitiveArray::new(
                            DataType::UInt64,
                            size.into_iter().map(|v| v.unwrap_or_default()).collect(),
                            size_bitmap,
                        )
                        .boxed()
                    },
                    {
                        let (somes, name): (Vec<_>, Vec<_>) = data
                            .iter()
                            .map(|datum| {
                                let datum = datum
                                    .as_ref()
                                    .map(|datum| {
                                        let Self { name, .. } = &**datum;
                                        name.clone()
                                    })
                                    .flatten();
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let name_bitmap: Option<::arrow2::bitmap::Bitmap> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        {
                            let inner_data: ::arrow2::buffer::Buffer<u8> =
                                name.iter().flatten().flat_map(|s| s.0.clone()).collect();
                            let offsets = ::arrow2::offset::Offsets::<i32>::try_from_lengths(
                                name.iter().map(|opt| {
                                    opt.as_ref().map(|datum| datum.0.len()).unwrap_or_default()
                                }),
                            )
                            .unwrap()
                            .into();
                            #[allow(unsafe_code, clippy::undocumented_unsafe_blocks)]
                            unsafe {
                                Utf8Array::<i32>::new_unchecked(
                                    DataType::Utf8,
                                    offsets,
                                    inner_data,
                                    name_bitmap,
                                )
                            }
                            .boxed()
                        }
                    },
                ],
                bitmap,
            )
            .boxed()
        })
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    fn from_arrow_opt(
        arrow_data: &dyn ::arrow2::array::Array,
    ) -> crate::DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        re_tracing::profile_function!();
        use crate::{Loggable as _, ResultExt as _};
        use ::arrow2::{array::*, buffer::*, datatypes::*};
        Ok({
            let arrow_data = arrow_data
                .as_any()
                .downcast_ref::<::arrow2::array::StructArray>()
                .ok_or_else(|| {
                    crate::DeserializationError::datatype_mismatch(
                        DataType::Struct(vec![
                            Field {
                                name: "size".to_owned(),
                                data_type: DataType::UInt64,
                                is_nullable: false,
                                metadata: [].into(),
                            },
                            Field {
                                name: "name".to_owned(),
                                data_type: DataType::Utf8,
                                is_nullable: true,
                                metadata: [].into(),
                            },
                        ]),
                        arrow_data.data_type().clone(),
                    )
                })
                .with_context("rerun.datatypes.TensorDimension")?;
            if arrow_data.is_empty() {
                Vec::new()
            } else {
                let (arrow_data_fields, arrow_data_arrays) =
                    (arrow_data.fields(), arrow_data.values());
                let arrays_by_name: ::std::collections::HashMap<_, _> = arrow_data_fields
                    .iter()
                    .map(|field| field.name.as_str())
                    .zip(arrow_data_arrays)
                    .collect();
                let size = {
                    if !arrays_by_name.contains_key("size") {
                        return Err(crate::DeserializationError::missing_struct_field(
                            Self::arrow_datatype(),
                            "size",
                        ))
                        .with_context("rerun.datatypes.TensorDimension");
                    }
                    let arrow_data = &**arrays_by_name["size"];
                    arrow_data
                        .as_any()
                        .downcast_ref::<UInt64Array>()
                        .ok_or_else(|| {
                            crate::DeserializationError::datatype_mismatch(
                                DataType::UInt64,
                                arrow_data.data_type().clone(),
                            )
                        })
                        .with_context("rerun.datatypes.TensorDimension#size")?
                        .into_iter()
                        .map(|opt| opt.copied())
                };
                let name = {
                    if !arrays_by_name.contains_key("name") {
                        return Err(crate::DeserializationError::missing_struct_field(
                            Self::arrow_datatype(),
                            "name",
                        ))
                        .with_context("rerun.datatypes.TensorDimension");
                    }
                    let arrow_data = &**arrays_by_name["name"];
                    {
                        let arrow_data = arrow_data
                            .as_any()
                            .downcast_ref::<::arrow2::array::Utf8Array<i32>>()
                            .ok_or_else(|| {
                                crate::DeserializationError::datatype_mismatch(
                                    DataType::Utf8,
                                    arrow_data.data_type().clone(),
                                )
                            })
                            .with_context("rerun.datatypes.TensorDimension#name")?;
                        let arrow_data_buf = arrow_data.values();
                        let offsets = arrow_data.offsets();
                        arrow2::bitmap::utils::ZipValidity::new_with_validity(
                            offsets.iter().zip(offsets.lengths()),
                            arrow_data.validity(),
                        )
                        .map(|elem| {
                            elem.map(|(start, len)| {
                                let start = *start as usize;
                                let end = start + len;
                                if end as usize > arrow_data_buf.len() {
                                    return Err(crate::DeserializationError::offset_slice_oob(
                                        (start, end),
                                        arrow_data_buf.len(),
                                    ));
                                }

                                #[allow(unsafe_code, clippy::undocumented_unsafe_blocks)]
                                let data =
                                    unsafe { arrow_data_buf.clone().sliced_unchecked(start, len) };
                                Ok(data)
                            })
                            .transpose()
                        })
                        .map(|res_or_opt| {
                            res_or_opt.map(|res_or_opt| res_or_opt.map(|v| crate::ArrowString(v)))
                        })
                        .collect::<crate::DeserializationResult<Vec<Option<_>>>>()
                        .with_context("rerun.datatypes.TensorDimension#name")?
                        .into_iter()
                    }
                };
                arrow2::bitmap::utils::ZipValidity::new_with_validity(
                    ::itertools::izip!(size, name),
                    arrow_data.validity(),
                )
                .map(|opt| {
                    opt.map(|(size, name)| {
                        Ok(Self {
                            size: size
                                .ok_or_else(crate::DeserializationError::missing_data)
                                .with_context("rerun.datatypes.TensorDimension#size")?,
                            name,
                        })
                    })
                    .transpose()
                })
                .collect::<crate::DeserializationResult<Vec<_>>>()
                .with_context("rerun.datatypes.TensorDimension")?
            }
        })
    }
}
