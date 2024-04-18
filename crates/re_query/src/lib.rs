//! High-level, cached query API for `re_data_store`.

mod cache;
mod cache_stats;
mod flat_vec_deque;
mod latest_at;
mod promise;
mod range;
mod visible_history;

pub mod clamped_zip;
pub mod range_zip;

pub use self::clamped_zip::*;
pub use self::promise::{Promise, PromiseId, PromiseResolver, PromiseResult};
pub use self::range_zip::*;
pub use self::visible_history::{ExtraQueryHistory, VisibleHistory, VisibleHistoryBoundary};

pub use self::cache::{CacheKey, Caches};
pub use self::cache_stats::{CachedComponentStats, CachesStats};
pub use self::flat_vec_deque::{ErasedFlatVecDeque, FlatVecDeque};
pub use self::latest_at::{
    LatestAtComponentResults, CachedLatestAtMonoResult, LatestAtResults,
};
pub use self::range::{RangeComponentResults, CachedRangeData, RangeResults};

pub(crate) use self::latest_at::LatestAtCache;
pub(crate) use self::range::{RangeComponentResultsInner, RangeCache};

pub mod external {
    pub use paste;
    pub use seq_macro;
}

// ---

#[derive(Debug, Clone, Copy)]
pub struct ComponentNotFoundError(pub re_types_core::ComponentName);

impl std::fmt::Display for ComponentNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Could not find component: {}", self.0))
    }
}

impl std::error::Error for ComponentNotFoundError {}

#[derive(thiserror::Error, Debug)]
pub enum QueryError {
    #[error("Tried to access a column that doesn't exist")]
    BadAccess,

    #[error("Could not find primary component: {0}")]
    PrimaryNotFound(re_types_core::ComponentName),

    #[error(transparent)]
    ComponentNotFound(#[from] ComponentNotFoundError),

    #[error("Tried to access component of type '{actual:?}' using component '{requested:?}'")]
    TypeMismatch {
        actual: re_types_core::ComponentName,
        requested: re_types_core::ComponentName,
    },

    #[error("Error with one or more the underlying data cells: {0}")]
    DataCell(#[from] re_log_types::DataCellError),

    #[error("Error deserializing: {0}")]
    DeserializationError(#[from] re_types_core::DeserializationError),

    #[error("Error serializing: {0}")]
    SerializationError(#[from] re_types_core::SerializationError),

    #[error("Error converting arrow data: {0}")]
    ArrowError(#[from] arrow2::error::Error),

    #[error("Not implemented")]
    NotImplemented,

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, QueryError>;

// ---

/// Helper extension trait to convert query results into [`re_types_core::Archetype`]s.
pub trait ToArchetype<A: re_types_core::Archetype> {
    /// Converts the result into an [`re_types_core::Archetype`].
    ///
    /// Automatically handles all aspects of the query process: deserialization, caching, promise
    /// resolution, etc.
    fn to_archetype(
        &self,
        resolver: &crate::PromiseResolver,
    ) -> crate::PromiseResult<crate::Result<A>>;
}

// ---

use re_data_store::{LatestAtQuery, RangeQuery};

#[derive(Debug)]
pub enum Results {
    LatestAt(LatestAtQuery, LatestAtResults),
    Range(RangeQuery, RangeResults),
}

impl From<(LatestAtQuery, LatestAtResults)> for Results {
    #[inline]
    fn from((query, results): (LatestAtQuery, LatestAtResults)) -> Self {
        Self::LatestAt(query, results)
    }
}

impl From<(RangeQuery, RangeResults)> for Results {
    #[inline]
    fn from((query, results): (RangeQuery, RangeResults)) -> Self {
        Self::Range(query, results)
    }
}

// ---

/// Returns `true` if the specified `component_name` can be cached.
///
/// Used internally to avoid unnecessarily caching components that are already cached in other
/// places, for historical reasons.
pub fn cacheable(component_name: re_types::ComponentName) -> bool {
    use std::sync::OnceLock;
    static NOT_CACHEABLE: OnceLock<re_types::ComponentNameSet> = OnceLock::new();

    use re_types_core::Loggable as _;
    let not_cacheable = NOT_CACHEABLE.get_or_init(|| {
        [
            // TODO(#5303): instance keys are on their way out anyhow.
            re_types::components::InstanceKey::name(),
            // TODO(#5974): tensors might already be cached in the ad-hoc JPEG cache, we don't
            // want yet another copy.
            re_types::components::TensorData::name(),
            // TODO(#5974): meshes are already cached in the ad-hoc mesh cache, we don't
            // want yet another copy.
            re_types::components::MeshProperties::name(),
            // TODO(#5974): blobs are used for assets, which are themselves already cached in
            // the ad-hoc mesh cache -- we don't want yet another copy.
            re_types::components::Blob::name(),
        ]
        .into()
    });

    !not_cacheable.contains(&component_name)
}
