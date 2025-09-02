//! Internal foundational model types.
//!
//! Defines core building blocks: dimensional type machinery (`dimension`),
//! generic runtime holder of a value with a unit phantom (`measure`), user
//! facing dimensioned value abstraction (`quantity`), unit trait & macros
//! (`unit`), and prefix infrastructure (`prefix`). Re-exported selectively
//! through the crate root; kept private to prevent external trait impls.

pub(crate) mod dimension;
pub(crate) mod measure;
pub(crate) mod prefix;
pub(crate) mod quantity;
pub(crate) mod unit;

/// Public trait in private module to prevent external implementations
mod sealed {
    pub trait Sealed {}
}
