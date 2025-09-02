use std::fmt::Debug;

use crate::model::{dimension::Dimensioned, measure::Measure, quantity::QuantityMarker};

/// Top-Level definition of a Unit.
pub trait Unit: Debug + Clone + Copy + crate::sealed::Sealed {
    type Quantity: QuantityMarker + Dimensioned;
    const FACTOR: f64;
    const ABBREV: &'static str;

    /// Create a new measure from a raw value
    fn new(value: impl Into<f64>) -> Measure<Self> {
        Measure::new(value.into())
    }
}

impl<U> crate::sealed::Sealed for U where U: Unit {}

// ============================================
// MACROS
// ============================================

/// Macro to define a new unit
#[macro_export]
macro_rules! unit {
    // Base units (prefixable or not)
    (base: $unit_name:ident, $abbrev:literal, $quantity:ty; $($optionals:tt)* ) => {
        unit!(base_internal: $unit_name, $abbrev, $quantity, 1.0; $($optionals)*);
    };


    (base_internal: $unit_name:ident, $abbrev:literal, $quantity:ty, $factor:expr; prefixable $(, $($optionals:tt)*)?) => {
        unit!(base_internal: $unit_name, $abbrev, $quantity, $factor; $($($optionals)*)?);

        impl $crate::model::prefix::Prefixable for $unit_name {}
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

        impl $crate::model::prefix::Prefixable for $unit_name {}
    };

    (derived: $unit_name:ident, $abbrev:literal, ($factor:expr, $base_unit:ty)) => {
        $crate::__unit!(
            $unit_name,
            <$base_unit as $crate::model::unit::Unit>::Quantity,
            ($factor as f64) * <$base_unit as $crate::model::unit::Unit>::FACTOR,
            $abbrev
        );
    };

    // Compound unit (not prefixable by default?)
    (compound: $unit_name:ident, $abbrev:literal, [$($components:tt),+]; prefixable) => {
        unit!(compound: $unit_name, $abbrev, [$($components),+]);

        impl $crate::model::prefix::Prefixable for $unit_name {}
    };

    (compound: $unit_name:ident, $abbrev:literal, [$($components:tt),+] ) => {
        $crate::__compound_unit!(
            $unit_name,
            $abbrev,
            [
                $crate::model::quantity::Quantity<$crate::model::dimension::DimensionZero>,
                1.0;
                $($components),+
            ]
        );
    };

    // Prefixed unit
    (prefix: $alias:ident, $prefix:ty, $base_unit:ty) => {
        $crate::__unit!(
            $alias,
            <$base_unit as $crate::model::unit::Unit>::Quantity,
            <$prefix as $crate::model::prefix::Prefix>::FACTOR * <$base_unit as $crate::model::unit::Unit>::FACTOR,
            const_format::concatcp!(
                <$prefix as $crate::model::prefix::Prefix>::SYMBOL,
                <$base_unit as $crate::model::unit::Unit>::ABBREV
            )
        );
    };
}

/// Inner macros
#[doc(hidden)]
pub mod __inner_unit_macros {
    /// Create a unit struct and impl Unit trait
    #[macro_export]
    #[doc(hidden)]
    macro_rules! __unit {
        ($unit_name:ident, $quantity:ty, $factor:expr, $abbrev:expr) => {
            #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
            pub struct $unit_name;

            impl $crate::model::unit::Unit for $unit_name {
                type Quantity = $quantity;
                const FACTOR: f64 = $factor;
                const ABBREV: &'static str = $abbrev;
            }

            impl std::fmt::Display for $unit_name {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(f, "{}", <$unit_name as $crate::model::unit::Unit>::ABBREV)
                }
            }
        };
    }

    /// Create a compound unit
    #[macro_export]
    #[doc(hidden)]
    macro_rules! __compound_unit {
        // Base case
        ($unit_name:ident, $abbrev:literal, [$quantity:ty, $factor_acc:expr;] ) => {
            $crate::__unit!(
                $unit_name,
                $quantity,
                $factor_acc,
                $abbrev
            );
        };

        // Recursive cases
        ($unit_name:ident, $abbrev:literal, [$quantity:ty, $factor_acc:expr; ($unit:ty, $exp:ty) $(, $components:tt)*] ) => {
            $crate::__compound_unit!(
                $unit_name,
                $abbrev,
                [$quantity, $factor_acc; (1.0, $unit, $exp) $(, $components)*]
            );
        };


        ($unit_name:ident, $abbrev:literal, [$quantity:ty, $factor_acc:expr; ($scalar:expr, $unit:ty, $exp:ty) $(, $components:tt)*] ) => {
            $crate::__compound_unit!(
                $unit_name,
                $abbrev,
                [
                    <$quantity as std::ops::Mul<
                        <<$unit as $crate::model::unit::Unit>::Quantity as $crate::model::dimension::TypePow<$exp>>::Output
                    >>::Output,
                    $factor_acc * $crate::model::unit::__inner_unit_macros::__powi_const(
                        ($scalar as f64) * <$unit as $crate::model::unit::Unit>::FACTOR, <$exp as typenum::ToInt<i32>>::INT
                    );
                    $($components),*
                ]
            );
        };

        ($unit_name:ident, $abbrev:literal, [$quantity:ty, $factor_acc:expr; (constant $constant:expr, $unit:ty, $exp:ty) $(, $components:tt)*] ) => {
            $crate::__compound_unit!(
                $unit_name,
                $abbrev,
                [
                    <$quantity as std::ops::Mul<
                        <<$unit as $crate::model::unit::Unit>::Quantity as $crate::model::dimension::TypePow<$exp>>::Output
                    >>::Output,
                    $factor_acc * $crate::model::unit::__inner_unit_macros::__powi_const(
                        ($constant).value_const(), <$exp as typenum::ToInt<i32>>::INT
                    );
                    $($components),*
                ]
            );
        };

    }

    /// Const fn for integer exponentiation
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
        if neg {
            1.0 / acc
        } else {
            acc
        }
    }
}
