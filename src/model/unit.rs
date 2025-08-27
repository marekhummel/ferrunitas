use std::{
    fmt::{Debug, Display},
    ops::{Add, Div, Mul, Sub},
};

use crate::model::{
    prefix::{Prefix, Prefixable},
    quantity::{IntoUnit, QuantityMarker},
};

/// Top-Level definition of a Unit. Note that the arithmetic traits are incomplete,
/// as they do not cover multiplication between units, since the type system can't cover things
/// like "+ for Q Mul<impl Unit<Quantity = Q>, Output=Self::Quantity as Mul<Q>>".
pub trait Unit:
    Debug
    + Display
    + Clone
    + Copy
    + PartialEq
    + PartialOrd
    + Sized
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<f64, Output = Self>
    + Div<f64, Output = Self>
{
    type Quantity: QuantityMarker + IntoUnit;
    const FACTOR: f64;
    const ABBREV: &'static str;

    /// Create a new unit from a raw value
    fn new(value: impl Into<f64>) -> Self;

    /// Return internal value
    fn raw_value(&self) -> f64;

    /// Convert quantity instance into a unit
    fn from_q(q: Self::Quantity) -> Self {
        Self::new(q.raw_value() / Self::FACTOR)
    }

    /// Convert unit instance into a quantity
    fn into_q(self) -> Self::Quantity {
        Self::Quantity::new(self.raw_value() * Self::FACTOR)
    }

    /// Convert unit instance to a quantity (just borrowed)
    fn to_q(&self) -> Self::Quantity {
        (*self).into_q()
    }

    /// Directly convert into another unit of same quantity
    fn convert<U: Unit<Quantity = Self::Quantity>>(self) -> U {
        self.to_q().to_unit::<U>()
    }
}

/// A compound struct for units with prefixes.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct PrefixedUnit<P: Prefix, U: Unit + Prefixable>(
    pub(crate) f64,
    pub(crate) std::marker::PhantomData<(P, U)>,
);
