//! SI prefixes for units

/// Trait for SI prefixes
pub trait Prefix {
    /// The multiplier for this prefix
    const FACTOR: f64;

    /// The symbol for this prefix
    const SYMBOL: &'static str;

    /// The name of this prefix
    const NAME: &'static str;
}

/// Implemented by types that can have SI prefixes
pub trait Prefixable {}
