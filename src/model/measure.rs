//! Strongly typed value bound to a specific unit (`Measure<U>` runtime wrapper).
//! Main way to interact with unit-coupled values, and all arithmetic operations are implemented here.

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::model::dimension::Dimensioned;
use crate::model::quantity::{Quantity, QuantityMarker, QuantityTag};
use crate::model::unit::Unit;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
/// Runtime holder of a numeric value tagged with a concrete unit type `U`.
///
/// `Measure` represents a physical quantity with a specific unit (like "5 metres" or "10 kilograms").
/// It stores the value in the unit's own scale and provides type-safe operations and conversions.
///
/// Use `Measure` when you need to work with specific units and want explicit unit semantics.
/// For generic dimensional arithmetic, consider using [`Quantity`] instead.
///
/// # Key Features
///
/// - **Type Safety**: Prevents mixing incompatible units at compile time
/// - **Unit Conversions**: Convert between compatible units with zero runtime cost
/// - **Arithmetic Operations**: Add, subtract, multiply, and divide with proper dimensional checking
/// - **Display**: Automatically formats with appropriate unit symbols
///
/// # Examples
///
/// ```
/// use ferrunitas::system::*;
/// use ferrunitas::{Measure, Unit};
///
/// // Create measures with specific units
/// let length = Metre::new(100.0);
/// let time = Second::new(10.0);
///
/// // Convert between compatible units
/// let feet: Measure<Foot> = length.convert();
/// let minutes: Measure<Minute> = time.convert();
///
/// // Arithmetic operations work across compatible units
/// let longer_length = length + Centimetre::new(50.0);  // 100 m + 50 cm = 100.5 m
///
/// // Multiplication/division creates quantities with proper dimensions
/// let velocity: Velocity = length / time;  // 100 m / 10 s = 10 m/s
///
/// // Display includes unit symbols
/// println!("Distance: {}", length);  // "100 m"
/// ```
pub struct Measure<U: Unit> {
    value: f64,
    _phantom: std::marker::PhantomData<U>,
}

impl<U: Unit> Measure<U> {
    /// Create a new measure from a value expressed in unit `U`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ferrunitas::system::*;
    /// use ferrunitas::{Measure, Unit};
    ///
    /// let distance = Metre::new(100.5);
    /// assert_eq!(distance.value(), 100.5);
    ///
    /// let mass = Kilogram::new(5);  // Works with any type that implements Into<f64>
    /// assert_eq!(mass.value(), 5.0);
    /// ```
    pub fn new(value: impl Into<f64>) -> Self {
        Self {
            value: value.into(),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Raw numeric value in unit `U` (no conversion).
    ///
    /// This returns the value as stored internally, in the specific unit this measure represents.
    /// No unit conversion is performed.
    ///
    /// # Examples
    ///
    /// ```
    /// use ferrunitas::{system::*, Unit};
    ///
    /// let distance = Metre::new(42.5);
    /// assert_eq!(distance.value(), 42.5);
    ///
    /// let feet = Foot::new(10.0);
    /// assert_eq!(feet.value(), 10.0);  // Still 10.0, not converted to metres
    /// ```
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Construct from a quantity of the same dimension.
    ///
    /// Takes a dimensioned quantity and converts it to a measure with this specific unit.
    /// The quantity's raw SI value is divided by the unit's conversion factor and shifted by the unit's offset, if present.
    ///
    /// # Examples
    ///
    /// ```
    /// use ferrunitas::system::*;
    /// use ferrunitas::{Measure, Unit};
    ///
    /// // Create a length quantity (internally stored in SI base units)
    /// let length: Length = Metre::new(100.0).into_q();
    ///
    /// // Convert to different unit measures
    /// let metres = Measure::<Metre>::from_q(length);
    /// let feet = Measure::<Foot>::from_q(length);
    ///
    /// assert_eq!(metres.value(), 100.0);
    /// assert!((feet.value() - 328.084).abs() < 0.001);  // ~328 feet
    /// ```
    pub fn from_q(q: U::Quantity) -> Self {
        Self::new((q.raw_value() - U::OFFSET) / U::FACTOR)
    }
    /// Convert into a dimensioned quantity using `U`'s factor.
    ///
    /// Transforms this measure into a generic quantity of the same dimension.
    /// The value is multiplied by the unit's conversion factor (and shifted by the unit's offset) to get the SI base unit value.
    /// Quantities are useful for dimensional arithmetic and generic calculations.
    ///
    /// # Examples
    ///
    /// ```
    /// use ferrunitas::{system::*, Unit};
    ///
    /// let distance = Foot::new(10.0);
    /// let length_quantity: Length = distance.into_q();
    ///
    /// let time = Second::new(5.0);
    /// let time_quantity: Time = time.into_q();
    ///
    /// // Now we can do dimensional arithmetic
    /// let velocity: Velocity = length_quantity / time_quantity;
    /// let speed = velocity.as_measure::<MetrePerSecond>();
    ///
    /// assert!((speed.value() - 0.6096).abs() < 0.0001);  // 10 ft / 5 s ≈ 0.6096 m/s
    /// ```
    pub fn into_q(self) -> U::Quantity {
        U::Quantity::new(self.value * U::FACTOR + U::OFFSET)
    }

    /// Convert to another unit of the same dimension.
    ///
    /// Performs a unit conversion between compatible units (same physical dimension).
    /// The conversion goes through the quantity representation: measure → quantity → measure.
    ///
    /// # Examples
    ///
    /// ```
    /// use ferrunitas::system::*;
    /// use ferrunitas::{Measure, Unit};
    ///
    /// let metres = Metre::new(1000.0);
    /// let kilometres: Measure<Kilometre> = metres.convert();
    /// let feet: Measure<Foot> = metres.convert();
    ///
    /// assert_eq!(kilometres.value(), 1.0);  // 1000 m = 1 km
    /// assert!((feet.value() - 3280.84).abs() < 0.01);  // 1000 m ≈ 3280.84 ft
    ///
    /// // Type inference can determine the target unit
    /// let inches: Measure<Inch> = metres.convert();
    /// assert!((inches.value() - 39370.1).abs() < 0.1);
    /// ```
    pub fn convert<UOther>(&self) -> Measure<UOther>
    where
        UOther: Unit<Quantity = U::Quantity>,
    {
        Measure::from_q(self.into_q())
    }

    /// Equality check across different units of same dimension.
    ///
    /// Compares two measures by converting both to their underlying quantities
    /// and checking if they represent the same physical amount, regardless of units.
    /// Typical PartialEq requires type equality, this extends the equality
    /// check to measures of the same quantity.
    ///
    /// # Examples
    ///
    /// ```
    /// use ferrunitas::{system::*, Unit};
    ///
    /// let metre = Metre::new(1.0);
    /// let centimetres = Centimetre::new(100.0);
    /// let kilometre = Kilometre::new(0.001);
    ///
    /// assert!(metre.is_equal_to(&centimetres));  // 1 m = 100 cm
    /// assert!(metre.is_equal_to(&kilometre));    // 1 m = 0.001 km
    ///
    /// // Regular == only works for same unit types
    /// assert_eq!(metre, Metre::new(1.0));
    /// // assert_eq!(metre, centimetres);  // This would not compile
    /// ```
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

/// Addition - works on different units but requires same quantity
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

/// Addition - with another quantity
impl<U, D, T> Add<Quantity<D, T>> for Measure<U>
where
    U: Unit<Quantity = Quantity<D, T>>,
    D: Dimensioned + Add<Output = D>,
    T: QuantityTag,
{
    type Output = Self;

    fn add(self, rhs: Quantity<D, T>) -> Self::Output {
        Self::from_q(self.into_q() + rhs)
    }
}

/// Assigned Addition - works on different units but requires same quantity
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

/// Assigned Addition - with another quantity
impl<U, D, T> AddAssign<Quantity<D, T>> for Measure<U>
where
    U: Unit<Quantity = Quantity<D, T>>,
    D: Dimensioned + Add<Output = D>,
    T: QuantityTag,
{
    fn add_assign(&mut self, rhs: Quantity<D, T>) {
        self.value += rhs.as_measure::<U>().value();
    }
}

/// Subtraction - works on different units but requires same quantity
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

/// Subtraction - with another quantity
impl<U, D, T> Sub<Quantity<D, T>> for Measure<U>
where
    U: Unit<Quantity = Quantity<D, T>>,
    D: Dimensioned + Sub<Output = D>,
    T: QuantityTag,
{
    type Output = Self;

    fn sub(self, rhs: Quantity<D, T>) -> Self::Output {
        Self::from_q(self.into_q() - rhs)
    }
}

/// Assigned Subtraction - works on different units but requires same quantity
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

/// Assigned Subtraction - with another quantity
impl<U, D, T> SubAssign<Quantity<D, T>> for Measure<U>
where
    U: Unit<Quantity = Quantity<D, T>>,
    D: Dimensioned + Sub<Output = D>,
    T: QuantityTag,
{
    fn sub_assign(&mut self, rhs: Quantity<D, T>) {
        self.value -= rhs.as_measure::<U>().value();
    }
}

/// Multiplication - just with unit assumes measure of 1.0
impl<U1: Unit, U2: Unit> Mul<U2> for Measure<U1>
where
    U1::Quantity: Mul<U2::Quantity>,
{
    type Output = <U1::Quantity as Mul<U2::Quantity>>::Output;

    fn mul(self, _rhs: U2) -> Self::Output {
        self.into_q() * U2::new(1.0).into_q()
    }
}

/// Multiplication - works with any unit but result is quantity
impl<U1: Unit, U2: Unit> Mul<Measure<U2>> for Measure<U1>
where
    U1::Quantity: Mul<U2::Quantity>,
{
    type Output = <U1::Quantity as Mul<U2::Quantity>>::Output;

    fn mul(self, rhs: Measure<U2>) -> Self::Output {
        self.into_q() * rhs.into_q()
    }
}

/// Multiplication - with another quantity
impl<U, D1, D2, T1, T2> Mul<Quantity<D2, T2>> for Measure<U>
where
    U: Unit<Quantity = Quantity<D1, T1>>,
    D1: Dimensioned + Mul<D2>,
    D2: Dimensioned,
    T1: QuantityTag,
    T2: QuantityTag,
    <D1 as Mul<D2>>::Output: Dimensioned,
{
    type Output = <Quantity<D1, T1> as Mul<Quantity<D2, T2>>>::Output;

    fn mul(self, rhs: Quantity<D2, T2>) -> Self::Output {
        self.into_q() * rhs
    }
}

/// Division - just with unit assumes measure of 1.0
impl<U1: Unit, U2: Unit> Div<U2> for Measure<U1>
where
    U1::Quantity: Div<U2::Quantity>,
{
    type Output = <U1::Quantity as Div<U2::Quantity>>::Output;

    fn div(self, _rhs: U2) -> Self::Output {
        self.into_q() / U2::new(1.0).into_q()
    }
}

/// Division - works with any unit but result is quantity
impl<U1: Unit, U2: Unit> Div<Measure<U2>> for Measure<U1>
where
    U1::Quantity: Div<U2::Quantity>,
{
    type Output = <U1::Quantity as Div<U2::Quantity>>::Output;

    fn div(self, rhs: Measure<U2>) -> Self::Output {
        self.into_q() / rhs.into_q()
    }
}

/// Division - with another quantity
impl<U, D1, D2, T1, T2> Div<Quantity<D2, T2>> for Measure<U>
where
    U: Unit<Quantity = Quantity<D1, T1>>,
    D1: Dimensioned + Div<D2>,
    D2: Dimensioned,
    T1: QuantityTag,
    T2: QuantityTag,
    <D1 as Div<D2>>::Output: Dimensioned,
{
    type Output = <Quantity<D1, T1> as Div<Quantity<D2, T2>>>::Output;

    fn div(self, rhs: Quantity<D2, T2>) -> Self::Output {
        self.into_q() / rhs
    }
}

/// RHS Scalar multiplication - no effect on unit or quantity
impl<U: Unit> Mul<f64> for Measure<U> {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
        Self::new(self.value * scalar)
    }
}

/// LHS Scalar multiplication - no effect on unit or quantity
impl<U: Unit> Mul<Measure<U>> for f64 {
    type Output = Measure<U>;

    fn mul(self, rhs: Measure<U>) -> Self::Output {
        Measure::new(self * rhs.value())
    }
}

/// RHS Assigned scalar multiplication - no effect on unit or quantity
impl<U: Unit> MulAssign<f64> for Measure<U> {
    fn mul_assign(&mut self, scalar: f64) {
        self.value *= scalar;
    }
}

/// RHS Scalar division - no effect on unit or quantity
impl<U: Unit> Div<f64> for Measure<U> {
    type Output = Self;

    fn div(self, scalar: f64) -> Self::Output {
        Self::new(self.value / scalar)
    }
}

/// LHS Scalar division - has to invert quantity, so result cant be unit
impl<U: Unit> Div<Measure<U>> for f64
where
    f64: Div<U::Quantity>,
{
    type Output = <f64 as Div<U::Quantity>>::Output;

    fn div(self, rhs: Measure<U>) -> Self::Output {
        self / rhs.into_q()
    }
}

/// RHS Assigned scalar division - no effect on unit or quantity
impl<U: Unit> DivAssign<f64> for Measure<U> {
    fn div_assign(&mut self, scalar: f64) {
        self.value /= scalar;
    }
}
