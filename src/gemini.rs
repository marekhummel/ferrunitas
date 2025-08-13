use std::fmt::Display;

// Core traits (Quantity and Unit)
pub trait Quantity {
    type BaseUnit: Unit<Quantity = Self>;
}

pub trait Unit: Copy + Clone + Default + Display {
    type Quantity: Quantity;
    type Base: Unit<Quantity = Self::Quantity>;
    const CONVERSION_FACTOR_TO_BASE: f64;
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
    const CONVERSION_FACTOR_TO_BASE: f64 = P::FACTOR;
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
        U::Quantity: PartialEq<TargetU::Quantity>,
        // U::Base: PartialEq<<U::Quantity::BaseUnit as Unit>::Base>,
    {
        // // Convert the value to the base unit of its dimension.
        // let value_in_base = self.value.into() * U::CONVERSION_FACTOR_TO_BASE;

        // // Convert the value from the base unit to the target unit.
        // let final_value = value_in_base / TargetU::CONVERSION_FACTOR_TO_BASE;

        // Value::new(V::from(final_value))

        // 1. Convert source value to its raw base (e.g., Millistone -> Stone).
        let value_in_raw_base = self.value.into() * U::FACTOR_TO_RAW_BASE;

        // 2. Convert raw base to quantity's canonical base (e.g., Stone -> Kilogram).
        let value_in_quantity_base =
            value_in_raw_base * <U::Base as ConvertibleToBase>::FACTOR_TO_QUANTITY_BASE;

        // 3. Convert from the quantity's canonical base to the target's raw base (e.g., Kilogram -> Pound).
        let value_to_target_base =
            value_in_quantity_base / <TargetU::Base as ConvertibleToBase>::FACTOR_TO_QUANTITY_BASE;

        // 4. Convert the target's raw base to the final target unit (e.g., Pound -> Megapound).
        let final_value = value_to_target_base / TargetU::FACTOR_TO_RAW_BASE;

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
    ($qty_name:ident, ($full_name:ident, $prefix_name:ident, $base_unit_name:ident), $base_unit_symbol:literal) => {
        // STEP 1
        // Declare the structs for the quantity and its base unit.
        // These need to exist before we can refer to them in the implementations.
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub struct $qty_name;

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub struct $base_unit_name;

        // Define the type alias for the prefixed base unit.
        // For example, `type Kilogram = PrefixedUnit<Kilo, Gram>;`
        pub type $full_name = $crate::gemini::PrefixedUnit<$prefix_name, $base_unit_name>;

        // STEP 2
        // Implement the Quantity trait for the quantity.
        impl $crate::gemini::Quantity for $qty_name {
            type BaseUnit = $full_name;
        }

        // Implement the Unit trait for the raw base unit.
        impl $crate::gemini::Unit for $base_unit_name {
            type Quantity = $qty_name;
            // The `Base` type for the raw unit is itself.
            type Base = $full_name;

            // The conversion factor from this unit to itself is 1.0.
            const CONVERSION_FACTOR_TO_BASE: f64 = 1.0 / $full_name::CONVERSION_FACTOR_TO_BASE;
        }

        impl Default for $base_unit_name {
            fn default() -> Self {
                $base_unit_name
            }
        }

        impl std::fmt::Display for $base_unit_name {
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

        impl $crate::gemini::Unit for $name {
            type Quantity = $quantity;
            // type Base = <<Self::Quantity as $crate::gemini::Quantity>::BaseUnit as $crate::gemini::Unit>::Base;
            type Base = <Self::Quantity as $crate::gemini::Quantity>::BaseUnit;
            const CONVERSION_FACTOR_TO_BASE: f64 = $conversion;
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
