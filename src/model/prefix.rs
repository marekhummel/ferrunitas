//! SI prefixes for units

use crate::common;

/// Trait for SI prefixes
pub trait Prefix: common::Sealed {
    /// The multiplier for this prefix
    const FACTOR: f64;

    /// The symbol for this prefix
    const SYMBOL: &'static str;
}

/// Implemented by types that can have SI prefixes
pub trait Prefixable: common::Sealed {}
