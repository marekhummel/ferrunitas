use std::fmt;
use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Sub};
// use typenum::*;

// ============================================================================
// Type-Level Dimensional System
// ============================================================================

// Quantity with type-level dimensional signature
// Dimensions: [Mass, Length, Time, Current, Temperature, Amount, Luminosity]
#[derive(Debug, Clone, Copy)]
pub struct Quantity<M, L, T, I, Theta, N, J> {
    value: f64,
    _phantom: PhantomData<(M, L, T, I, Theta, N, J)>,
}

impl<M, L, T, I, Theta, N, J> Quantity<M, L, T, I, Theta, N, J> {
    pub fn new(value: f64) -> Self {
        Self {
            value,
            _phantom: PhantomData,
        }
    }

    pub fn value(&self) -> f64 {
        self.value
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
    // type Output = Quantity<
    //     Sum<M1, M2>,
    //     Sum<L1, L2>,
    //     Sum<T1, T2>,
    //     Sum<I1, I2>,
    //     Sum<Theta1, Theta2>,
    //     Sum<N1, N2>,
    //     Sum<J1, J2>,
    // >;

    type Output = Quantity<
        <M1 as Add<M2>>::Output,
        <L1 as Add<L2>>::Output,
        <T1 as Add<T2>>::Output,
        <I1 as Add<I2>>::Output,
        <Theta1 as Add<Theta2>>::Output,
        <N1 as Add<N2>>::Output,
        <J1 as Add<J2>>::Output,
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
        <M1 as Sub<M2>>::Output,
        <L1 as Sub<L2>>::Output,
        <T1 as Sub<T2>>::Output,
        <I1 as Sub<I2>>::Output,
        <Theta1 as Sub<Theta2>>::Output,
        <N1 as Sub<N2>>::Output,
        <J1 as Sub<J2>>::Output,
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
