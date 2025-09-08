//! Unit-agnostic dimensioned (and optionally tagged) magnitude (`Quantity<D, T>`) for arithmetic & conversion.
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

/// Optional tag type implemented by quantities to distinguish between different
/// physical quantities with the same dimensions (e.g., Information (Bit), Angle (Radians) etc.).
/// Quantities use only the unit-type () for this, if the feature flag `quantity_tags` is not enabled.
pub trait QuantityTag: Debug + Clone + Copy + PartialOrd + PartialEq {}
impl QuantityTag for () {}

/// Dimensioned magnitude independent of a concrete display unit.
///
/// `Quantity` represents a physical quantity with its dimensional information encoded
/// at the type level, but without being tied to a specific unit. Values are stored
/// as raw `f64` in canonical SI base units. This makes `Quantity` ideal for dimensional
/// arithmetic and calculations where the specific display unit doesn't matter.
///
/// The 7-component dimensional signature is encoded in the type parameter `D`, representing
/// the seven SI base dimensions: Mass, Length, Time, Current, Temperature, Amount, Luminosity.
///
/// The type parameter `T` provides quantity tagging when the `quantity_tags` feature
/// is enabled, allowing you to distinguish between dimensionally identical but semantically
/// different quantities (e.g., `Information` bits vs `Angle` radians). Without the feature, this is always the unit type `()`.
///
/// # Key Features
///
/// - **Dimensional Safety**: Prevents adding incompatible quantities at compile time
/// - **Arithmetic Operations**: Supports +, -, *, / with proper dimensional analysis
/// - **Unit Flexibility**: Can be converted to/from any compatible unit via [`Measure`]
/// - **Generic Calculations**: Perfect for functions that work with any unit of a dimension
/// - **Zero Runtime Cost**: All dimensional checking happens at compile time
/// - **Quantity Tagging**: Optional type-level distinction between semantically different quantities
///
/// # Relationship with Measure
///
/// - [`Measure<Unit>`] = Value with specific unit (e.g., "5 meters", "10 kilograms")
/// - [`Quantity<Dimension>`] = Value with dimension only (e.g., "length", "mass")
/// - Convert between them using [`as_measure()`] and [`into_q()`]
///
/// # Examples
///
/// ```
/// use ferrunitas::system::*;
/// use ferrunitas::Unit;
///
/// // Create quantities from measures
/// let length: Length = Metre::new(100.0).into_q();
/// let time: Time = Second::new(10.0).into_q();
///
/// // Dimensional arithmetic creates new quantity types
/// let velocity: Velocity = length / time;  // Length ÷ Time = Velocity
/// let area: Area = length * length;        // Length × Length = Area
///
/// // Convert back to specific units for display
/// let speed_mps = velocity.as_measure::<MetrePerSecond>();
/// let speed_mph = velocity.as_measure::<MilePerHour>();
/// let area_sqm = area.as_measure::<SquareMetre>();
///
/// println!("Speed: {} or {}", speed_mps, speed_mph);
/// println!("Area: {}", area_sqm);
///
/// // Quantities can be added/subtracted (same dimensions only)
/// let more_length = length + Length::from::<Foot>(10.0);
///
/// // Display shows dimensional information
/// println!("Velocity: {}", velocity);  // Shows value with [L T^-1] dimensions
///
/// // Quantity tag example (with `quantity_tags` feature enabled)
/// let data_bits = Bit::new(8.0).into_q();      // Information quantity
/// let angle_rads = Radian::new(1.0).into_q();  // Angle quantity
///
/// #[cfg(feature = "quantity_tags")]
/// {
///     // Explicitly convert to the same semantic type:
///     let combined = data_bits.specify::<Dimensionless>() +
///                    angle_rads.specify::<Dimensionless>();
/// }
///
/// #[cfg(not(feature = "quantity_tags"))]
/// {
///     // Without quantity tags, we can add them directly:
///     let combined = data_bits + angle_rads;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Quantity<D: Dimensioned, T: QuantityTag> {
    /// This value holds the raw f64 value of the quantity, only meant for internal use
    pub(crate) value: f64,
    _phantom: PhantomData<(D, T)>,
}

/// Internal marker implemented by all quantity instantiations to gate trait
/// implementations outside the crate (sealed via `sealed::Sealed`).
#[doc(hidden)]
pub trait QuantityMarker:
    Sized + Debug + Clone + Copy + PartialEq + crate::model::sealed::Sealed
{
    type DimensionVector: Dimensioned;
    type Tag: QuantityTag;

    fn new(value: f64) -> Self;
    fn raw_value(&self) -> f64;
}

// ===========================
// MACRO
// ===========================

/// Define a new quantity either by a list of 7 dimensions or by compounding others.
///
/// This macro creates type aliases for [`Quantity`] with specific dimensional signatures.
/// You can define quantities in three ways:
///
/// 1. **By explicit dimensions**: Specify the 7 SI base dimensions directly
/// 2. **By explicit dimensions with tagging**: Same as above, but creates a unique quantity tag (if `quantity_tags` feature is enabled)
/// 3. **By compounding existing quantities**: Combine existing quantity types with exponents
///
/// The 7 base dimensions are: **M**ass, **L**ength, **T**ime, **I**ntensity of current, **Th**ermodynamic temperature, **N**umerus (amount of substance), **J** (luminous intensity).
///
/// # Examples
///
/// ## Explicit Dimensions
///
/// ```
/// use ferrunitas::{quantity, typenum_consts::*};
///
/// // Define basic quantities with explicit dimensional signatures
/// quantity!(Length: M Z0, L P1, T Z0, I Z0, Th Z0, N Z0, J Z0);  // [L]
/// quantity!(Time:   M Z0, L Z0, T P1, I Z0, Th Z0, N Z0, J Z0);  // [T]
/// quantity!(Mass:   M P1, L Z0, T Z0, I Z0, Th Z0, N Z0, J Z0);  // [M]
///
/// // More complex quantities
/// quantity!(Force:  M P1, L P1, T N2, I Z0, Th Z0, N Z0, J Z0);  // [M L T^-2]
/// ```
///
/// ## Explicit Dimensions with Quantity Tagging
///
/// When the `quantity_tags` feature is enabled, you can create quantities with unique tags
/// to distinguish between dimensionally identical but semantically different quantities:
///
/// ```rust
/// use ferrunitas::{quantity, typenum_consts::*};
///
/// // These are all dimensionless but represent different concepts
/// quantity!(Angle:       M Z0, L Z0, T Z0, I Z0, Th Z0, N Z0, J Z0; marked);   // [1] (angles)
/// quantity!(Information: M Z0, L Z0, T Z0, I Z0, Th Z0, N Z0, J Z0; marked);   // [1] (data)
/// quantity!(SolidAngle:  M Z0, L Z0, T Z0, I Z0, Th Z0, N Z0, J Z0; marked);   // [1] (solid angles)
///
/// // With quantity tagging, these become distinct types even though they're dimensionally identical
/// ```
///
/// ## Compounding Existing Quantities
///
/// ```
/// use ferrunitas::{quantity, typenum_consts::*};
/// use ferrunitas::system::*;  // Get predefined quantities
///
/// // Define velocity as Length / Time
/// quantity!(Velocity: [(Length, P1), (Time, N1)]);  // [L T^-1]
///
/// // Define acceleration as Velocity / Time or Length / Time²
/// quantity!(Acceleration: [(Velocity, P1), (Time, N1)]);  // [L T^-2]
///
/// // Define area as Length²
/// quantity!(Area: [(Length, P2)]);  // [L^2]
///
/// // Define density as Mass / Volume (Volume = Length³)
/// quantity!(Volume: [(Length, P3)]);   // [L^3]
/// quantity!(Density: [(Mass, P1), (Volume, N1)]);  // [M L^-3]
/// ```
///
/// # Dimension Exponents
///
/// Use these typenum constants from `ferrunitas::typenum_consts`:
/// - `Z0` = Zero (dimension not present)
/// - `P1`, `P2`, `P3`, ... = Positive exponents (+1, +2, +3, ...)
/// - `N1`, `N2`, `N3`, ... = Negative exponents (-1, -2, -3, ...)
///
/// # Quantity Tagging
///
/// The `marked` keyword creates a unique quantity tag when the `quantity_tags` feature is enabled:
/// - **With feature enabled**: Creates distinct types for each marked quantity, preventing mixing
/// - **With feature disabled**: All quantities of the same dimension are interchangeable
///
/// This helps catch semantic errors at compile time, like accidentally adding data bits to angle radians.
#[macro_export]
macro_rules! quantity {
    // Literal case
    ($quantity:ident: M $mass:ty, L $length:ty, T $time:ty, I $current:ty, Th $temperature:ty, N $amount:ty, J $luminosity:ty; marked) => {
        paste::paste! {
            /// Auto-generated quantity marker type produced by `quantity!` macro for named quantities.
            #[cfg(feature = "quantity_tags")]
            #[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
            pub struct [<$quantity Tag>] {}

            #[cfg(feature = "quantity_tags")]
            impl $crate::__model::QuantityTag for [<$quantity Tag>] {}

            #[cfg(feature = "quantity_tags")]
            quantity!(
                $quantity:
                $crate::__model::Quantity<
                    $crate::__model::DimensionVector<$mass, $length, $time, $current, $temperature, $amount, $luminosity>,
                    [<$quantity Tag>]
                >;
            );

            #[cfg(not(feature = "quantity_tags"))]
            quantity!(
                $quantity:
                $crate::__model::Quantity<
                    $crate::__model::DimensionVector<$mass, $length, $time, $current, $temperature, $amount, $luminosity>,
                    ()
                >;
            );
        }
    };

    ($quantity:ident: M $mass:ty, L $length:ty, T $time:ty, I $current:ty, Th $temperature:ty, N $amount:ty, J $luminosity:ty) => {
        quantity!(
            $quantity:
            $crate::__model::Quantity<
                $crate::__model::DimensionVector<
                    $mass, $length, $time, $current, $temperature, $amount, $luminosity
                >,
                ()
            >;
        );
    };

    // External compound case
    ($comp_quantity:ident: [$(($quantities:ty, $exps:ty)),+]) => {
        quantity!(
            $comp_quantity:
            $crate::__model::Quantity<$crate::__model::DimensionZero, ()>; $(($quantities, $exps)),+
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
        /// Auto-generated quantity type alias produced by `quantity!` macro.
        pub type $comp_quantity = $quantity_acc;
    };
}

// ===========================
// IMPLS
// ===========================

/// Default impl
impl<D: Dimensioned, T: QuantityTag> Quantity<D, T> {
    /// Create a new quantity from a canonical base value (crate internal).
    pub(crate) fn new(value: f64) -> Self {
        Self {
            value,
            _phantom: PhantomData,
        }
    }

    /// Construct this quantity from a numeric value expressed in unit `U`.
    ///
    /// Creates a quantity by taking a numeric value in a specific unit and converting
    /// it to the internal SI base representation. This is equivalent to creating a
    /// [`Measure`] and then calling [`into_q()`] on it.
    ///
    /// # Examples
    ///
    /// ```
    /// use ferrunitas::system::*;
    ///
    /// // Create length quantities from different units
    /// let length1 = Length::from::<Metre>(100.0);
    /// let length2 = Length::from::<Foot>(328.084);
    /// let length3 = Length::from::<Kilometre>(0.1);
    ///
    /// // All represent approximately the same physical quantity
    /// assert!((length1.as_measure::<Metre>().value() - 100.0).abs() < 0.001);
    /// assert!((length2.as_measure::<Metre>().value() - 100.0).abs() < 0.001);
    /// assert!((length3.as_measure::<Metre>().value() - 100.0).abs() < 0.001);
    ///
    /// // Works with any unit of the same dimension
    /// let mass = Mass::from::<Kilogram>(5.0);
    /// let same_mass = Mass::from::<Pound>(11.023);
    /// ```
    pub fn from<U: Unit<Quantity = Self>>(value: impl Into<f64>) -> Self {
        Measure::<U>::new(value.into()).into_q()
    }

    /// Represent this quantity as a `Measure<U>`.
    ///
    /// Converts this quantity to a measure with a specific unit. The internal SI base
    /// value is converted to the target unit's scale. This is the primary way to get
    /// a displayable value from a quantity.
    ///
    /// # Examples
    ///
    /// ```
    /// use ferrunitas::{system::*, Unit};
    ///
    /// // Create a velocity quantity from dimensional arithmetic
    /// let distance: Length = Metre::new(1000.0).into_q();
    /// let time: Time = Second::new(50.0).into_q();
    /// let velocity: Velocity = distance / time;  // 20 m/s
    ///
    /// // Convert to different velocity units
    /// let mps = velocity.as_measure::<MetrePerSecond>();
    /// let kmh = velocity.as_measure::<KilometrePerHour>();
    /// let mph = velocity.as_measure::<MilePerHour>();
    ///
    /// assert_eq!(mps.value(), 20.0);
    /// assert!((kmh.value() - 72.0).abs() < 0.01);  // 20 m/s = 72 km/h
    /// assert!((mph.value() - 44.74).abs() < 0.01); // 20 m/s ≈ 44.74 mph
    /// ```
    pub fn as_measure<U: Unit<Quantity = Self>>(&self) -> Measure<U> {
        Measure::from_q(*self)
    }

    /// Convenience: Convert numeric `value` from unit `U1` to unit `U2` for this quantity type.
    ///
    /// A one-line utility function that creates a measure in unit `U1`, converts it to
    /// this quantity type, then converts it to a measure in unit `U2`. Useful for
    /// quick unit conversions without intermediate variables.
    ///
    /// # Examples
    ///
    /// ```
    /// use ferrunitas::system::*;
    ///
    /// // Convert between length units
    /// let feet = Length::convert::<Metre, Foot>(100.0);
    /// let miles = Length::convert::<Kilometre, Mile>(10.0);
    /// let inches = Length::convert::<Centimetre, Inch>(254.0);
    ///
    /// assert!((feet.value() - 328.084).abs() < 0.001);    // 100 m ≈ 328.084 ft
    /// assert!((miles.value() - 6.214).abs() < 0.001);     // 10 km ≈ 6.214 mi
    /// assert!((inches.value() - 100.0).abs() < 0.1);      // 254 cm = 100 in
    ///
    /// // Convert between mass units
    /// let pounds = Mass::convert::<Kilogram, Pound>(1.0);
    /// assert!((pounds.value() - 2.205).abs() < 0.01);     // 1 kg ≈ 2.205 lb
    ///
    /// // Convert between time units
    /// let minutes = Time::convert::<Second, Minute>(3600.0);
    /// assert_eq!(minutes.value(), 60.0);                  // 3600 s = 60 min
    /// ```
    pub fn convert<U1, U2>(value: f64) -> Measure<U2>
    where
        U1: Unit<Quantity = Self>,
        U2: Unit<Quantity = Self>,
    {
        U1::new(value).convert::<U2>()
    }

    /// Convert this quantity to a different quantity tag with the same dimensions.
    ///
    /// This method is only available when the `quantity_tags` feature is enabled.
    /// It allows you to explicitly convert between quantities that have the same
    /// dimensional signature but different semantic meaning (e.g., from `Angle` to
    /// `Information` since both are dimensionless).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ferrunitas::system::*;
    /// use ferrunitas::Unit;
    ///
    /// // Both are dimensionless but have different tags
    /// let data_bits = Bit::new(8.0).into_q();       // Information
    /// let angle_rads = Radian::new(1.0).into_q();   // Angle
    ///
    /// let combined = data_bits.specify::<Dimensionless>() + angle_rads.specify::<Dimensionless>();
    /// ```
    #[cfg(feature = "quantity_tags")]
    pub fn specify<Q2>(self) -> Quantity<D, Q2::Tag>
    where
        Q2: QuantityMarker<DimensionVector = D>,
    {
        Quantity::new(self.value)
    }
}

/// Make sure quantity is sealed for QuantityMarker trait
impl<D: Dimensioned, T: QuantityTag> crate::model::sealed::Sealed for Quantity<D, T> {}

impl<D: Dimensioned, T: QuantityTag> QuantityMarker for Quantity<D, T> {
    type DimensionVector = D;
    type Tag = T;

    fn new(value: f64) -> Self {
        Self::new(value)
    }

    fn raw_value(&self) -> f64 {
        self.value
    }
}

/// Display of quantity
impl<D: Dimensioned, T: QuantityTag> fmt::Display for Quantity<D, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        std::fmt::Display::fmt(&self.value, f)?;

        let dim_string = crate::common::format_dims::<Self>();
        write!(f, " [{}]", dim_string)?;

        let mut tag_name = std::any::type_name::<T>();
        if tag_name != "()" {
            tag_name = tag_name.rsplit_once("::").unwrap().1;
            tag_name = tag_name.strip_suffix("Tag").unwrap();
            write!(f, " as {}", tag_name)?;
        }
        Ok(())
    }
}

// ===========================
// Generic Arithmetic Operations
// ===========================

/// Addition - only works for same dimensions
impl<D, T> Add<Self> for Quantity<D, T>
where
    D: Dimensioned + Add,
    T: QuantityTag,
    <D as Add>::Output: Dimensioned,
{
    type Output = Quantity<<D as Add>::Output, T>;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.raw_value() + rhs.raw_value())
    }
}

/// Addition - with a measure of same quantity
impl<D, T, U> Add<Measure<U>> for Quantity<D, T>
where
    D: Dimensioned + Add,
    T: QuantityTag,
    U: Unit<Quantity = Self>,
    <D as Add>::Output: Dimensioned,
{
    type Output = Quantity<<D as Add>::Output, T>;

    fn add(self, rhs: Measure<U>) -> Self::Output {
        Self::Output::new(self.raw_value() + rhs.into_q().raw_value())
    }
}

/// Assigned Addition - only works for same dimensions
impl<D: Dimensioned, T: QuantityTag> AddAssign<Self> for Quantity<D, T> {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.raw_value();
    }
}

/// Assigned Addition - with a measure of same quantity
impl<D: Dimensioned, T: QuantityTag, U> AddAssign<Measure<U>> for Quantity<D, T>
where
    U: Unit<Quantity = Self>,
{
    fn add_assign(&mut self, rhs: Measure<U>) {
        *self += rhs.into_q();
    }
}

/// Subtraction - only works for same dimensions
impl<D: Dimensioned, T: QuantityTag> Sub<Self> for Quantity<D, T>
where
    D: Sub,
    <D as Sub>::Output: Dimensioned,
{
    type Output = Quantity<<D as Sub>::Output, T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.raw_value() - rhs.raw_value())
    }
}

/// Subtraction - with a measure of same quantity
impl<D, T, U> Sub<Measure<U>> for Quantity<D, T>
where
    D: Dimensioned + Sub,
    T: QuantityTag,
    U: Unit<Quantity = Self>,
    <D as Sub>::Output: Dimensioned,
{
    type Output = Quantity<<D as Sub>::Output, T>;

    fn sub(self, rhs: Measure<U>) -> Self::Output {
        self - rhs.into_q()
    }
}

/// Assigned Subtraction - only works for same dimensions
impl<D: Dimensioned, T: QuantityTag> SubAssign<Self> for Quantity<D, T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.value -= rhs.raw_value();
    }
}

/// Assigned Subtraction - with a measure of same quantity
impl<D: Dimensioned, T: QuantityTag, U> SubAssign<Measure<U>> for Quantity<D, T>
where
    U: Unit<Quantity = Self>,
{
    fn sub_assign(&mut self, rhs: Measure<U>) {
        *self -= rhs.into_q();
    }
}

/// Multiplication - adds dimensions at type level
impl<D1: Dimensioned, D2: Dimensioned, T1: QuantityTag, T2: QuantityTag> Mul<Quantity<D2, T2>>
    for Quantity<D1, T1>
where
    D1: Mul<D2>,
    <D1 as Mul<D2>>::Output: Dimensioned,
{
    type Output = Quantity<<D1 as Mul<D2>>::Output, ()>;

    fn mul(self, rhs: Quantity<D2, T2>) -> Self::Output {
        Self::Output::new(self.raw_value() * rhs.raw_value())
    }
}

/// Multiplication - with another unit
impl<D1: Dimensioned, D2: Dimensioned, T1: QuantityTag, T2: QuantityTag, U> Mul<Measure<U>>
    for Quantity<D1, T1>
where
    U: Unit<Quantity = Quantity<D2, T2>>,
    D1: Mul<D2>,
    <D1 as Mul<D2>>::Output: Dimensioned,
{
    type Output = Quantity<<D1 as Mul<D2>>::Output, ()>;

    fn mul(self, rhs: Measure<U>) -> Self::Output {
        self * rhs.into_q()
    }
}

/// Division - subtracts dimensions at type level
impl<D1: Dimensioned, D2: Dimensioned, T1: QuantityTag, T2: QuantityTag> Div<Quantity<D2, T2>>
    for Quantity<D1, T1>
where
    D1: Div<D2>,
    <D1 as Div<D2>>::Output: Dimensioned,
{
    type Output = Quantity<<D1 as Div<D2>>::Output, ()>;

    fn div(self, rhs: Quantity<D2, T2>) -> Self::Output {
        Self::Output::new(self.raw_value() / rhs.raw_value())
    }
}

/// Division - with another unit
impl<D1: Dimensioned, D2: Dimensioned, T1: QuantityTag, T2: QuantityTag, U> Div<Measure<U>>
    for Quantity<D1, T1>
where
    U: Unit<Quantity = Quantity<D2, T2>>,
    D1: Div<D2>,
    <D1 as Div<D2>>::Output: Dimensioned,
{
    type Output = Quantity<<D1 as Div<D2>>::Output, ()>;

    fn div(self, rhs: Measure<U>) -> Self::Output {
        self / rhs.into_q()
    }
}

/// RHS Scalar multiplication - scales the value but keeps dimensions
impl<D: Dimensioned, T: QuantityTag> Mul<f64> for Quantity<D, T> {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
        Self::new(self.value * scalar)
    }
}

/// RHS Assigned scalar multiplication - scales the value but keeps dimensions
impl<D: Dimensioned, T: QuantityTag> MulAssign<f64> for Quantity<D, T> {
    fn mul_assign(&mut self, scalar: f64) {
        self.value *= scalar;
    }
}

/// LHS Scalar multiplication - scales the value but keeps dimensions
impl<D: Dimensioned, T: QuantityTag> Mul<Quantity<D, T>> for f64 {
    type Output = Quantity<D, T>;

    fn mul(self, quantity: Quantity<D, T>) -> Self::Output {
        Self::Output::new(self * quantity.value)
    }
}

/// RHS Scalar division - scales the value but keeps dimensions
impl<D: Dimensioned, T: QuantityTag> Div<f64> for Quantity<D, T> {
    type Output = Self;

    fn div(self, scalar: f64) -> Self::Output {
        Self::new(self.value / scalar)
    }
}

/// RHS Assigned scalar division - scales the value but keeps dimensions
impl<D: Dimensioned, T: QuantityTag> DivAssign<f64> for Quantity<D, T> {
    fn div_assign(&mut self, scalar: f64) {
        self.value /= scalar;
    }
}

// LHS Scalar division - Scale and inverse at the same time
impl<D: Dimensioned, T: QuantityTag> Div<Quantity<D, T>> for f64
where
    D: Inv,
    <D as Inv>::Output: Dimensioned,
{
    type Output = Quantity<<D as Inv>::Output, ()>;

    fn div(self, quantity: Quantity<D, T>) -> Self::Output {
        Self::Output::new(self / quantity.raw_value())
    }
}

/// Exponentiation - raises quantity to a type-level integer power, required for compound units
impl<D: Dimensioned, T: QuantityTag, Exp> TypePow<Exp> for Quantity<D, T>
where
    D: TypePow<Exp>,
    <D as TypePow<Exp>>::Output: Dimensioned,
    Exp: Integer + NonZero + ToInt<i32>,
{
    type Output = Quantity<<D as TypePow<Exp>>::Output, ()>;

    fn pow(self) -> Self::Output {
        Self::Output::new(self.value.powi(Exp::INT))
    }
}
