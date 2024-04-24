// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/to_archetype.rs

#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::cloned_instead_of_copied)]

use crate::{CachedLatestAtResults, PromiseResolver, PromiseResult};
use re_types_core::{Archetype, Loggable as _};
use std::sync::Arc;

impl crate::ToArchetype<re_types_blueprint::blueprint::archetypes::ContainerBlueprint>
    for CachedLatestAtResults
{
    #[inline]
    fn to_archetype(
        &self,
        resolver: &PromiseResolver,
    ) -> PromiseResult<crate::Result<re_types_blueprint::blueprint::archetypes::ContainerBlueprint>>
    {
        re_tracing::profile_function!(
            <re_types_blueprint::blueprint::archetypes::ContainerBlueprint>::name()
        );

        // --- Required ---

        use re_types_blueprint::blueprint::components::ContainerKind;
        let container_kind = match self.get_required(<ContainerKind>::name()) {
            Ok(container_kind) => container_kind,
            Err(query_err) => return PromiseResult::Ready(Err(query_err)),
        };
        let container_kind = match container_kind.to_dense::<ContainerKind>(resolver) {
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

        use re_types::components::Name;
        let display_name = if let Some(display_name) = self.get(<Name>::name()) {
            match display_name.to_dense::<Name>(resolver) {
                PromiseResult::Pending => return PromiseResult::Pending,
                PromiseResult::Error(promise_err) => return PromiseResult::Error(promise_err),
                PromiseResult::Ready(query_res) => match query_res {
                    Ok(data) => data.first().cloned(),
                    Err(query_err) => return PromiseResult::Ready(Err(query_err)),
                },
            }
        } else {
            None
        };

        use re_types::blueprint::components::IncludedContent;
        let contents = if let Some(contents) = self.get(<IncludedContent>::name()) {
            match contents.to_dense::<IncludedContent>(resolver) {
                PromiseResult::Pending => return PromiseResult::Pending,
                PromiseResult::Error(promise_err) => return PromiseResult::Error(promise_err),
                PromiseResult::Ready(query_res) => match query_res {
                    Ok(data) => Some(data.to_vec()),
                    Err(query_err) => return PromiseResult::Ready(Err(query_err)),
                },
            }
        } else {
            None
        };

        use re_types::blueprint::components::ColumnShare;
        let col_shares = if let Some(col_shares) = self.get(<ColumnShare>::name()) {
            match col_shares.to_dense::<ColumnShare>(resolver) {
                PromiseResult::Pending => return PromiseResult::Pending,
                PromiseResult::Error(promise_err) => return PromiseResult::Error(promise_err),
                PromiseResult::Ready(query_res) => match query_res {
                    Ok(data) => Some(data.to_vec()),
                    Err(query_err) => return PromiseResult::Ready(Err(query_err)),
                },
            }
        } else {
            None
        };

        use re_types::blueprint::components::RowShare;
        let row_shares = if let Some(row_shares) = self.get(<RowShare>::name()) {
            match row_shares.to_dense::<RowShare>(resolver) {
                PromiseResult::Pending => return PromiseResult::Pending,
                PromiseResult::Error(promise_err) => return PromiseResult::Error(promise_err),
                PromiseResult::Ready(query_res) => match query_res {
                    Ok(data) => Some(data.to_vec()),
                    Err(query_err) => return PromiseResult::Ready(Err(query_err)),
                },
            }
        } else {
            None
        };

        use re_types::blueprint::components::ActiveTab;
        let active_tab = if let Some(active_tab) = self.get(<ActiveTab>::name()) {
            match active_tab.to_dense::<ActiveTab>(resolver) {
                PromiseResult::Pending => return PromiseResult::Pending,
                PromiseResult::Error(promise_err) => return PromiseResult::Error(promise_err),
                PromiseResult::Ready(query_res) => match query_res {
                    Ok(data) => data.first().cloned(),
                    Err(query_err) => return PromiseResult::Ready(Err(query_err)),
                },
            }
        } else {
            None
        };

        use re_types::blueprint::components::Visible;
        let visible = if let Some(visible) = self.get(<Visible>::name()) {
            match visible.to_dense::<Visible>(resolver) {
                PromiseResult::Pending => return PromiseResult::Pending,
                PromiseResult::Error(promise_err) => return PromiseResult::Error(promise_err),
                PromiseResult::Ready(query_res) => match query_res {
                    Ok(data) => data.first().cloned(),
                    Err(query_err) => return PromiseResult::Ready(Err(query_err)),
                },
            }
        } else {
            None
        };

        use re_types_blueprint::blueprint::components::GridColumns;
        let grid_columns = if let Some(grid_columns) = self.get(<GridColumns>::name()) {
            match grid_columns.to_dense::<GridColumns>(resolver) {
                PromiseResult::Pending => return PromiseResult::Pending,
                PromiseResult::Error(promise_err) => return PromiseResult::Error(promise_err),
                PromiseResult::Ready(query_res) => match query_res {
                    Ok(data) => data.first().cloned(),
                    Err(query_err) => return PromiseResult::Ready(Err(query_err)),
                },
            }
        } else {
            None
        };

        // ---

        let arch = re_types_blueprint::blueprint::archetypes::ContainerBlueprint {
            container_kind,
            display_name,
            contents,
            col_shares,
            row_shares,
            active_tab,
            visible,
            grid_columns,
        };

        PromiseResult::Ready(Ok(arch))
    }
}
