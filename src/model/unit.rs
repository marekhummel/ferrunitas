use std::fmt::Debug;

use crate::model::{dimension::Dimensioned, measure::Measure, quantity::QuantityMarker};

pub trait UnitBase: crate::sealed::Sealed + Debug + Clone + Copy {
    type DimensionVector: Dimensioned;
    const FACTOR: f64;
    const ABBREV: &'static str;
}

impl<U> crate::sealed::Sealed for U where U: UnitBase {}

/// Top-Level definition of a Unit. Note that the arithmetic traits are incomplete,
/// as they do not cover multiplication between units, since the type system can't cover things
/// like "+ for Q Mul<impl Unit<Quantity = Q>, Output=Self::Quantity as Mul<Q>>".
pub trait Unit:
    UnitBase<DimensionVector = <Self::Quantity as QuantityMarker>::DimensionVector>
{
    type Quantity: QuantityMarker + Dimensioned;

    /// Create a new measure from a raw value
    fn new(value: impl Into<f64>) -> Measure<Self> {
        Measure::new(value.into())
    }
}
