// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/to_archetype.rs

#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::cloned_instead_of_copied)]

use crate::{CachedLatestAtResults, PromiseResolver, PromiseResult};
use re_types_core::{Archetype, Loggable as _};
use std::sync::Arc;

impl crate::ToArchetype<re_types_core::archetypes::Clear> for CachedLatestAtResults {
    #[inline]
    fn to_archetype(
        &self,
        resolver: &PromiseResolver,
    ) -> PromiseResult<crate::Result<re_types_core::archetypes::Clear>> {
        re_tracing::profile_function!(<re_types_core::archetypes::Clear>::name());

        // --- Required ---

        use re_types_core::components::ClearIsRecursive;
        let is_recursive = match self.get_required(<ClearIsRecursive>::name()) {
            Ok(is_recursive) => is_recursive,
            Err(query_err) => return PromiseResult::Ready(Err(query_err)),
        };
        let is_recursive = match is_recursive.to_dense::<ClearIsRecursive>(resolver) {
            PromiseResult::Pending => return PromiseResult::Pending,
            PromiseResult::Error(promise_err) => return PromiseResult::Error(promise_err),
            PromiseResult::Ready(query_res) => match query_res {
                Ok(data) => {
                    let Some(first) = data.first().cloned() else {
                        return PromiseResult::Error(std::sync::Arc::new(
                            re_types_core::DeserializationError::missing_data(),
                        ));
                    };
                    first
                }
                Err(query_err) => return PromiseResult::Ready(Err(query_err)),
            },
        };

        // --- Recommended/Optional ---

        // ---

        let arch = re_types_core::archetypes::Clear { is_recursive };

        PromiseResult::Ready(Ok(arch))
    }
}
