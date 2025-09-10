//! Dimensional type system primitives (exponent vectors & compile-time ops).
//! Most notably it contains the DimensionVector type, which represents a vector of the SI base dimensions.

use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Neg, Sub};

use num_traits::Inv;
use typenum::*;

/// Dimension exponent marker.
///
/// Implemented by typenum integers representing a single base dimension's
/// exponent (e.g. `P2`, `N1`, `Z0`). Provides conversion to a signed integer.
/// Required to be used with the dimensional vector.
pub trait Dimension: Debug + Clone + Copy + PartialOrd + PartialEq {
    fn to_int() -> i8;
}
impl<T: typenum::Integer + Debug + PartialOrd> Dimension for T {
    fn to_int() -> i8 {
        Self::to_i8()
    }
}

/// Implemented by types that carry a 7‑component dimensional signature.
///
/// Associated types correspond to exponents of base dimensions in order:
/// Mass (M), Length (L), Time (T), Electric Current (I), Temperature (Th),
/// Amount of Substance (N), Luminous Intensity (J).
pub trait Dimensioned: Debug + Clone + Copy + PartialEq + crate::model::sealed::Sealed {
    type M: Dimension;
    type L: Dimension;
    type T: Dimension;
    type I: Dimension;
    type Th: Dimension;
    type N: Dimension;
    type J: Dimension;

    fn to_array() -> [i8; 7] {
        [
            Self::M::to_int(),
            Self::L::to_int(),
            Self::T::to_int(),
            Self::I::to_int(),
            Self::Th::to_int(),
            Self::N::to_int(),
            Self::J::to_int(),
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
/// Concrete carrier of seven base dimension exponents.
///
/// Invariant: contains no runtime data; all information lives at the type
/// level. Construction is always zero-cost.
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

/// Convenience alias for a zero (all exponents 0) dimension signature.
pub type DimensionZero = DimensionVector<Z0, Z0, Z0, Z0, Z0, Z0, Z0>;

impl<M, L, T, I, Th, N, J> crate::model::sealed::Sealed for DimensionVector<M, L, T, I, Th, N, J>
where
    M: Dimension,
    L: Dimension,
    T: Dimension,
    I: Dimension,
    Th: Dimension,
    N: Dimension,
    J: Dimension,
{
}

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

// ======================
// ARITHMETIC
// ======================

/// Addition - only works for same dimensions
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

/// Subtraction - only works for same dimensions
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

/// Multiplication - adds dimensions at type level
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

/// Division - subtracts dimensions at type level
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

/// Inverse quantity - negates dimensions at type level
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
    /// Resulting dimension vector.
    type Output;
    /// Apply exponent of type level hence no value.
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

#[cfg(test)]
mod tests {
    use super::*;
    use typenum::{N1, N2, N3, P1, P2, P3, Z0};

    // Helper function to verify dimension vector exponents
    #[allow(clippy::too_many_arguments)]
    fn assert_dimensions<M, L, T, I, Th, N, J>(
        _: DimensionVector<M, L, T, I, Th, N, J>,
        expected_m: i8,
        expected_l: i8,
        expected_t: i8,
        expected_i: i8,
        expected_th: i8,
        expected_n: i8,
        expected_j: i8,
    ) where
        M: Dimension,
        L: Dimension,
        T: Dimension,
        I: Dimension,
        Th: Dimension,
        N: Dimension,
        J: Dimension,
    {
        assert_eq!(<M as Dimension>::to_int(), expected_m);
        assert_eq!(<L as Dimension>::to_int(), expected_l);
        assert_eq!(<T as Dimension>::to_int(), expected_t);
        assert_eq!(<I as Dimension>::to_int(), expected_i);
        assert_eq!(<Th as Dimension>::to_int(), expected_th);
        assert_eq!(<N as Dimension>::to_int(), expected_n);
        assert_eq!(<J as Dimension>::to_int(), expected_j);
    }

    #[test]
    fn test_dimension_trait() {
        // Test dimension exponent conversion to integers
        assert_eq!(<Z0 as Dimension>::to_int(), 0);
        assert_eq!(<P1 as Dimension>::to_int(), 1);
        assert_eq!(<P2 as Dimension>::to_int(), 2);
        assert_eq!(<N1 as Dimension>::to_int(), -1);
        assert_eq!(<N2 as Dimension>::to_int(), -2);
        assert_eq!(<P3 as Dimension>::to_int(), 3);
        assert_eq!(<N3 as Dimension>::to_int(), -3);
    }

    #[test]
    fn test_addition_same_dimensions() {
        // [M L T^-1 I^2 Th^-1 N^3 J^-2] + [M L T^-1 I^2 Th^-1 N^3 J^-2] = [M L T^-1 I^2 Th^-1 N^3 J^-2]
        let dim1 = DimensionVector::<P1, P1, N1, P2, N1, P3, N2>::default();
        let dim2 = DimensionVector::<P1, P1, N1, P2, N1, P3, N2>::default();
        let result = dim1 + dim2;
        assert_dimensions(result, 1, 1, -1, 2, -1, 3, -2);
    }

    #[test]
    fn test_subtraction_same_dimensions() {
        // [M^2 L^-3 T^1 I^-1 Th^2 N^-2 J^3] - [M^2 L^-3 T^1 I^-1 Th^2 N^-2 J^3] = [M^2 L^-3 T^1 I^-1 Th^2 N^-2 J^3]
        let dim1 = DimensionVector::<P2, N3, P1, N1, P2, N2, P3>::default();
        let dim2 = DimensionVector::<P2, N3, P1, N1, P2, N2, P3>::default();
        let result = dim1 - dim2;
        assert_dimensions(result, 2, -3, 1, -1, 2, -2, 3);
    }

    #[test]
    fn test_multiplication_adds_dimensions() {
        // [M^1 L^0 T^0 I^0 Th^0 N^0 J^0] × [L^1 T^-2 I^0 Th^0 N^0 J^0] = [M^1 L^1 T^-2 I^0 Th^0 N^0 J^0]
        let dim1 = DimensionVector::<P1, Z0, Z0, Z0, Z0, Z0, Z0>::default();
        let dim2 = DimensionVector::<Z0, P1, N2, Z0, Z0, Z0, Z0>::default();
        let result = dim1 * dim2;
        assert_dimensions(result, 1, 1, -2, 0, 0, 0, 0);

        // [M^2 L^-1 T^3 I^-2 Th^1 N^-3 J^2] × [M^-1 L^2 T^-1 I^1 Th^0 N^1 J^-1]
        // = [M^1 L^1 T^2 I^-1 Th^1 N^-2 J^1]
        let dim3 = DimensionVector::<P2, N1, P3, N2, P1, N3, P2>::default();
        let dim4 = DimensionVector::<N1, P2, N1, P1, Z0, P1, N1>::default();
        let result2 = dim3 * dim4;
        assert_dimensions(result2, 1, 1, 2, -1, 1, -2, 1);
    }

    #[test]
    fn test_division_subtracts_dimensions() {
        // [M^0 L^1 T^0 I^0 Th^0 N^0 J^0] ÷ [M^0 L^0 T^1 I^0 Th^0 N^0 J^0] = [M^0 L^1 T^-1 I^0 Th^0 N^0 J^0]
        let dim1 = DimensionVector::<Z0, P1, Z0, Z0, Z0, Z0, Z0>::default();
        let dim2 = DimensionVector::<Z0, Z0, P1, Z0, Z0, Z0, Z0>::default();
        let result = dim1 / dim2;
        assert_dimensions(result, 0, 1, -1, 0, 0, 0, 0);

        // [M^3 L^-2 T^1 I^2 Th^-1 N^0 J^3] ÷ [M^1 L^1 T^-1 I^-1 Th^2 N^-1 J^1]
        // = [M^2 L^-3 T^2 I^3 Th^-3 N^1 J^2]
        let dim3 = DimensionVector::<P3, N2, P1, P2, N1, Z0, P3>::default();
        let dim4 = DimensionVector::<P1, P1, N1, N1, P2, N1, P1>::default();
        let result2 = dim3 / dim4;
        assert_dimensions(result2, 2, -3, 2, 3, -3, 1, 2);
    }

    #[test]
    fn test_inverse_negates_dimensions() {
        // 1/[M^0 L^0 T^1 I^0 Th^0 N^0 J^0] = [M^0 L^0 T^-1 I^0 Th^0 N^0 J^0]
        let dim1 = DimensionVector::<Z0, Z0, P1, Z0, Z0, Z0, Z0>::default();
        let result = dim1.inv();
        assert_dimensions(result, 0, 0, -1, 0, 0, 0, 0);

        // 1/[M^2 L^-1 T^3 I^-2 Th^1 N^-3 J^2] = [M^-2 L^1 T^-3 I^2 Th^-1 N^3 J^-2]
        let dim2 = DimensionVector::<P2, N1, P3, N2, P1, N3, P2>::default();
        let result2 = dim2.inv();
        assert_dimensions(result2, -2, 1, -3, 2, -1, 3, -2);
    }

    #[test]
    fn test_exponentiation() {
        // [M^0 L^1 T^0 I^0 Th^0 N^0 J^0]^2 = [M^0 L^2 T^0 I^0 Th^0 N^0 J^0]
        let dim1 = DimensionVector::<Z0, P1, Z0, Z0, P2, N2, Z0>::default();
        let res1 = TypePow::<P2>::pow(dim1);
        assert_dimensions(res1, 0, 2, 0, 0, 4, -4, 0);

        // [M^0 L^1 T^0 I^0 Th^0 N^0 J^0]^3 = [M^0 L^3 T^0 I^0 Th^0 N^0 J^0]
        let dim2 = DimensionVector::<Z0, P1, N1, Z0, P2, P3, Z0>::default();
        let res2 = TypePow::<P3>::pow(dim2);
        assert_dimensions(res2, 0, 3, -3, 0, 6, 9, 0);

        // [M^0 L^0 T^1 I^0 Th^0 N^0 J^0]^(-1) = [M^0 L^0 T^-1 I^0 Th^0 N^0 J^0]
        let dim3 = DimensionVector::<Z0, Z0, P1, Z0, N2, Z0, N3>::default();
        let res3 = TypePow::<N1>::pow(dim3);
        assert_dimensions(res3, 0, 0, -1, 0, 2, 0, 3);

        // [M^1 L^-1 T^2 I^-1 Th^1 N^-1 J^1]^2 = [M^2 L^-2 T^4 I^-2 Th^2 N^-2 J^2]
        let dim4 = DimensionVector::<P1, N1, P2, N1, P1, N1, P1>::default();
        let res4 = TypePow::<P2>::pow(dim4);
        assert_dimensions(res4, 2, -2, 4, -2, 2, -2, 2);
    }

    #[test]
    fn test_dimensionless_operations() {
        // [1] + [1] = [1]
        let result_add = DimensionZero::default() + DimensionZero::default();
        assert_dimensions(result_add, 0, 0, 0, 0, 0, 0, 0);

        // [1] × [M L T^-1] = [M L T^-1]
        let dim1 = DimensionVector::<P1, P1, N1, N2, P2, P2, Z0>::default();
        let result_mul = DimensionZero::default() * dim1;
        assert_dimensions(result_mul, 1, 1, -1, -2, 2, 2, 0);

        // [M] ÷ [M] = [1] (ratio of same dimension)
        let dim2 = DimensionVector::<P1, N2, N4, Z0, Z0, P2, P4>::default();
        let dim3 = DimensionVector::<P1, N2, N4, Z0, Z0, P2, P4>::default();
        let ratio = dim2 / dim3;
        assert_dimensions(ratio, 0, 0, 0, 0, 0, 0, 0);
    }
}
