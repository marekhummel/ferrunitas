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
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct PrefixedUnit<P: Prefix, U>(
    pub f64,
    std::marker::PhantomData<P>,
    std::marker::PhantomData<U>,
);

impl<P: Prefix, U> PrefixedUnit<P, U> {
    /// Create a new prefixed unit with the given value
    pub fn new(value: f64) -> Self {
        Self(value, std::marker::PhantomData, std::marker::PhantomData)
    }

    /// Get the value of the prefixed unit
    pub fn value(&self) -> f64 {
        self.0
    }
}

// ============================================================================
// Macro to define SI prefixes
// ============================================================================

/// Macro to define a prefix with its factor, symbol, and name
#[macro_export]
macro_rules! define_prefix {
    ($prefix_name:ident, $factor:expr, $symbol:expr, $name:expr) => {
        pub struct $prefix_name;
        impl $crate::model::prefix::Prefix for $prefix_name {
            const FACTOR: f64 = $factor;
            const SYMBOL: &'static str = $symbol;
            const NAME: &'static str = $name;
        }
    };
}

// ============================================================================
// Macro for creating prefixed unit type aliases with conversions
// ============================================================================

/// Macro to create a type alias for a prefixed unit (conversions implemented generically)
#[macro_export]
macro_rules! prefixed_unit {
    ($alias:ident, $prefix:ty, $base_unit:ty) => {
        pub type $alias = $crate::model::prefix::PrefixedUnit<$prefix, $base_unit>;

        // Allow construction with function call syntax like Kilogram(5.0)
        pub fn $alias(value: f64) -> $alias {
            <$alias>::new(value)
        }
    };
}

// Implement conversions for specific quantity types
macro_rules! impl_prefix_conversions {
    ($quantity_type:ty) => {
        impl<P: $crate::model::prefix::Prefix, U> From<$crate::model::prefix::PrefixedUnit<P, U>>
            for $quantity_type
        {
            fn from(unit: $crate::model::prefix::PrefixedUnit<P, U>) -> Self {
                <$quantity_type>::new(unit.0 * P::FACTOR)
            }
        }

        impl<P: $crate::model::prefix::Prefix, U> $crate::model::unit::FromQuantity<$quantity_type>
            for $crate::model::prefix::PrefixedUnit<P, U>
        {
            fn from_quantity(quantity: $quantity_type) -> Self {
                Self::new(quantity.value() / P::FACTOR)
            }
        }
    };
}

// Implement conversions for all quantity types
impl_prefix_conversions!(crate::system::defs::Mass);
impl_prefix_conversions!(crate::system::defs::Length);
impl_prefix_conversions!(crate::system::defs::Time);
