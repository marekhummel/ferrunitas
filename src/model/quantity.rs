use std::fmt::{self, Debug};
use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::model::unit::Unit;
use num_traits::Inv;
use typenum::{Diff, Integer, Negate, NonZero, Prod, Sum, ToInt};

// ===========================
// Type-Level Dimensional System
// ===========================

/// Marker trait for all quantity types, used in macros
pub trait QuantityMarker: Debug + Clone + Copy + PartialOrd + PartialEq {
    type M: Debug + Clone + Copy + PartialOrd + PartialEq;
    type L: Debug + Clone + Copy + PartialOrd + PartialEq;
    type T: Debug + Clone + Copy + PartialOrd + PartialEq;
    type I: Debug + Clone + Copy + PartialOrd + PartialEq;
    type Th: Debug + Clone + Copy + PartialOrd + PartialEq;
    type N: Debug + Clone + Copy + PartialOrd + PartialEq;
    type J: Debug + Clone + Copy + PartialOrd + PartialEq;

    fn new(value: f64) -> Self; // This should not be public
    fn raw_value(&self) -> f64;
}

/// Marker trait for types that can be converted into a unit
pub trait IntoUnit: Sized + Clone {
    fn into_unit<U: Unit<Quantity = Self>>(self) -> U {
        U::from_q(self)
    }
    fn to_unit<U: Unit<Quantity = Self>>(&self) -> U {
        self.clone().into_unit()
    }
}

/// Quantity with type-level dimensional signature
/// Dimensions: [Mass, Length, Time, Current, Temperature, Amount, Luminosity]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Quantity<M, L, T, I, Th, N, J> {
    /// This value holds the raw f64 value of the quantity, only meant for internal use
    value: f64,
    _phantom: PhantomData<(M, L, T, I, Th, N, J)>,
}

/// Basic handlers for quantities
impl<M, L, T, I, Th, N, J> QuantityMarker for Quantity<M, L, T, I, Th, N, J>
where
    M: Debug + Clone + Copy + PartialOrd + PartialEq,
    L: Debug + Clone + Copy + PartialOrd + PartialEq,
    T: Debug + Clone + Copy + PartialOrd + PartialEq,
    I: Debug + Clone + Copy + PartialOrd + PartialEq,
    Th: Debug + Clone + Copy + PartialOrd + PartialEq,
    N: Debug + Clone + Copy + PartialOrd + PartialEq,
    J: Debug + Clone + Copy + PartialOrd + PartialEq,
{
    type M = M;
    type L = L;
    type T = T;
    type I = I;
    type Th = Th;
    type N = N;
    type J = J;

    fn new(value: f64) -> Self {
        Self {
            value,
            _phantom: PhantomData,
        }
    }
    fn raw_value(&self) -> f64 {
        self.value
    }
}

/// Conversion for quantities
impl<M, L, T, I, Th, N, J> IntoUnit for Quantity<M, L, T, I, Th, N, J>
where
    M: Sized + Clone,
    L: Sized + Clone,
    T: Sized + Clone,
    I: Sized + Clone,
    Th: Sized + Clone,
    N: Sized + Clone,
    J: Sized + Clone,
{
    fn into_unit<U: Unit<Quantity = Self>>(self) -> U {
        U::from_q(self)
    }
}

/// Quick conversion without trait
impl<M, L, T, I, Th, N, J> Quantity<M, L, T, I, Th, N, J> {
    pub fn as_unit<U: Unit<Quantity = Self>>(self) -> U {
        U::from_q(self)
    }
}

/// Display of quantity
impl<M, L, T, I, Th, N, J> fmt::Display for Quantity<M, L, T, I, Th, N, J>
where
    M: typenum::Integer + Debug + PartialOrd + PartialEq,
    L: typenum::Integer + Debug + PartialOrd + PartialEq,
    T: typenum::Integer + Debug + PartialOrd + PartialEq,
    I: typenum::Integer + Debug + PartialOrd + PartialEq,
    Th: typenum::Integer + Debug + PartialOrd + PartialEq,
    N: typenum::Integer + Debug + PartialOrd + PartialEq,
    J: typenum::Integer + Debug + PartialOrd + PartialEq,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let dim_string = crate::format_quantity_dims!(Self);
        std::fmt::Display::fmt(&self.value, f)?;
        write!(f, " [{}]", dim_string)?;
        Ok(())
    }
}

// ===========================
// Generic Arithmetic Operations
// ===========================

// Addition - only works for same dimensions
impl<M, L, T, I, Th, N, J> Add for Quantity<M, L, T, I, Th, N, J>
where
    M: Debug + Clone + Copy + PartialOrd + PartialEq,
    L: Debug + Clone + Copy + PartialOrd + PartialEq,
    T: Debug + Clone + Copy + PartialOrd + PartialEq,
    I: Debug + Clone + Copy + PartialOrd + PartialEq,
    Th: Debug + Clone + Copy + PartialOrd + PartialEq,
    N: Debug + Clone + Copy + PartialOrd + PartialEq,
    J: Debug + Clone + Copy + PartialOrd + PartialEq,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.value + rhs.value)
    }
}

impl<M, L, T, I, Th, N, J> AddAssign for Quantity<M, L, T, I, Th, N, J> {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
    }
}

impl<M, L, T, I, Th, N, J, U> Add<U> for Quantity<M, L, T, I, Th, N, J>
where
    U: Unit<Quantity = Self>,
    M: Debug + Clone + Copy + PartialOrd + PartialEq,
    L: Debug + Clone + Copy + PartialOrd + PartialEq,
    T: Debug + Clone + Copy + PartialOrd + PartialEq,
    I: Debug + Clone + Copy + PartialOrd + PartialEq,
    Th: Debug + Clone + Copy + PartialOrd + PartialEq,
    N: Debug + Clone + Copy + PartialOrd + PartialEq,
    J: Debug + Clone + Copy + PartialOrd + PartialEq,
{
    type Output = Self;

    fn add(self, rhs: U) -> Self::Output {
        self + rhs.into_q()
    }
}

impl<M, L, T, I, Th, N, J, U> AddAssign<U> for Quantity<M, L, T, I, Th, N, J>
where
    U: Unit<Quantity = Self>,
{
    fn add_assign(&mut self, rhs: U) {
        *self += rhs.into_q();
    }
}

// Subtraction - only works for same dimensions
impl<M, L, T, I, Th, N, J> Sub for Quantity<M, L, T, I, Th, N, J>
where
    M: Debug + Clone + Copy + PartialOrd + PartialEq,
    L: Debug + Clone + Copy + PartialOrd + PartialEq,
    T: Debug + Clone + Copy + PartialOrd + PartialEq,
    I: Debug + Clone + Copy + PartialOrd + PartialEq,
    Th: Debug + Clone + Copy + PartialOrd + PartialEq,
    N: Debug + Clone + Copy + PartialOrd + PartialEq,
    J: Debug + Clone + Copy + PartialOrd + PartialEq,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.value - rhs.value)
    }
}

impl<M, L, T, I, Th, N, J> SubAssign for Quantity<M, L, T, I, Th, N, J> {
    fn sub_assign(&mut self, rhs: Self) {
        self.value -= rhs.value;
    }
}

impl<M, L, T, I, Th, N, J, U> Sub<U> for Quantity<M, L, T, I, Th, N, J>
where
    U: Unit<Quantity = Self>,
    M: Debug + Clone + Copy + PartialOrd + PartialEq,
    L: Debug + Clone + Copy + PartialOrd + PartialEq,
    T: Debug + Clone + Copy + PartialOrd + PartialEq,
    I: Debug + Clone + Copy + PartialOrd + PartialEq,
    Th: Debug + Clone + Copy + PartialOrd + PartialEq,
    N: Debug + Clone + Copy + PartialOrd + PartialEq,
    J: Debug + Clone + Copy + PartialOrd + PartialEq,
{
    type Output = Self;

    fn sub(self, rhs: U) -> Self::Output {
        self - rhs.into_q()
    }
}

impl<M, L, T, I, Th, N, J, U> SubAssign<U> for Quantity<M, L, T, I, Th, N, J>
where
    U: Unit<Quantity = Self>,
{
    fn sub_assign(&mut self, rhs: U) {
        *self -= rhs.into_q();
    }
}

// Multiplication - adds dimensions at type level
impl<M1, L1, T1, I1, Th1, N1, J1, M2, L2, T2, I2, Th2, N2, J2>
    Mul<Quantity<M2, L2, T2, I2, Th2, N2, J2>> for Quantity<M1, L1, T1, I1, Th1, N1, J1>
where
    M1: Add<M2>,
    L1: Add<L2>,
    T1: Add<T2>,
    I1: Add<I2>,
    Th1: Add<Th2>,
    N1: Add<N2>,
    J1: Add<J2>,
    Sum<M1, M2>: Debug + Clone + Copy + PartialOrd + PartialEq,
    Sum<L1, L2>: Debug + Clone + Copy + PartialOrd + PartialEq,
    Sum<T1, T2>: Debug + Clone + Copy + PartialOrd + PartialEq,
    Sum<I1, I2>: Debug + Clone + Copy + PartialOrd + PartialEq,
    Sum<Th1, Th2>: Debug + Clone + Copy + PartialOrd + PartialEq,
    Sum<N1, N2>: Debug + Clone + Copy + PartialOrd + PartialEq,
    Sum<J1, J2>: Debug + Clone + Copy + PartialOrd + PartialEq,
{
    type Output = Quantity<
        Sum<M1, M2>,
        Sum<L1, L2>,
        Sum<T1, T2>,
        Sum<I1, I2>,
        Sum<Th1, Th2>,
        Sum<N1, N2>,
        Sum<J1, J2>,
    >;

    fn mul(self, rhs: Quantity<M2, L2, T2, I2, Th2, N2, J2>) -> Self::Output {
        Self::Output::new(self.value * rhs.value)
    }
}

impl<M, L, T, I, Th, N, J, U> Mul<U> for Quantity<M, L, T, I, Th, N, J>
where
    U: Unit,
    Self: Mul<<U as Unit>::Quantity>,
{
    type Output = <Self as Mul<<U as Unit>::Quantity>>::Output;

    fn mul(self, rhs: U) -> Self::Output {
        self * rhs.into_q()
    }
}

// Division - subtracts dimensions at type level
impl<M1, L1, T1, I1, Th1, N1, J1, M2, L2, T2, I2, Th2, N2, J2>
    Div<Quantity<M2, L2, T2, I2, Th2, N2, J2>> for Quantity<M1, L1, T1, I1, Th1, N1, J1>
where
    M1: Sub<M2>,
    L1: Sub<L2>,
    T1: Sub<T2>,
    I1: Sub<I2>,
    Th1: Sub<Th2>,
    N1: Sub<N2>,
    J1: Sub<J2>,
    Diff<M1, M2>: Debug + Clone + Copy + PartialOrd + PartialEq,
    Diff<L1, L2>: Debug + Clone + Copy + PartialOrd + PartialEq,
    Diff<T1, T2>: Debug + Clone + Copy + PartialOrd + PartialEq,
    Diff<I1, I2>: Debug + Clone + Copy + PartialOrd + PartialEq,
    Diff<Th1, Th2>: Debug + Clone + Copy + PartialOrd + PartialEq,
    Diff<N1, N2>: Debug + Clone + Copy + PartialOrd + PartialEq,
    Diff<J1, J2>: Debug + Clone + Copy + PartialOrd + PartialEq,
{
    type Output = Quantity<
        Diff<M1, M2>,
        Diff<L1, L2>,
        Diff<T1, T2>,
        Diff<I1, I2>,
        Diff<Th1, Th2>,
        Diff<N1, N2>,
        Diff<J1, J2>,
    >;

    fn div(self, rhs: Quantity<M2, L2, T2, I2, Th2, N2, J2>) -> Self::Output {
        Self::Output::new(self.value / rhs.value)
    }
}

impl<M, L, T, I, Th, N, J, U> Div<U> for Quantity<M, L, T, I, Th, N, J>
where
    U: Unit,
    Self: Div<<U as Unit>::Quantity>,
{
    type Output = <Self as Div<<U as Unit>::Quantity>>::Output;

    fn div(self, rhs: U) -> Self::Output {
        self / rhs.into_q()
    }
}

// Scalar multiplication - scales the value but keeps dimensions
impl<M, L, T, I, Th, N, J> Mul<f64> for Quantity<M, L, T, I, Th, N, J>
where
    M: Debug + Clone + Copy + PartialOrd + PartialEq,
    L: Debug + Clone + Copy + PartialOrd + PartialEq,
    T: Debug + Clone + Copy + PartialOrd + PartialEq,
    I: Debug + Clone + Copy + PartialOrd + PartialEq,
    Th: Debug + Clone + Copy + PartialOrd + PartialEq,
    N: Debug + Clone + Copy + PartialOrd + PartialEq,
    J: Debug + Clone + Copy + PartialOrd + PartialEq,
{
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
        Self::new(self.value * scalar)
    }
}

impl<M, L, T, I, Th, N, J> Mul<Quantity<M, L, T, I, Th, N, J>> for f64
where
    M: Debug + Clone + Copy + PartialOrd + PartialEq,
    L: Debug + Clone + Copy + PartialOrd + PartialEq,
    T: Debug + Clone + Copy + PartialOrd + PartialEq,
    I: Debug + Clone + Copy + PartialOrd + PartialEq,
    Th: Debug + Clone + Copy + PartialOrd + PartialEq,
    N: Debug + Clone + Copy + PartialOrd + PartialEq,
    J: Debug + Clone + Copy + PartialOrd + PartialEq,
{
    type Output = Quantity<M, L, T, I, Th, N, J>;

    fn mul(self, quantity: Quantity<M, L, T, I, Th, N, J>) -> Self::Output {
        Self::Output::new(self * quantity.value)
    }
}

impl<M, L, T, I, Th, N, J> MulAssign<f64> for Quantity<M, L, T, I, Th, N, J> {
    fn mul_assign(&mut self, scalar: f64) {
        self.value *= scalar;
    }
}

// Scalar division - scales the value but keeps dimensions
impl<M, L, T, I, Th, N, J> Div<f64> for Quantity<M, L, T, I, Th, N, J>
where
    M: Debug + Clone + Copy + PartialOrd + PartialEq,
    L: Debug + Clone + Copy + PartialOrd + PartialEq,
    T: Debug + Clone + Copy + PartialOrd + PartialEq,
    I: Debug + Clone + Copy + PartialOrd + PartialEq,
    Th: Debug + Clone + Copy + PartialOrd + PartialEq,
    N: Debug + Clone + Copy + PartialOrd + PartialEq,
    J: Debug + Clone + Copy + PartialOrd + PartialEq,
{
    type Output = Self;

    fn div(self, scalar: f64) -> Self::Output {
        Self::new(self.value / scalar)
    }
}

impl<M, L, T, I, Th, N, J> DivAssign<f64> for Quantity<M, L, T, I, Th, N, J> {
    fn div_assign(&mut self, scalar: f64) {
        self.value /= scalar;
    }
}

// Division with inverse - Scale and inverse at the same time
impl<M, L, T, I, Th, N, J> Div<Quantity<M, L, T, I, Th, N, J>> for f64
where
    M: Neg + Debug + Clone + Copy + PartialOrd + PartialEq,
    L: Neg + Debug + Clone + Copy + PartialOrd + PartialEq,
    T: Neg + Debug + Clone + Copy + PartialOrd + PartialEq,
    I: Neg + Debug + Clone + Copy + PartialOrd + PartialEq,
    Th: Neg + Debug + Clone + Copy + PartialOrd + PartialEq,
    N: Neg + Debug + Clone + Copy + PartialOrd + PartialEq,
    J: Neg + Debug + Clone + Copy + PartialOrd + PartialEq,
    Negate<M>: Debug + Clone + Copy + PartialOrd + PartialEq,
    Negate<L>: Debug + Clone + Copy + PartialOrd + PartialEq,
    Negate<T>: Debug + Clone + Copy + PartialOrd + PartialEq,
    Negate<I>: Debug + Clone + Copy + PartialOrd + PartialEq,
    Negate<Th>: Debug + Clone + Copy + PartialOrd + PartialEq,
    Negate<N>: Debug + Clone + Copy + PartialOrd + PartialEq,
    Negate<J>: Debug + Clone + Copy + PartialOrd + PartialEq,
{
    type Output =
        Quantity<Negate<M>, Negate<L>, Negate<T>, Negate<I>, Negate<Th>, Negate<N>, Negate<J>>;

    fn div(self, quantity: Quantity<M, L, T, I, Th, N, J>) -> Self::Output {
        Self::Output::new(self / quantity.value)
    }
}

// Inverse quantity
impl<M, L, T, I, Th, N, J> Inv for Quantity<M, L, T, I, Th, N, J>
where
    M: Neg + Debug + Clone + Copy + PartialOrd + PartialEq,
    L: Neg + Debug + Clone + Copy + PartialOrd + PartialEq,
    T: Neg + Debug + Clone + Copy + PartialOrd + PartialEq,
    I: Neg + Debug + Clone + Copy + PartialOrd + PartialEq,
    Th: Neg + Debug + Clone + Copy + PartialOrd + PartialEq,
    N: Neg + Debug + Clone + Copy + PartialOrd + PartialEq,
    J: Neg + Debug + Clone + Copy + PartialOrd + PartialEq,
    Negate<M>: Debug + Clone + Copy + PartialOrd + PartialEq,
    Negate<L>: Debug + Clone + Copy + PartialOrd + PartialEq,
    Negate<T>: Debug + Clone + Copy + PartialOrd + PartialEq,
    Negate<I>: Debug + Clone + Copy + PartialOrd + PartialEq,
    Negate<Th>: Debug + Clone + Copy + PartialOrd + PartialEq,
    Negate<N>: Debug + Clone + Copy + PartialOrd + PartialEq,
    Negate<J>: Debug + Clone + Copy + PartialOrd + PartialEq,
{
    type Output =
        Quantity<Negate<M>, Negate<L>, Negate<T>, Negate<I>, Negate<Th>, Negate<N>, Negate<J>>;

    fn inv(self) -> Self::Output {
        Self::Output::new(1.0 / self.value)
    }
}

/// Exponentiation of quantity - multiplies dimensions by exponent
/// (new trait cause exponent is completely given by type)
pub trait TypePow<Exp>
where
    Exp: Integer + NonZero + ToInt<i32>,
{
    type Output;
    fn pow(self) -> Self::Output;
}

impl<M, L, T, I, Th, N, J, Exp> TypePow<Exp> for Quantity<M, L, T, I, Th, N, J>
where
    M: Mul<Exp>,
    L: Mul<Exp>,
    T: Mul<Exp>,
    I: Mul<Exp>,
    Th: Mul<Exp>,
    N: Mul<Exp>,
    J: Mul<Exp>,
    Exp: Integer + NonZero + ToInt<i32>,
    Prod<M, Exp>: Debug + Clone + Copy + PartialOrd + PartialEq,
    Prod<L, Exp>: Debug + Clone + Copy + PartialOrd + PartialEq,
    Prod<T, Exp>: Debug + Clone + Copy + PartialOrd + PartialEq,
    Prod<I, Exp>: Debug + Clone + Copy + PartialOrd + PartialEq,
    Prod<Th, Exp>: Debug + Clone + Copy + PartialOrd + PartialEq,
    Prod<N, Exp>: Debug + Clone + Copy + PartialOrd + PartialEq,
    Prod<J, Exp>: Debug + Clone + Copy + PartialOrd + PartialEq,
{
    type Output = Quantity<
        Prod<M, Exp>,
        Prod<L, Exp>,
        Prod<T, Exp>,
        Prod<I, Exp>,
        Prod<Th, Exp>,
        Prod<N, Exp>,
        Prod<J, Exp>,
    >;

    fn pow(self) -> Self::Output {
        Self::Output::new(self.value.powi(Exp::INT))
    }
}
