use std::fmt::Display;

// Core traits (Quantity and Unit)
pub const DIMENSIONS: usize = 7;
pub type Dimension = i16;
pub type DimensionVector = [Dimension; DIMENSIONS];
pub type EncodedDimensionVector = i64; // cant use unsigned due to overflow problems

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Quantity<const EDV: EncodedDimensionVector>;

// Expose a type's encoded exponents as a single const
pub trait EncodedDimensions {
    const EDV: EncodedDimensionVector;
}
impl<const EDV: EncodedDimensionVector> EncodedDimensions for Quantity<EDV> {
    const EDV: EncodedDimensionVector = EDV;
}

pub trait Quantitiable {
    type BaseUnit: Unit<Quantity = Self>;
}

pub trait Unit: Copy + Clone + Default + Display {
    type Quantity: Quantitiable;
    type Base: Unit<Quantity = Self::Quantity>;
    const FACTOR_TO_QUANTITY_BASE: f64;
    const OFFSET_TO_QUANTITY_BASE: f64;
}

pub trait Prefix: Copy + Clone + Default + Display {
    const FACTOR: f64;
}

// Prefixed Unit
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct PrefixedUnit<P, U>(std::marker::PhantomData<(P, U)>);

impl<P: Prefix, U: Unit> Unit for PrefixedUnit<P, U> {
    type Quantity = U::Quantity;
    type Base = U::Base;
    const FACTOR_TO_QUANTITY_BASE: f64 = P::FACTOR * U::FACTOR_TO_QUANTITY_BASE;
    const OFFSET_TO_QUANTITY_BASE: f64 = P::FACTOR * U::OFFSET_TO_QUANTITY_BASE;
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

// ---------- packing layout ----------
// Each dimension is stored in a lane within those 64 bits, with a bias to allow negative dimensionality.
pub mod enc {
    use super::{Dimension, DimensionVector, EncodedDimensionVector, DIMENSIONS};

    const BITS: Dimension = (EncodedDimensionVector::BITS as usize / DIMENSIONS) as Dimension;
    const MASK: EncodedDimensionVector = (1 << BITS) - 1;
    const BIAS: EncodedDimensionVector = 1 << (BITS - 1);

    #[inline(always)]
    pub const fn pack(e: DimensionVector) -> EncodedDimensionVector {
        let mut edv: EncodedDimensionVector = 0;
        let mut i = 0;
        while i < DIMENSIONS {
            let v = (e[i] as EncodedDimensionVector) + BIAS;
            edv |= (v & MASK) << (i as Dimension * BITS);
            i += 1;
        }
        edv
    }

    #[inline(always)]
    pub const fn unpack(code: EncodedDimensionVector) -> DimensionVector {
        let mut dv = [0; DIMENSIONS];
        let mut i = 0;
        while i < DIMENSIONS {
            let raw = (code >> (i as Dimension * BITS)) & MASK;
            dv[i] = (raw - BIAS) as Dimension;
            i += 1;
        }
        dv
    }

    #[inline(always)]
    pub const fn add(
        a: EncodedDimensionVector,
        b: EncodedDimensionVector,
    ) -> EncodedDimensionVector {
        let ea = unpack(a);
        let eb = unpack(b);
        let mut r = [0; DIMENSIONS];
        let mut i = 0;
        while i < DIMENSIONS {
            r[i] = ea[i] + eb[i];
            i += 1;
        }
        pack(r)
    }

    #[inline(always)]
    pub const fn scale(a: EncodedDimensionVector, k: Dimension) -> EncodedDimensionVector {
        let ea = unpack(a);
        let mut r = [0; DIMENSIONS];
        let mut i = 0;
        while i < DIMENSIONS {
            r[i] = ea[i] * k;
            i += 1;
        }
        pack(r)
    }

    pub const ZERO: EncodedDimensionVector = pack([0, 0, 0, 0, 0, 0, 0]);
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
    {
        let value_in_quantity_base: f64 =
            self.value.into() * U::FACTOR_TO_QUANTITY_BASE + U::OFFSET_TO_QUANTITY_BASE;
        let final_value: f64 = (value_in_quantity_base - TargetU::OFFSET_TO_QUANTITY_BASE)
            / TargetU::FACTOR_TO_QUANTITY_BASE;

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
    ($qty_name:ident, [ encoded_dims: $encoded_dims:expr  ], $base_unit:ident, $base_unit_symbol:literal) => {
        // Declare the structs for the quantity and its base unit.
        // These need to exist before we can refer to them in the implementations.
        // #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        // pub struct $qty_name;
        pub type $qty_name = $crate::gemini::Quantity::<{ $encoded_dims }>;

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub struct $base_unit;

        // Implement the Quantity trait for the quantity.
        impl $crate::gemini::Quantitiable for $qty_name {
            type BaseUnit = $base_unit;
        }

        // Implement the Unit trait for the raw base unit.
        impl $crate::gemini::Unit for $base_unit {
            type Quantity = $qty_name;
            type Base = $base_unit;
            const FACTOR_TO_QUANTITY_BASE: f64 = 1.0;
            const OFFSET_TO_QUANTITY_BASE: f64 = 0.0;
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

    ($qty_name:ident, [ L$l:expr, M$m:expr, T$t:expr, I$i:expr, Th$th:expr, N$n:expr, J$j:expr ], $base_unit:ident, $base_unit_symbol:literal) => {
        quantity!($qty_name, [ encoded_dims: $crate::gemini::enc::pack([$l, $m, $t, $i, $th, $n, $j]) ], $base_unit, $base_unit_symbol);
    };


    ( $qty_name:ident, [ $( ($ty:ty, $pow:expr) ),* $(,)? ], $base_unit:ident, $base_unit_symbol:literal ) => {
        let mut acc = $crate::gemini::enc::ZERO;
        $( acc = $crate::gemini::enc::add(acc,  $crate::gemini::enc::scale(<$ty as $crate::gemini::EncodedDimensions>::EDV, $pow)); )*
        quantity!($qty_name, [ encoded_dims: acc ], $base_unit, $base_unit_symbol);
    };

    // Empty => dimensionless
    ( [] ) => { $crate::gemini::Quantity::<{ $crate::enc::ZERO }> };
}

#[macro_export]
macro_rules! unit {
    // Full pattern with conversion factor, offset and conversion unit
    ($name:ident, $quantity:ty, (factor: $conversion_factor:expr, offset: $conversion_offset:expr, $conversion_unit:ty), $symbol:literal) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub struct $name;

        impl $crate::gemini::Unit for $name {
            type Quantity = $quantity;
            type Base = <Self::Quantity as $crate::gemini::Quantitiable>::BaseUnit;
            const FACTOR_TO_QUANTITY_BASE: f64 = $conversion_factor
                * <$conversion_unit as $crate::gemini::Unit>::FACTOR_TO_QUANTITY_BASE;
            const OFFSET_TO_QUANTITY_BASE: f64 = $conversion_offset
                * <$conversion_unit as $crate::gemini::Unit>::FACTOR_TO_QUANTITY_BASE
                + <$conversion_unit as $crate::gemini::Unit>::OFFSET_TO_QUANTITY_BASE;
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

    // Simplified pattern with conversion factor only, no
    ($name:ident, $quantity:ty, ($conversion_factor:expr, $conversion_unit:ty), $symbol:literal) => {
        unit!(
            $name,
            $quantity,
            (
                factor: $conversion_factor,
                offset: 0.0,
                $conversion_unit
            ),
            $symbol
        );
    };

    // Simplified pattern only with conversion factor, base unit is from the quantity
    ($name:ident, $quantity:ty, $conversion_factor:expr, $symbol:literal) => {
        unit!(
            $name,
            $quantity,
            (
                $conversion_factor,
                <$quantity as $crate::gemini::Quantitiable>::BaseUnit
            ),
            $symbol
        );
    };
}
