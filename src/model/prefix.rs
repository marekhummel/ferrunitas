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

/// Macro to define a prefix with its factor, symbol, and name
#[macro_export]
macro_rules! prefix {
    ($prefix_name:ident, $factor:expr, $symbol:expr) => {
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
        pub struct $prefix_name;

        impl $crate::sealed::Sealed for $prefix_name {}

        impl $crate::model::prefix::Prefix for $prefix_name {
            const FACTOR: f64 = $factor;
            const SYMBOL: &'static str = $symbol;
        }
    };
}
