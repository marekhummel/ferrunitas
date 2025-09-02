//! Unit-agnostic dimensioned magnitude (`Quantity<D>`) for arithmetic & conversion.
//! Quantity supports all arithmetics with measures. Apart from that quantities can
//! be created with the `quantity!` macro and implement some traits useful for type-safety.

use std::fmt::{self, Debug};
use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::model::dimension::{Dimensioned, TypePow};
use crate::model::measure::Measure;
use crate::model::unit::Unit;
use num_traits::Inv;
use typenum::{Integer, NonZero, ToInt};

/// Quantity with type-level dimensional signature
/// Dimensions: [Mass, Length, Time, Current, Temperature, Amount, Luminosity]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Quantity<D: Dimensioned> {
    /// This value holds the raw f64 value of the quantity, only meant for internal use
    pub(crate) value: f64,
    _phantom: PhantomData<D>,
}

/// Marker trait for types to be used in which the Unit::Quantity
#[doc(hidden)]
pub trait QuantityMarker:
    Sized + Debug + Clone + Copy + PartialEq + crate::model::sealed::Sealed
{
    fn new(value: f64) -> Self;
    fn raw_value(&self) -> f64;
}

// ===========================
// MACRO
// ===========================

/// Define a new quantity either by a list of 7 dimensions or by compounding others.
#[macro_export]
macro_rules! quantity {
    // Literal case
    ($quantity:ident: M $mass:ty, L $length:ty, T $time:ty, I $current:ty, Th $temperature:ty, N $amount:ty, J $luminosity:ty) => {
        quantity!(
            $quantity:
            $crate::__model::Quantity<
                $crate::__model::DimensionVector<
                    $mass, $length, $time, $current, $temperature, $amount, $luminosity
                >
            >;
        );
    };

    // External compound case
    ($comp_quantity:ident: [$(($quantities:ty, $exps:ty)),+]) => {
        quantity!(
            $comp_quantity:
            $crate::__model::Quantity<$crate::__model::DimensionZero>; $(($quantities, $exps)),+
        );
    };

    // Recursive case
    ($comp_quantity:ident: $quantity_acc:ty; ($quantity:ty, $exp:ty) $(, ($quantities:ty, $exps:ty))*) => {
        quantity!(
            $comp_quantity:
            <$quantity_acc as std::ops::Mul<
                <$quantity as $crate::__model::TypePow<$exp>>::Output
            >>::Output;
            $(($quantities, $exps)),*
        );
    };

    // Base case
    ($comp_quantity:ident: $quantity_acc:ty;) => {
        pub type $comp_quantity = $quantity_acc;
    };
}

// ===========================
// IMPLS
// ===========================

/// Default impl
impl<D: Dimensioned> Quantity<D> {
    /// Create a new Quantity.
    pub(crate) fn new(value: f64) -> Self {
        Self {
            value,
            _phantom: PhantomData,
        }
    }

    /// Creates quantity instance based on value and unit
    pub fn from<U: Unit<Quantity = Self>>(value: impl Into<f64>) -> Self {
        Measure::<U>::new(value.into()).into_q()
    }

    /// Converts this quantity into a measure of given unit
    pub fn as_measure<U: Unit<Quantity = Self>>(&self) -> Measure<U> {
        Measure::from_q(*self)
    }

    /// Direct convert of a unit into another of this quantity.
    pub fn convert<U1, U2>(value: f64) -> Measure<U2>
    where
        U1: Unit<Quantity = Self>,
        U2: Unit<Quantity = Self>,
    {
        U1::new(value).convert::<U2>()
    }
}

/// Make sure quantity is sealed for QuantityMarker trait
impl<D: Dimensioned> crate::model::sealed::Sealed for Quantity<D> {}

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

impl<D: Dimensioned> QuantityMarker for Quantity<D> {
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
        let dim_string = crate::common::format_dims::<Self>();
        std::fmt::Display::fmt(&self.value, f)?;
        write!(f, " [{}]", dim_string)?;
        Ok(())
    }
}

// ===========================
// Generic Arithmetic Operations
// ===========================

// Addition - only works for same dimensions
impl<D> Add<Quantity<D>> for Quantity<D>
where
    D: Dimensioned + Add,
    <D as Add>::Output: Dimensioned,
{
    type Output = Quantity<<D as Add>::Output>;

    fn add(self, rhs: Quantity<D>) -> Self::Output {
        Self::Output::new(self.raw_value() + rhs.raw_value())
    }
}

impl<D, U> Add<Measure<U>> for Quantity<D>
where
    D: Dimensioned + Add,
    U: Unit<Quantity = Quantity<D>>,
    <D as Add>::Output: Dimensioned,
{
    type Output = Quantity<<D as Add>::Output>;

    fn add(self, rhs: Measure<U>) -> Self::Output {
        Self::Output::new(self.raw_value() + rhs.into_q().raw_value())
    }
}

impl<D: Dimensioned> AddAssign<Quantity<D>> for Quantity<D> {
    fn add_assign(&mut self, rhs: Quantity<D>) {
        self.value += rhs.raw_value();
    }
}

impl<D: Dimensioned, U> AddAssign<Measure<U>> for Quantity<D>
where
    U: Unit<Quantity = Quantity<D>>,
{
    fn add_assign(&mut self, rhs: Measure<U>) {
        *self += rhs.into_q();
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

impl<D, U> Sub<Measure<U>> for Quantity<D>
where
    D: Dimensioned + Sub,
    U: Unit<Quantity = Quantity<D>>,
    <D as Sub>::Output: Dimensioned,
{
    type Output = Quantity<<D as Sub>::Output>;

    fn sub(self, rhs: Measure<U>) -> Self::Output {
        self - rhs.into_q()
    }
}

impl<D: Dimensioned> SubAssign<Quantity<D>> for Quantity<D> {
    fn sub_assign(&mut self, rhs: Quantity<D>) {
        self.value -= rhs.raw_value();
    }
}

impl<D: Dimensioned, U> SubAssign<Measure<U>> for Quantity<D>
where
    U: Unit<Quantity = Quantity<D>>,
{
    fn sub_assign(&mut self, rhs: Measure<U>) {
        *self -= rhs.into_q();
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

impl<D1: Dimensioned, D2: Dimensioned, U> Mul<Measure<U>> for Quantity<D1>
where
    U: Unit<Quantity = Quantity<D2>>,
    D1: Mul<D2>,
    <D1 as Mul<D2>>::Output: Dimensioned,
{
    type Output = Quantity<<D1 as Mul<D2>>::Output>;

    fn mul(self, rhs: Measure<U>) -> Self::Output {
        self * rhs.into_q()
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

impl<D1: Dimensioned, D2: Dimensioned, U> Div<Measure<U>> for Quantity<D1>
where
    U: Unit<Quantity = Quantity<D2>>,
    D1: Div<D2>,
    <D1 as Div<D2>>::Output: Dimensioned,
{
    type Output = Quantity<<D1 as Div<D2>>::Output>;

    fn div(self, rhs: Measure<U>) -> Self::Output {
        self / rhs.into_q()
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
