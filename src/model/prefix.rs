//! SI prefixes for units

/// Trait for SI prefixes
pub trait Prefix: crate::sealed::Sealed {
    /// The multiplier for this prefix
    const FACTOR: f64;

    /// The symbol for this prefix
    const SYMBOL: &'static str;
}

/// Implemented by types that can have SI prefixes
pub trait Prefixable: crate::sealed::Sealed {}
