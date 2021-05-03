pub use crate::core::process::{Postproc, PostprocBuilder, PostprocCollector};
use crate::filter::{Filter, IdentityFilter};

/// Creates postprocessor with already included device source from `clay` and `clay-core`.
pub fn create_postproc<F: Filter>() -> PostprocCollector<F> {
    let mut collector = crate::core::process::create_postproc::<F>();
    collector.add_hook(crate::source());
    collector
}

/// Creates postprocessor with identity filter.
pub fn create_default_postproc() -> PostprocCollector<IdentityFilter> {
    create_postproc::<IdentityFilter>()
}
