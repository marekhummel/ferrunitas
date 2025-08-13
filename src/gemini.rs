use std::fmt::Display;

// Core traits (Quantity and Unit)
pub trait Quantity {
    type BaseUnit: Unit<Quantity = Self>;
}

// A trait for units to define their conversion to the quantity's canonical base.
pub trait ConvertibleToQuantityBaseUnit {
    const FACTOR_TO_QUANTITY_BASE: f64;
}

pub trait Unit: Copy + Clone + Default + Display {
    type Quantity: Quantity;
    type Base: Unit<Quantity = Self::Quantity>;
    const FACTOR_TO_UNIT_BASE: f64;
}

pub trait Prefix: Copy + Clone + Default + Display {
    const FACTOR: f64;
}

// Prefixed Unit
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct PrefixedUnit<P, U>(std::marker::PhantomData<(P, U)>);

impl<P: Prefix, U: Unit> Unit for PrefixedUnit<P, U> {
    type Quantity = U::Quantity;
    type Base = U;
    const FACTOR_TO_UNIT_BASE: f64 = P::FACTOR;
}

impl<P: Prefix, U: Unit> Default for PrefixedUnit<P, U> {
    fn default() -> Self {
        PrefixedUnit(std::marker::PhantomData)
    }
}

impl<P: Prefix, U: Unit> Display for PrefixedUnit<P, U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", P::default(), U::default())
    }
}

// Value
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Value<V, U> {
    pub value: V,
    _unit: std::marker::PhantomData<U>,
}

impl<V, U> Value<V, U> {
    // A constructor for creating a new Value.
    pub const fn new(value: V) -> Self {
        Value {
            value,
            _unit: std::marker::PhantomData,
        }
    }
}

impl<V, U> Value<V, U>
where
    V: Copy + Into<f64> + From<f64>,
    U: Unit,
{
    pub fn convert<TargetU: Unit>(self) -> Value<V, TargetU>
    where
        // Ensures both units belong to the same physical quantity.
        U::Quantity: PartialEq<TargetU::Quantity>,
        // Requires the raw base units to be convertible to the quantity's base.
        U::Base: ConvertibleToQuantityBaseUnit,
        TargetU::Base: ConvertibleToQuantityBaseUnit,
    {
        // 1. Convert source value to its raw base (e.g., Millistone -> Stone).
        let value_in_raw_base = self.value.into() * U::FACTOR_TO_UNIT_BASE;

        // 2. Convert raw base to quantity's canonical base (e.g., Stone -> Kilogram).
        let value_in_quantity_base =
            value_in_raw_base * <U::Base as ConvertibleToQuantityBaseUnit>::FACTOR_TO_QUANTITY_BASE;

        // 3. Convert from the quantity's canonical base to the target's raw base (e.g., Kilogram -> Pound).
        let value_to_target_base = value_in_quantity_base
            / <TargetU::Base as ConvertibleToQuantityBaseUnit>::FACTOR_TO_QUANTITY_BASE;

        // 4. Convert the target's raw base to the final target unit (e.g., Pound -> Megapound).
        let final_value = value_to_target_base / TargetU::FACTOR_TO_UNIT_BASE;

        Value::new(V::from(final_value))
    }
}

impl<V, U> Display for Value<V, U>
where
    V: Display,
    U: Display + Default,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.value, U::default())
    }
}

// Macros
#[macro_export]
macro_rules! prefix {
    ($name:ident, $factor:literal, $symbol:literal) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub struct $name;

        impl $crate::gemini::Prefix for $name {
            const FACTOR: f64 = $factor;
        }

        impl Default for $name {
            fn default() -> Self {
                $name
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, $symbol)
            }
        }
    };
}

#[macro_export]
macro_rules! quantity {
    // Pattern for when the base unit of the quantity is prefixed (kg for mass)
    ($qty_name:ident, ($base_unit:ident, $prefix_name:ident, $raw_unit_name:ident), $base_unit_symbol:literal) => {
        use $crate::gemini::Prefix;

        // Declare the structs for the quantity and its base unit.
        // These need to exist before we can refer to them in the implementations.
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub struct $qty_name;

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub struct $raw_unit_name;

        // Define the type alias for the prefixed base unit.
        pub type $base_unit = $crate::gemini::PrefixedUnit<$prefix_name, $raw_unit_name>;

        // Implement the Quantity trait for the quantity.
        impl $crate::gemini::Quantity for $qty_name {
            type BaseUnit = $base_unit;
        }

        // Implement the Unit trait for the raw base unit.
        impl $crate::gemini::Unit for $raw_unit_name {
            type Quantity = $qty_name;
            type Base = $base_unit;

            // The conversion factor from this unit to the base_unit is the prefix factor
            const FACTOR_TO_UNIT_BASE: f64 = $prefix_name::FACTOR;
        }

        // Implement ConvertibleToQuantityBaseUnit  trait for the raw base unit.
        impl $crate::gemini::ConvertibleToQuantityBaseUnit for $raw_unit_name {
            // Conversion factor to the quantity's base unit is the inverse of the factor to the raw base.
            const FACTOR_TO_QUANTITY_BASE: f64 = 1.0 / $base_unit::FACTOR_TO_UNIT_BASE;
        }

        impl Default for $raw_unit_name {
            fn default() -> Self {
                $raw_unit_name
            }
        }

        impl std::fmt::Display for $raw_unit_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, $base_unit_symbol)
            }
        }
    };

    // Pattern for when the base unit of the quantity is not prefixed (e.g., s for time)
    ($qty_name:ident, $base_unit:ident, $base_unit_symbol:literal) => {
        // Declare the structs for the quantity and its base unit.
        // These need to exist before we can refer to them in the implementations.
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub struct $qty_name;

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub struct $base_unit;

        // Implement the Quantity trait for the quantity.
        impl $crate::gemini::Quantity for $qty_name {
            type BaseUnit = $base_unit;
        }

        // Implement the Unit trait for the raw base unit.
        impl $crate::gemini::Unit for $base_unit {
            type Quantity = $qty_name;
            type Base = $base_unit;
            const FACTOR_TO_UNIT_BASE: f64 = 1.0;
        }

        // Implement ConvertibleToQuantityBaseUnit  trait for the raw base unit.
        impl $crate::gemini::ConvertibleToQuantityBaseUnit for $base_unit {
            const FACTOR_TO_QUANTITY_BASE: f64 = 1.0;
        }

        impl Default for $base_unit {
            fn default() -> Self {
                $base_unit
            }
        }

        impl std::fmt::Display for $base_unit {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, $base_unit_symbol)
            }
        }
    };
}

#[macro_export]
macro_rules! unit {
    ($name:ident, $quantity:ty, $conversion:expr, $symbol:literal) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub struct $name;

        // Implement ConvertibleToBase to set the factor to the quantity's base unit.
        impl $crate::gemini::ConvertibleToQuantityBaseUnit for $name {
            const FACTOR_TO_QUANTITY_BASE: f64 = $conversion;
        }

        impl $crate::gemini::Unit for $name {
            type Quantity = $quantity;
            type Base = $name; // <Self::Quantity as $crate::gemini::Quantity>::BaseUnit;
            const FACTOR_TO_UNIT_BASE: f64 = 1.0;
        }

        impl Default for $name {
            fn default() -> Self {
                $name
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, $symbol)
            }
        }
    };
}
