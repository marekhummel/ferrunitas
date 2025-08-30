use std::{
    fmt::{Debug, Display},
    ops::{Add, Div, Mul, Sub},
};

use crate::{
    common,
    model::{
        prefix::{Prefix, Prefixable},
        quantity::{Dimensioned, QuantityMarker, ToQuantity},
    },
};

pub trait UnitBase:
    Debug + Display + Clone + Copy + PartialEq + PartialOrd + common::Sealed
{
    type InternalQuantity: Dimensioned + QuantityMarker;
    const FACTOR: f64;
    const ABBREV: &'static str;

    /// Create a new unit from a raw value
    fn new(value: impl Into<f64>) -> Self;

    /// Return internal value
    fn raw_value(&self) -> f64;
}

/// Top-Level definition of a Unit. Note that the arithmetic traits are incomplete,
/// as they do not cover multiplication between units, since the type system can't cover things
/// like "+ for Q Mul<impl Unit<Quantity = Q>, Output=Self::Quantity as Mul<Q>>".
pub trait Unit:
    UnitBase<InternalQuantity = Self::Quantity>
    + ToQuantity<BaseQuantity = Self::Quantity>
    + Sized
    + Add<Self, Output = Self>
// + Sub<Self, Output = Self>
// + Mul<f64, Output = Self>
// + Div<f64, Output = Self>
{
    type Quantity: Dimensioned + QuantityMarker;

    /// Create a new unit from a raw value
    fn new(value: impl Into<f64>) -> Self {
        <Self as UnitBase>::new(value)
    }

    /// Return internal value
    fn raw_value(&self) -> f64 {
        <Self as UnitBase>::raw_value(self)
    }

    /// Convert quantity instance into a unit
    fn from_q(q: Self::Quantity) -> Self {
        <Self as ToQuantity>::internal_from_q(q)
    }

    fn into_q(self) -> Self::Quantity {
        <Self as ToQuantity>::internal_into_q(self)
    }

    /// Convert unit instance to a quantity (just borrowed)
    fn to_q(&self) -> Self::Quantity {
        <Self as ToQuantity>::internal_to_q(&self)
    }

    /// Directly convert into another unit of same quantity
    fn convert<U: Unit<Quantity = Self::Quantity>>(self) -> U {
        <Self as ToQuantity>::internal_to_q(&self).as_unit::<U>()
    }

    /// Checks for numerical equality within the quantity, contrary to the default equality check which also requires
    /// type equality.
    fn is_equal_to<U: Unit<Quantity = Self::Quantity>>(&self, other: &U) -> bool {
        <Self as ToQuantity>::internal_to_q(&self) == <U as ToQuantity>::internal_to_q(&other)
    }
}

/// A compound struct for units with prefixes.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct PrefixedUnit<P: Prefix, U: Unit + Prefixable>(
    pub(crate) f64,
    pub(crate) std::marker::PhantomData<(P, U)>,
);

impl<P: Prefix, U: Unit + Prefixable> common::Sealed for PrefixedUnit<P, U> {}

impl<P: Prefix, U: Unit + Prefixable> PrefixedUnit<P, U> {
    pub fn new(value: impl Into<f64>) -> Self {
        Self(value.into(), std::marker::PhantomData)
    }

    pub fn raw_value(&self) -> f64 {
        self.0
    }
}
