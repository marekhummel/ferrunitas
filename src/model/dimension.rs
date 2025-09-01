use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Neg, Sub};

use num_traits::Inv;
use typenum::*;

/// Dimension marker trait
pub trait Dimension: Debug + Clone + Copy + PartialOrd + PartialEq {
    fn to_int() -> i8;
}
impl<T: typenum::Integer + Debug + PartialOrd> Dimension for T {
    fn to_int() -> i8 {
        Self::to_i8()
    }
}

/// Marker trait for all quantity types to extract dimensional components
pub trait Dimensioned: Debug + Clone + Copy + PartialEq {
    type M: Dimension;
    type L: Dimension;
    type T: Dimension;
    type I: Dimension;
    type Th: Dimension;
    type N: Dimension;
    type J: Dimension;
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct DimensionVector<M, L, T, I, Th, N, J>
where
    M: Dimension,
    L: Dimension,
    T: Dimension,
    I: Dimension,
    Th: Dimension,
    N: Dimension,
    J: Dimension,
{
    _phantom: PhantomData<(M, L, T, I, Th, N, J)>,
}

pub type DimensionZero = DimensionVector<Z0, Z0, Z0, Z0, Z0, Z0, Z0>;

impl<M, L, T, I, Th, N, J> Dimensioned for DimensionVector<M, L, T, I, Th, N, J>
where
    M: Dimension,
    L: Dimension,
    T: Dimension,
    I: Dimension,
    Th: Dimension,
    N: Dimension,
    J: Dimension,
{
    type M = M;
    type L = L;
    type T = T;
    type I = I;
    type Th = Th;
    type N = N;
    type J = J;
}

impl<M, L, T, I, Th, N, J> Default for DimensionVector<M, L, T, I, Th, N, J>
where
    M: Dimension,
    L: Dimension,
    T: Dimension,
    I: Dimension,
    Th: Dimension,
    N: Dimension,
    J: Dimension,
{
    fn default() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

// ARITHMETIC

impl<M, L, T, I, Th, N, J> Add for DimensionVector<M, L, T, I, Th, N, J>
where
    M: Dimension,
    L: Dimension,
    T: Dimension,
    I: Dimension,
    Th: Dimension,
    N: Dimension,
    J: Dimension,
{
    type Output = Self;

    fn add(self, _: Self) -> Self::Output {
        Self::Output::default()
    }
}

// Subtraction - only works for same dimensions
impl<M, L, T, I, Th, N, J> Sub for DimensionVector<M, L, T, I, Th, N, J>
where
    M: Dimension,
    L: Dimension,
    T: Dimension,
    I: Dimension,
    Th: Dimension,
    N: Dimension,
    J: Dimension,
{
    type Output = Self;

    fn sub(self, _: Self) -> Self::Output {
        Self::Output::default()
    }
}

// // Multiplication - adds dimensions at type level
impl<M1, L1, T1, I1, Th1, N1, J1, M2, L2, T2, I2, Th2, N2, J2>
    Mul<DimensionVector<M2, L2, T2, I2, Th2, N2, J2>>
    for DimensionVector<M1, L1, T1, I1, Th1, N1, J1>
where
    M1: Dimension + Add<M2>,
    L1: Dimension + Add<L2>,
    T1: Dimension + Add<T2>,
    I1: Dimension + Add<I2>,
    Th1: Dimension + Add<Th2>,
    N1: Dimension + Add<N2>,
    J1: Dimension + Add<J2>,
    M2: Dimension,
    L2: Dimension,
    T2: Dimension,
    I2: Dimension,
    Th2: Dimension,
    N2: Dimension,
    J2: Dimension,
    Sum<M1, M2>: Dimension,
    Sum<L1, L2>: Dimension,
    Sum<T1, T2>: Dimension,
    Sum<I1, I2>: Dimension,
    Sum<Th1, Th2>: Dimension,
    Sum<N1, N2>: Dimension,
    Sum<J1, J2>: Dimension,
{
    type Output = DimensionVector<
        Sum<M1, M2>,
        Sum<L1, L2>,
        Sum<T1, T2>,
        Sum<I1, I2>,
        Sum<Th1, Th2>,
        Sum<N1, N2>,
        Sum<J1, J2>,
    >;

    fn mul(self, _: DimensionVector<M2, L2, T2, I2, Th2, N2, J2>) -> Self::Output {
        Self::Output::default()
    }
}

// Division - subtracts dimensions at type level
impl<M1, L1, T1, I1, Th1, N1, J1, M2, L2, T2, I2, Th2, N2, J2>
    Div<DimensionVector<M2, L2, T2, I2, Th2, N2, J2>>
    for DimensionVector<M1, L1, T1, I1, Th1, N1, J1>
where
    M1: Dimension + Sub<M2>,
    L1: Dimension + Sub<L2>,
    T1: Dimension + Sub<T2>,
    I1: Dimension + Sub<I2>,
    Th1: Dimension + Sub<Th2>,
    N1: Dimension + Sub<N2>,
    J1: Dimension + Sub<J2>,
    M2: Dimension,
    L2: Dimension,
    T2: Dimension,
    I2: Dimension,
    Th2: Dimension,
    N2: Dimension,
    J2: Dimension,
    Diff<M1, M2>: Dimension,
    Diff<L1, L2>: Dimension,
    Diff<T1, T2>: Dimension,
    Diff<I1, I2>: Dimension,
    Diff<Th1, Th2>: Dimension,
    Diff<N1, N2>: Dimension,
    Diff<J1, J2>: Dimension,
{
    type Output = DimensionVector<
        Diff<M1, M2>,
        Diff<L1, L2>,
        Diff<T1, T2>,
        Diff<I1, I2>,
        Diff<Th1, Th2>,
        Diff<N1, N2>,
        Diff<J1, J2>,
    >;

    fn div(self, _: DimensionVector<M2, L2, T2, I2, Th2, N2, J2>) -> Self::Output {
        Self::Output::default()
    }
}

// Inverse quantity
impl<M, L, T, I, Th, N, J> Inv for DimensionVector<M, L, T, I, Th, N, J>
where
    M: Neg + Dimension,
    L: Neg + Dimension,
    T: Neg + Dimension,
    I: Neg + Dimension,
    Th: Neg + Dimension,
    N: Neg + Dimension,
    J: Neg + Dimension,
    Negate<M>: Dimension,
    Negate<L>: Dimension,
    Negate<T>: Dimension,
    Negate<I>: Dimension,
    Negate<Th>: Dimension,
    Negate<N>: Dimension,
    Negate<J>: Dimension,
{
    type Output = DimensionVector<
        Negate<M>,
        Negate<L>,
        Negate<T>,
        Negate<I>,
        Negate<Th>,
        Negate<N>,
        Negate<J>,
    >;

    fn inv(self) -> Self::Output {
        Self::Output::default()
    }
}

/// Exponentiation of quantity - multiplies dimensions by exponent
/// (New trait cause exponent is completely given by type, no need for runtime argument)
pub trait TypePow<Exp>
where
    Exp: Integer + NonZero + ToInt<i32>,
{
    type Output;
    fn pow(self) -> Self::Output;
}

impl<M, L, T, I, Th, N, J, Exp> TypePow<Exp> for DimensionVector<M, L, T, I, Th, N, J>
where
    M: Dimension + Mul<Exp>,
    L: Dimension + Mul<Exp>,
    T: Dimension + Mul<Exp>,
    I: Dimension + Mul<Exp>,
    Th: Dimension + Mul<Exp>,
    N: Dimension + Mul<Exp>,
    J: Dimension + Mul<Exp>,
    Exp: Integer + NonZero + ToInt<i32>,
    Prod<M, Exp>: Dimension,
    Prod<L, Exp>: Dimension,
    Prod<T, Exp>: Dimension,
    Prod<I, Exp>: Dimension,
    Prod<Th, Exp>: Dimension,
    Prod<N, Exp>: Dimension,
    Prod<J, Exp>: Dimension,
{
    type Output = DimensionVector<
        Prod<M, Exp>,
        Prod<L, Exp>,
        Prod<T, Exp>,
        Prod<I, Exp>,
        Prod<Th, Exp>,
        Prod<N, Exp>,
        Prod<J, Exp>,
    >;

    fn pow(self) -> Self::Output {
        Self::Output::default()
    }
}
