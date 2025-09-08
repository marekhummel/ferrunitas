//! Unit trait & macros for defining base, derived, compound and prefixed units.
//!
//! This module provides the foundation for the unit system in Ferrunitas. Units are
//! zero-sized marker types that carry compile-time information about their associated
//! quantity, conversion factor, and display symbol. The primary interface is the
//! [`unit!`] macro, which can create four different types of units:

use std::fmt::Debug;

use crate::model::{measure::Measure, quantity::QuantityMarker};

/// Core trait implemented by every concrete unit type.
///
/// The `Unit` trait is the foundation of the type system, providing the essential
/// information needed for dimensional analysis and unit conversions. Each unit is
/// a zero-sized marker type that carries compile-time information about its
/// associated quantity, conversion factor, and display symbol.
///
/// Units are typically created using the [`crate::unit!`] macro rather than implementing
/// this trait manually. The trait provides the scaffolding for type-safe arithmetic
/// and conversions between different units of the same dimension.
///
/// # Associated Types and Constants
///
/// - **`Quantity`**: The dimensional quantity this unit measures (e.g., `Length`, `Mass`)
/// - **`FACTOR`**: Conversion factor to canonical SI base units
/// - **`ABBREV`**: Short symbol for display (e.g., "m", "kg", "ft")
///
/// # Examples
///
/// ```
/// use ferrunitas::system::*;
/// use ferrunitas::{Measure, Unit};
///
/// // Units provide compile-time information
/// assert_eq!(Metre::FACTOR, 1.0);        // Base SI unit
/// assert_eq!(Metre::ABBREV, "m");
///
/// assert_eq!(Foot::FACTOR, 0.3048);      // Conversion to metres
/// assert_eq!(Foot::ABBREV, "ft");
///
/// assert_eq!(Kilometre::FACTOR, 1000.0); // 1000 metres per kilometre
/// assert_eq!(Kilometre::ABBREV, "km");
///
/// // Temperature scales are a bit special due to offsets
/// assert_eq!(DegreeCelsius::FACTOR, 1.0);
/// assert_eq!(DegreeCelsius::OFFSET, 273.15);
/// assert_eq!(DegreeCelsius::ABBREV, "°C");
///
/// // Create measures using the unit's new() method
/// let distance = Metre::new(100.0);
/// let height = Foot::new(6.0);
/// let long_distance = Kilometre::new(5.0);
///
/// // Units can be converted between compatible types via the Measure struct
/// let distance_in_feet: Measure<Foot> = distance.convert();
/// println!("100 metres = {:.2}", distance_in_feet);
/// ```
///
/// # Unit Arithmetic
///
/// Units participate in dimensional arithmetic through their associated measures:
///
/// ```
/// use ferrunitas::{system::*, Unit};
///
/// let length = Metre::new(10.0);
/// let time = Second::new(2.0);
///
/// // Division creates a new quantity with proper dimensions
/// let velocity: Velocity = length / time;  // 10 m ÷ 2 s = 5 m/s
///
/// let speed_kmh = velocity.as_measure::<KilometrePerHour>();
/// println!("Velocity: {}", speed_kmh);  // 18 km/h
/// ```
pub trait Unit: Debug + Clone + Copy + PartialEq + PartialOrd {
    /// The associated quantity type for this unit.
    type Quantity: QuantityMarker;
    /// The scaling factor to canonical base units.
    const FACTOR: f64;
    /// The offset to canonical base units (for affine units like Celsius)
    const OFFSET: f64;
    /// The abbreviation for this unit.
    const ABBREV: &'static str;

    /// Create a new measure from a raw value
    ///
    /// This is the primary constructor for creating [`Measure`] instances.
    /// It takes any value that can be converted to `f64` and wraps it in
    /// a measure with this unit type.
    ///
    /// # Examples
    ///
    /// ```
    /// use ferrunitas::{system::*, Unit};
    ///
    /// // Create measures with different numeric types
    /// let distance1 = Metre::new(100.0);   // f64
    /// let distance2 = Metre::new(100);     // integer
    /// let distance3 = Metre::new(100_u32); // u32
    ///
    /// // All create equivalent measures
    /// assert_eq!(distance1.value(), 100.0);
    /// assert_eq!(distance2.value(), 100.0);
    /// assert_eq!(distance3.value(), 100.0);
    ///
    /// // Works with any unit type
    /// let mass = Kilogram::new(5.5);
    /// let time = Second::new(10);
    /// let force = Newton::new(50.0);
    /// ```
    fn new(value: impl Into<f64>) -> Measure<Self> {
        Measure::new(value.into())
    }
}

// ============================================
// MACROS
// ============================================

/// Macro to define a new unit
///
/// This is the primary macro for creating unit types in the Ferrunitas library.
/// It supports four different patterns for unit definition, each suited to different
/// use cases. All generated units implement the [`Unit`] trait and can be used
/// with [`Measure`] for type-safe calculations.
///
/// # Unit Types
///
/// ## 1. Base Units (`base:`)
///
/// Define fundamental units for a quantity, typically with a factor of 1.0 for SI base units.
///
/// ```
/// use ferrunitas::{unit, quantity, typenum_consts::*};
///
/// quantity!(Length: M Z0, L P1, T Z0, I Z0, Th Z0, N Z0, J Z0);
/// quantity!(Mass: M P1, L Z0, T Z0, I Z0, Th Z0, N Z0, J Z0);
/// quantity!(Time: M Z0, L Z0, T P1, I Z0, Th Z0, N Z0, J Z0);
///
/// // SI base units
/// unit!(base: Metre, "m", Length; prefixable);
/// unit!(base: Gram, "g", Mass; prefixable, factor = 0.001); // Adjusts for kg base (anomaly in SI system)
///
/// // Non-prefixable base unit
/// unit!(base: Second, "s", Time);
/// ```
///
/// ## 2. Derived Units (`derived:`)
///
/// Create units as scaled versions of existing units using a simple multiplication factor.
///
/// ```
/// use ferrunitas::unit;
/// use ferrunitas::system::*;
///
/// // Imperial length units
/// unit!(derived: Inch, "in", (2.54, Centimetre));
/// unit!(derived: Foot, "ft", (12, Inch));
/// unit!(derived: Yard, "yd", (3, Foot));
/// unit!(derived: Mile, "mi", (1760, Yard));
///
/// // Mass units
/// unit!(derived: Pound, "lb", (453.592, Gram));
/// unit!(derived: Ounce, "oz", (28.3495, Gram));
///
/// // Some derived units can be prefixable
/// unit!(derived: Tonne, "t", (1000, Kilogram); prefixable);
///
/// // Temperatures use offsets, however this is rather unusual. Providing an offset will limit the
/// // unit from being used with prefixes, and it's strongly discouraged to use these as compound units,
/// // as the offsets are lost (so only differences in temperature are meaningful then).
/// unit!(derived: DegreeCelsius, "°C", (1.0, 273.15, Kelvin));
/// ```
///
/// ## 3. Compound Units (`compound:`)
///
/// Define units by combining multiple existing units with exponents, like m/s² or kg⋅m/s².
/// Avoids hard coding conversion factors if the underlying factors are already present.
/// E.g., since we know one kilometer equals 1000 meters and one hour equals 3600 seconds,
/// we can express km/h as a compound unit instead of defining it as 0.277778 m/s.
///
/// ```
/// use ferrunitas::{unit, typenum_consts::*};
/// use ferrunitas::system::*;
///
/// // Velocity: length/time
/// unit!(compound: MetrePerSecond, "m/s", [(Metre, P1), (Second, N1)]);
/// unit!(compound: KilometrePerHour, "km/h", [(Kilometre, P1), (Hour, N1)]);
///
/// // Acceleration: length/time²
/// unit!(compound: MetrePerSecondSquared, "m/s²", [(Metre, P1), (Second, N2)]);
///
/// // Force: mass × acceleration
/// unit!(compound: Newton, "N", [(Kilogram, P1), (Metre, P1), (Second, N2)]);
///
/// // With scalar factors
/// unit!(compound: YardsPerMinute, "yd/min", [(3, Foot, P1), (60, Second, N1)]);
///
/// // With quantity tagging (requires `quantity_tags` feature)
/// unit!(compound: BitPerSecond, "bit/s", [(Bit, P1), (Second, N1)]; marked DataRate);
/// unit!(compound: RadianPerSecond, "rad/s", [(Radian, P1), (Second, N1)]; marked AngularVelocity);
/// ```
///
/// ## 4. Prefixed Units (`prefix:`)
///
/// Create prefixed versions of existing units automatically.
///
/// ```
/// use ferrunitas::{unit, prefix};
/// use ferrunitas::system::*;
///
/// prefix!(Kilo, 1000, "k");
/// prefix!(Centi, 0.01, "c");
/// prefix!(Milli, 0.001, "m");
///
/// // These create Kilometre, Centimetre, Millimetre automatically
/// unit!(prefix: Kilometre, Kilo, Metre);
/// unit!(prefix: Centimetre, Centi, Metre);
/// unit!(prefix: Millimetre, Milli, Metre);
/// ```
///
/// # Options
///
/// - **`prefixable`**: Allows the unit to be used with prefixes
/// - **`factor = value`**: Override the conversion factor for base units
/// - **`marked QuantityType`**: (compound units only) Associate with a specific tagged quantity type (requires `quantity_tags` feature)
///
/// # Generated Code
///
/// Each `unit!` invocation creates:
/// - A zero-sized struct representing the unit
/// - Implementation of the [`Unit`] trait with proper constants
/// - Implementation of `Display` for the unit symbol
/// - Optional implementation of trait to mark it as compatible with prefixes if specified
///
/// # Examples
///
/// ## Complete Example
///
/// ```
/// use ferrunitas::{unit, prefix, quantity, typenum_consts::*, Measure, Unit};
///
/// // Define the quantity
/// quantity!(Length: M Z0, L P1, T Z0, I Z0, Th Z0, N Z0, J Z0);
///
/// // Define base unit
/// unit!(base: Metre, "m", Length; prefixable);
///
/// // Define prefixes
/// prefix!(Kilo, 1000, "k");
/// prefix!(Centi, 0.01, "c");
///
/// // Create prefixed units
/// unit!(prefix: Kilometre, Kilo, Metre);
/// unit!(prefix: Centimetre, Centi, Metre);
///
/// // Define derived units
/// unit!(derived: Foot, "ft", (0.3048, Metre));
/// unit!(derived: Inch, "in", (2.54, Centimetre));
///
/// // Usage
/// let distance = Metre::new(100.0);
/// let km_distance: Measure<Kilometre> = distance.convert();
/// let ft_distance: Measure<Foot> = distance.convert();
///
/// println!("Distance: {} = {} = {}", distance, km_distance, ft_distance);
/// ```
#[macro_export]
macro_rules! unit {
    // Base units (prefixable or not)
    (base: $unit_name:ident, $abbrev:literal, $quantity:ty $(; $($optionals:tt)*)? ) => {
        unit!(base_internal: $unit_name, $abbrev, $quantity, 1.0; $($($optionals)*)?);
    };


    (base_internal: $unit_name:ident, $abbrev:literal, $quantity:ty, $factor:expr; prefixable $(, $($optionals:tt)*)?) => {
        unit!(base_internal: $unit_name, $abbrev, $quantity, $factor; $($($optionals)*)?);

        impl $crate::__model::Prefixable for $unit_name {}
    };

    (base_internal: $unit_name:ident, $abbrev:literal, $quantity:ty, $factor:expr; factor = $new_factor:expr $(, $($optionals:tt)*)?) => {
        unit!(base_internal: $unit_name, $abbrev, $quantity, $new_factor; $($($optionals)*)?);
    };

    (base_internal: $unit_name:ident, $abbrev:literal, $quantity:ty, $factor:expr;) => {
        $crate::__unit!($unit_name, $quantity, $factor, $abbrev);
    };

    // Derived units based on other unit
    (derived: $unit_name:ident,  $abbrev:literal, ($factor:expr, $base_unit:ty); prefixable) => {
        unit!(derived: $unit_name, $abbrev, ($factor, $base_unit));

        impl $crate::__model::Prefixable for $unit_name {}
    };

    (derived: $unit_name:ident,  $abbrev:literal, ($factor:expr, $base_unit:ty)) => {
        unit!(derived: $unit_name, $abbrev, ($factor, 0.0, $base_unit));
    };

    (derived: $unit_name:ident, $abbrev:literal, ($factor:expr, $offset:expr, $base_unit:ty)) => {
        $crate::__unit!(
            $unit_name,
            <$base_unit as $crate::__model::Unit>::Quantity,
            ($factor as f64) * <$base_unit as $crate::__model::Unit>::FACTOR,
            <$base_unit as $crate::__model::Unit>::OFFSET + (<$base_unit as $crate::__model::Unit>::FACTOR * $offset),
            $abbrev
        );
    };

    // Compound unit
    (compound: $unit_name:ident, $abbrev:literal, [$($components:tt),+] $(; $($optionals:tt)*)? ) => {
        unit!(compound_internal: $unit_name, $abbrev, (), [$($components),+]; $($($optionals)*)?);
    };


    (compound_internal: $unit_name:ident, $abbrev:literal, $name_tag:ty, [$($components:tt),+]; prefixable $(, $($optionals:tt)*)?) => {
        unit!(compound_internal: $unit_name, $abbrev, $name_tag, [$($components),+]; $($($optionals)*)?);

        impl $crate::__model::Prefixable for $unit_name {}
    };

    (compound_internal: $unit_name:ident, $abbrev:literal, $name_tag:ty, [$($components:tt),+]; marked $quantity_for_tag:ty $(, $($optionals:tt)*)?) => {
        unit!(compound_internal:
            $unit_name,
            $abbrev,
            <$quantity_for_tag as $crate::__model::QuantityMarker>::Tag,
            [$($components),+];
            $($($optionals)*)?
        );
    };

    (compound_internal: $unit_name:ident, $abbrev:literal, $quantity_for_tag:ty, [$($components:tt),+];) => {
        $crate::__compound_unit!(
            $unit_name,
            $abbrev,
            $quantity_for_tag,
            [
                $crate::__model::Quantity<$crate::__model::DimensionZero, ()>,
                1.0;
                $($components),+
            ]
        );
    };

    // Prefixed unit
    (prefix: $alias:ident, $prefix:ty, $base_unit:ty) => {
        $crate::__unit!(
            $alias,
            <$base_unit as $crate::__model::Unit>::Quantity,
            <$prefix as $crate::__model::Prefix>::FACTOR * <$base_unit as $crate::__model::Unit>::FACTOR,
            const_format::concatcp!(
                <$prefix as $crate::__model::Prefix>::SYMBOL,
                <$base_unit as $crate::__model::Unit>::ABBREV
            )
        );
    };
}

/// Inner macros
///
/// This module contains implementation details for the [`unit!`] macro.
/// These macros are not intended for direct use and are hidden from the
/// public API. They handle the actual code generation for different unit types.
#[doc(hidden)]
pub mod __inner_unit_macros {
    /// Create a unit struct and impl Unit trait
    #[macro_export]
    #[doc(hidden)]
    macro_rules! __unit {
        // No offset
        ($unit_name:ident, $quantity:ty, $factor:expr, $abbrev:expr) => {
            $crate::__unit!($unit_name, $quantity, $factor, 0.0, $abbrev);
        };

        // Default impl with offset
        ($unit_name:ident, $quantity:ty, $factor:expr, $offset:expr, $abbrev:expr) => {
            #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
            /// Zero-sized marker struct representing a concrete unit.
            ///
            /// Generated by `unit!` macro; implements `Unit` with provided
            /// quantity mapping, factor and abbreviation.
            pub struct $unit_name;

            impl $crate::__model::Unit for $unit_name {
                type Quantity = $quantity;
                const FACTOR: f64 = $factor;
                const OFFSET: f64 = $offset;
                const ABBREV: &'static str = $abbrev;
            }

            impl std::fmt::Display for $unit_name {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(f, "{}", <$unit_name as $crate::__model::Unit>::ABBREV)
                }
            }

            $crate::__unit_times_number!($unit_name, [i32, f64]);
        };
    }

    /// Create a compound unit
    #[macro_export]
    #[doc(hidden)]
    macro_rules! __compound_unit {
        // Base case
        ($unit_name:ident, $abbrev:literal, $name_tag:ty, [$quantity:ty, $factor_acc:expr;] ) => {
            $crate::__unit!(
                $unit_name,
                $crate::__model::Quantity<
                    <$quantity as $crate::__model::QuantityMarker>::DimensionVector,
                    $name_tag
                >,
                $factor_acc,
                $abbrev
            );
        };

        // Recursive cases
        ($unit_name:ident, $abbrev:literal, $name_tag:ty, [$quantity:ty, $factor_acc:expr; ($unit:ty, $exp:ty) $(, $components:tt)*] ) => {
            $crate::__compound_unit!(
                $unit_name,
                $abbrev,
                $name_tag,
                [$quantity, $factor_acc; (1.0, $unit, $exp) $(, $components)*]
            );
        };

        ($unit_name:ident, $abbrev:literal, $name_tag:ty, [$quantity:ty, $factor_acc:expr; ($scalar:expr, $unit:ty, $exp:ty) $(, $components:tt)*] ) => {
            $crate::__compound_unit!(
                $unit_name,
                $abbrev,
                $name_tag,
                [
                    <$quantity as std::ops::Mul<
                        <<$unit as $crate::__model::Unit>::Quantity as $crate::__model::TypePow<$exp>>::Output
                    >>::Output,
                    $factor_acc * $crate::__model::__inner_unit_macros::__powi_const(
                        ($scalar as f64) * <$unit as $crate::__model::Unit>::FACTOR, <$exp as typenum::ToInt<i32>>::INT
                    );
                    $($components),*
                ]
            );
        };

        ($unit_name:ident, $abbrev:literal, $quantity_for_tag:ty, [$quantity:ty, $factor_acc:expr; (constant $constant:expr, $unit:ty, $exp:ty) $(, $components:tt)*] ) => {
            $crate::__compound_unit!(
                $unit_name,
                $abbrev,
                $quantity_for_tag,
                [
                    <$quantity as std::ops::Mul<
                        <<$unit as $crate::__model::Unit>::Quantity as $crate::__model::TypePow<$exp>>::Output
                    >>::Output,
                    $factor_acc * $crate::__model::__inner_unit_macros::__powi_const(
                        ($constant).value_const(), <$exp as typenum::ToInt<i32>>::INT
                    );
                    $($components),*
                ]
            );
        };
    }

    /// Implement multiplication of unit by number for QOL instantiation of measures.
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __unit_times_number {
        ($unit:ty, [$($numeric_type:ty),*]) => {
            $(
                impl std::ops::Mul<$unit> for $numeric_type {
                    type Output = $crate::Measure<$unit>;

                    fn  mul(self, _rhs: $unit) -> Self::Output {
                        <$unit as $crate::__model::Unit>::new(self)
                    }
                }
            )*
        };
    }

    /// Const fn for integer exponentiation
    ///
    /// Computes `base^exp` at compile time for integer exponents.
    /// Used internally by compound unit macros to calculate conversion factors
    /// when units are raised to various powers.
    ///
    /// This is a compile-time implementation of exponentiation that handles
    /// both positive and negative exponents correctly.
    #[doc(hidden)]
    pub const fn __powi_const(mut base: f64, mut exp: i32) -> f64 {
        if exp == 0 {
            return 1.0;
        }
        let neg = exp < 0;
        if neg {
            exp = -exp;
        }
        let mut e = exp as u32;
        let mut acc = 1.0;
        while e != 0 {
            if (e & 1) == 1 {
                acc *= base;
            }
            base *= base;
            e >>= 1;
        }
        if neg { 1.0 / acc } else { acc }
    }
}
