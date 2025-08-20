/// SI prefixes for units
/// This module defines standard SI prefixes and provides a generic PrefixedUnit type

/// Trait for SI prefixes
pub trait Prefix {
    /// The multiplier for this prefix
    const FACTOR: f64;

    /// The symbol for this prefix
    const SYMBOL: &'static str;

    /// The name of this prefix
    const NAME: &'static str;
}

/// Generic prefixed unit that combines a base unit with a prefix

// ============================================================================
// Macro to define SI prefixes
// ============================================================================

/// Macro to define a prefix with its factor, symbol, and name
#[macro_export]
macro_rules! prefix {
    ($prefix_name:ident, $factor:expr, $symbol:expr, $name:expr) => {
        pub struct $prefix_name;
        impl $crate::model::prefix::Prefix for $prefix_name {
            const FACTOR: f64 = $factor;
            const SYMBOL: &'static str = $symbol;
            const NAME: &'static str = $name;
        }
    };
}
