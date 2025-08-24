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

/// Macro to define a prefix with its factor, symbol, and name
#[macro_export]
macro_rules! prefix {
    ($prefix_name:ident, $factor:expr, $symbol:expr, $name:expr) => {
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
        pub struct $prefix_name;

        impl $crate::model::prefix::Prefix for $prefix_name {
            const FACTOR: f64 = $factor;
            const SYMBOL: &'static str = $symbol;
            const NAME: &'static str = $name;
        }
    };
}
