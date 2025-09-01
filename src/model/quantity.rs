use std::fmt::{self, Debug};
use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::common;
use crate::model::dimension::{Dimensioned, TypePow};
use crate::model::unit::{Unit, UnitBase};
use num_traits::Inv;
use typenum::{Diff, Integer, Negate, NonZero, Prod, Sum, ToInt};

// ===========================
// Type-Level Dimensional System
// ===========================

/// Quantity with type-level dimensional signature
/// Dimensions: [Mass, Length, Time, Current, Temperature, Amount, Luminosity]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Quantity<D: Dimensioned> {
    /// This value holds the raw f64 value of the quantity, only meant for internal use
    value: f64,
    _phantom: PhantomData<D>,
}

/// Default impl
impl<D: Dimensioned> Quantity<D> {
    /// Create a new Quantity. Note that this is not intended for public use, use concrete units and convert to quantity.
    #[doc(hidden)]
    pub fn new(value: f64) -> Self {
        Self {
            value,
            _phantom: PhantomData,
        }
    }

    pub const fn new_const(value: f64) -> Self {
        Self {
            value,
            _phantom: PhantomData,
        }
    }

    pub fn raw_value(&self) -> f64 {
        self.value
    }

    pub const fn value_const(&self) -> f64 {
        self.value
    }

    pub fn format<U: UnitBase>(&self) -> String {
        format!("{} {}", self.raw_value() / U::FACTOR, U::ABBREV)
    }
}

/// Make sure quantity is sealed for QuantityMarker trait
impl<D: Dimensioned> crate::sealed::Sealed for Quantity<D> {}

/// Basic handlers for quantities
impl<D: Dimensioned> Dimensioned for Quantity<D> {
    type M = D::M;
    type L = D::L;
    type T = D::T;
    type I = D::I;
    type Th = D::Th;
    type N = D::N;
    type J = D::J;
}

/// Marker trait for types to be used in macros, not meant for outside usage
#[doc(hidden)]
pub trait QuantityMarker: Sized + Debug + Clone + Copy + crate::sealed::Sealed {
    type DimensionVector: Dimensioned;

    fn new(value: f64) -> Self;
    fn raw_value(&self) -> f64;

    // fn as_unit<U: ToQuantity<BaseQuantity = Self>>(&self) -> U {
    //     U::internal_from_q(*self)
    // }
}

/// Conversion for quantities
impl<D: Dimensioned> QuantityMarker for Quantity<D> {
    type DimensionVector = D;

    fn new(value: f64) -> Self {
        Self::new(value)
    }

    fn raw_value(&self) -> f64 {
        self.value
    }
}

/// Display of quantity
impl<D: Dimensioned> fmt::Display for Quantity<D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let dim_string = crate::common::format_quantity::<Self>();
        std::fmt::Display::fmt(&self.value, f)?;
        write!(f, " [{}]", dim_string)?;
        Ok(())
    }
}

// ===========================
// Generic Arithmetic Operations
// ===========================

// Addition - only works for same dimensions
impl<D: Dimensioned> Add<Quantity<D>> for Quantity<D>
where
    D: Add,
    <D as Add>::Output: Dimensioned,
{
    type Output = Quantity<<D as Add>::Output>;

    fn add(self, rhs: Quantity<D>) -> Self::Output {
        Self::Output::new(self.raw_value() + rhs.raw_value())
    }
}

impl<D: Dimensioned> AddAssign<Quantity<D>> for Quantity<D> {
    fn add_assign(&mut self, rhs: Quantity<D>) {
        self.value += rhs.raw_value();
    }
}

// Subtraction - only works for same dimensions
impl<D: Dimensioned> Sub<Quantity<D>> for Quantity<D>
where
    D: Sub,
    <D as Sub>::Output: Dimensioned,
{
    type Output = Quantity<<D as Sub>::Output>;

    fn sub(self, rhs: Quantity<D>) -> Self::Output {
        Self::Output::new(self.raw_value() - rhs.raw_value())
    }
}

impl<D: Dimensioned> SubAssign<Quantity<D>> for Quantity<D> {
    fn sub_assign(&mut self, rhs: Quantity<D>) {
        self.value -= rhs.raw_value();
    }
}

// Multiplication - adds dimensions at type level
impl<D1: Dimensioned, D2: Dimensioned> Mul<Quantity<D2>> for Quantity<D1>
where
    D1: Mul<D2>,
    <D1 as Mul<D2>>::Output: Dimensioned,
{
    type Output = Quantity<<D1 as Mul<D2>>::Output>;

    fn mul(self, rhs: Quantity<D2>) -> Self::Output {
        Self::Output::new(self.raw_value() * rhs.raw_value())
    }
}

// Division - subtracts dimensions at type level
impl<D1: Dimensioned, D2: Dimensioned> Div<Quantity<D2>> for Quantity<D1>
where
    D1: Div<D2>,
    <D1 as Div<D2>>::Output: Dimensioned,
{
    type Output = Quantity<<D1 as Div<D2>>::Output>;

    fn div(self, rhs: Quantity<D2>) -> Self::Output {
        Self::Output::new(self.raw_value() / rhs.raw_value())
    }
}

// Scalar multiplication - scales the value but keeps dimensions
impl<D: Dimensioned> Mul<f64> for Quantity<D> {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
        Self::new(self.value * scalar)
    }
}

impl<D: Dimensioned> Mul<Quantity<D>> for f64 {
    type Output = Quantity<D>;

    fn mul(self, quantity: Quantity<D>) -> Self::Output {
        Self::Output::new(self * quantity.value)
    }
}

impl<D: Dimensioned> MulAssign<f64> for Quantity<D> {
    fn mul_assign(&mut self, scalar: f64) {
        self.value *= scalar;
    }
}

// Scalar division - scales the value but keeps dimensions
impl<D: Dimensioned> Div<f64> for Quantity<D> {
    type Output = Self;

    fn div(self, scalar: f64) -> Self::Output {
        Self::new(self.value / scalar)
    }
}

impl<D: Dimensioned> DivAssign<f64> for Quantity<D> {
    fn div_assign(&mut self, scalar: f64) {
        self.value /= scalar;
    }
}

// Division with inverse - Scale and inverse at the same time
impl<D: Dimensioned> Div<Quantity<D>> for f64
where
    D: Inv,
    <D as Inv>::Output: Dimensioned,
{
    type Output = Quantity<<D as Inv>::Output>;

    fn div(self, quantity: Quantity<D>) -> Self::Output {
        Self::Output::new(self / quantity.raw_value())
    }
}

impl<D: Dimensioned, Exp> TypePow<Exp> for Quantity<D>
where
    D: TypePow<Exp>,
    <D as TypePow<Exp>>::Output: Dimensioned,
    Exp: Integer + NonZero + ToInt<i32>,
{
    type Output = Quantity<<D as TypePow<Exp>>::Output>;

    fn pow(self) -> Self::Output {
        Self::Output::new(self.value.powi(Exp::INT))
    }
}
