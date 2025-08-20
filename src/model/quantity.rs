use std::fmt;
use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Sub};

use typenum::{Diff, Sum};

use crate::model::unit::Unit;

// ============================================================================
// Type-Level Dimensional System
// ============================================================================

pub(crate) trait QuantityMarker {
    fn new(value: f64) -> Self; // This should not be public
}

// Quantity with type-level dimensional signature
// Dimensions: [Mass, Length, Time, Current, Temperature, Amount, Luminosity]
#[derive(Debug, Clone, Copy)]
pub struct Quantity<M, L, T, I, Theta, N, J> {
    /// This value holds the raw f64 value of the quantity, only meant for internal use
    pub(crate) value: f64,
    _phantom: PhantomData<(M, L, T, I, Theta, N, J)>,
}

impl<M, L, T, I, Theta, N, J> Quantity<M, L, T, I, Theta, N, J> {
    pub fn as_unit<U: Unit<Quantity = Self>>(self) -> U {
        U::from(self)
    }
}

impl<M, L, T, I, Theta, N, J> QuantityMarker for Quantity<M, L, T, I, Theta, N, J> {
    fn new(value: f64) -> Self {
        Self {
            value,
            _phantom: PhantomData,
        }
    }
}

impl<M, L, T, I, Theta, N, J> fmt::Display for Quantity<M, L, T, I, Theta, N, J> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.value, f)
    }
}

// ============================================================================
// Generic Arithmetic Operations (The Magic!)
// ============================================================================

// Addition - only works for same dimensions
impl<M, L, T, I, Theta, N, J> Add for Quantity<M, L, T, I, Theta, N, J> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.value + rhs.value)
    }
}

// Subtraction - only works for same dimensions
impl<M, L, T, I, Theta, N, J> Sub for Quantity<M, L, T, I, Theta, N, J> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.value - rhs.value)
    }
}

// Multiplication - adds dimensions at type level
impl<M1, L1, T1, I1, Theta1, N1, J1, M2, L2, T2, I2, Theta2, N2, J2>
    Mul<Quantity<M2, L2, T2, I2, Theta2, N2, J2>> for Quantity<M1, L1, T1, I1, Theta1, N1, J1>
where
    M1: Add<M2>,
    L1: Add<L2>,
    T1: Add<T2>,
    I1: Add<I2>,
    Theta1: Add<Theta2>,
    N1: Add<N2>,
    J1: Add<J2>,
{
    type Output = Quantity<
        Sum<M1, M2>,
        Sum<L1, L2>,
        Sum<T1, T2>,
        Sum<I1, I2>,
        Sum<Theta1, Theta2>,
        Sum<N1, N2>,
        Sum<J1, J2>,
    >;

    fn mul(self, rhs: Quantity<M2, L2, T2, I2, Theta2, N2, J2>) -> Self::Output {
        Self::Output::new(self.value * rhs.value)
    }
}

// Division - subtracts dimensions at type level
impl<M1, L1, T1, I1, Theta1, N1, J1, M2, L2, T2, I2, Theta2, N2, J2>
    Div<Quantity<M2, L2, T2, I2, Theta2, N2, J2>> for Quantity<M1, L1, T1, I1, Theta1, N1, J1>
where
    M1: Sub<M2>,
    L1: Sub<L2>,
    T1: Sub<T2>,
    I1: Sub<I2>,
    Theta1: Sub<Theta2>,
    N1: Sub<N2>,
    J1: Sub<J2>,
{
    type Output = Quantity<
        Diff<M1, M2>,
        Diff<L1, L2>,
        Diff<T1, T2>,
        Diff<I1, I2>,
        Diff<Theta1, Theta2>,
        Diff<N1, N2>,
        Diff<J1, J2>,
    >;

    fn div(self, rhs: Quantity<M2, L2, T2, I2, Theta2, N2, J2>) -> Self::Output {
        Self::Output::new(self.value / rhs.value)
    }
}

// Scalar multiplication - scales the value but keeps dimensions
impl<M, L, T, I, Theta, N, J> Mul<f64> for Quantity<M, L, T, I, Theta, N, J> {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
        Self::new(self.value * scalar)
    }
}

// Scalar division - scales the value but keeps dimensions
impl<M, L, T, I, Theta, N, J> Div<f64> for Quantity<M, L, T, I, Theta, N, J> {
    type Output = Self;

    fn div(self, scalar: f64) -> Self::Output {
        Self::new(self.value / scalar)
    }
}
