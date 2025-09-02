use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::model::dimension::Dimensioned;
use crate::model::quantity::{Quantity, QuantityMarker};
use crate::model::unit::Unit;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Measure<U: Unit> {
    value: f64,
    _phantom: std::marker::PhantomData<U>,
}

impl<U: Unit> Measure<U> {
    pub fn new(value: impl Into<f64>) -> Self {
        Self {
            value: value.into(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn from_q(q: U::Quantity) -> Self {
        Self::new(q.raw_value() / U::FACTOR)
    }
    pub fn into_q(self) -> U::Quantity {
        U::Quantity::new(self.value * U::FACTOR)
    }

    pub fn convert<UOther>(&self) -> Measure<UOther>
    where
        UOther: Unit<Quantity = U::Quantity>,
    {
        Measure::from_q(self.into_q())
    }

    pub fn is_equal_to<UOther>(&self, other: &Measure<UOther>) -> bool
    where
        UOther: Unit<Quantity = U::Quantity>,
    {
        self.into_q() == other.into_q()
    }

    // --------------------

    pub(crate) const fn new_const(value: f64) -> Self {
        Self {
            value,
            _phantom: std::marker::PhantomData,
        }
    }

    pub(crate) const fn value_const(&self) -> f64 {
        self.value
    }
}

impl<U: Unit> std::fmt::Display for Measure<U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.value, f)?;
        write!(f, " {}", U::ABBREV)
    }
}

// ---------------------------------------
// Operations
// ---------------------------------------

impl<U1, U2> Add<Measure<U2>> for Measure<U1>
where
    U1: Unit,
    U2: Unit<Quantity = U1::Quantity>,
    U1::Quantity: Add<U2::Quantity, Output = U1::Quantity>,
{
    type Output = Self;

    fn add(self, rhs: Measure<U2>) -> Self::Output {
        Self::from_q(self.into_q() + rhs.into_q())
    }
}

impl<U, D> Add<Quantity<D>> for Measure<U>
where
    U: Unit<Quantity = Quantity<D>>,
    D: Dimensioned + Add<Output = D>,
{
    type Output = Self;

    fn add(self, rhs: Quantity<D>) -> Self::Output {
        Self::from_q(self.into_q() + rhs)
    }
}

impl<U1, U2> AddAssign<Measure<U2>> for Measure<U1>
where
    U1: Unit,
    U2: Unit<Quantity = U1::Quantity>,
    U1::Quantity: Add<U2::Quantity, Output = U1::Quantity>,
{
    fn add_assign(&mut self, rhs: Measure<U2>) {
        self.value += rhs.convert::<U1>().value();
    }
}

impl<U, D> AddAssign<Quantity<D>> for Measure<U>
where
    U: Unit<Quantity = Quantity<D>>,
    D: Dimensioned + Add<Output = D>,
{
    fn add_assign(&mut self, rhs: Quantity<D>) {
        self.value += rhs.as_measure::<U>().value();
    }
}

impl<U1, U2> Sub<Measure<U2>> for Measure<U1>
where
    U1: Unit,
    U2: Unit<Quantity = U1::Quantity>,
    U1::Quantity: Sub<U2::Quantity, Output = U1::Quantity>,
{
    type Output = Self;

    fn sub(self, rhs: Measure<U2>) -> Self::Output {
        Self::from_q(self.into_q() - rhs.into_q())
    }
}

impl<U, D> Sub<Quantity<D>> for Measure<U>
where
    U: Unit<Quantity = Quantity<D>>,
    D: Dimensioned + Sub<Output = D>,
{
    type Output = Self;

    fn sub(self, rhs: Quantity<D>) -> Self::Output {
        Self::from_q(self.into_q() - rhs)
    }
}

impl<U1, U2> SubAssign<Measure<U2>> for Measure<U1>
where
    U1: Unit,
    U2: Unit<Quantity = U1::Quantity>,
    U1::Quantity: Sub<U2::Quantity, Output = U1::Quantity>,
{
    fn sub_assign(&mut self, rhs: Measure<U2>) {
        self.value -= rhs.convert::<U1>().value();
    }
}

impl<U, D> SubAssign<Quantity<D>> for Measure<U>
where
    U: Unit<Quantity = Quantity<D>>,
    D: Dimensioned + Sub<Output = D>,
{
    fn sub_assign(&mut self, rhs: Quantity<D>) {
        self.value -= rhs.as_measure::<U>().value();
    }
}

impl<U1: Unit, U2: Unit> Mul<Measure<U2>> for Measure<U1>
where
    U1::Quantity: Mul<U2::Quantity>,
{
    type Output = <U1::Quantity as Mul<U2::Quantity>>::Output;

    fn mul(self, rhs: Measure<U2>) -> Self::Output {
        self.into_q() * rhs.into_q()
    }
}

impl<U, D1, D2> Mul<Quantity<D2>> for Measure<U>
where
    U: Unit<Quantity = Quantity<D1>>,
    D1: Dimensioned + Mul<D2>,
    D2: Dimensioned,
    <D1 as Mul<D2>>::Output: Dimensioned,
{
    type Output = <Quantity<D1> as Mul<Quantity<D2>>>::Output;

    fn mul(self, rhs: Quantity<D2>) -> Self::Output {
        self.into_q() * rhs
    }
}

impl<U1: Unit, U2: Unit> Div<Measure<U2>> for Measure<U1>
where
    U1::Quantity: Div<U2::Quantity>,
{
    type Output = <U1::Quantity as Div<U2::Quantity>>::Output;

    fn div(self, rhs: Measure<U2>) -> Self::Output {
        self.into_q() / rhs.into_q()
    }
}

impl<U, D1, D2> Div<Quantity<D2>> for Measure<U>
where
    U: Unit<Quantity = Quantity<D1>>,
    D1: Dimensioned + Div<D2>,
    D2: Dimensioned,
    <D1 as Div<D2>>::Output: Dimensioned,
{
    type Output = <Quantity<D1> as Div<Quantity<D2>>>::Output;

    fn div(self, rhs: Quantity<D2>) -> Self::Output {
        self.into_q() / rhs
    }
}

impl<U: Unit> Mul<f64> for Measure<U> {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
        Self::new(self.value * scalar)
    }
}

impl<U: Unit> Mul<Measure<U>> for f64 {
    type Output = Measure<U>;

    fn mul(self, rhs: Measure<U>) -> Self::Output {
        Measure::new(self * rhs.value())
    }
}

impl<U: Unit> MulAssign<f64> for Measure<U> {
    fn mul_assign(&mut self, scalar: f64) {
        self.value *= scalar;
    }
}

impl<U: Unit> Div<f64> for Measure<U> {
    type Output = Self;

    fn div(self, scalar: f64) -> Self::Output {
        Self::new(self.value / scalar)
    }
}

impl<U: Unit> Div<Measure<U>> for f64
where
    f64: Div<U::Quantity>,
{
    type Output = <f64 as Div<U::Quantity>>::Output;

    fn div(self, rhs: Measure<U>) -> Self::Output {
        self / rhs.into_q()
    }
}

impl<U: Unit> DivAssign<f64> for Measure<U> {
    fn div_assign(&mut self, scalar: f64) {
        self.value /= scalar;
    }
}
