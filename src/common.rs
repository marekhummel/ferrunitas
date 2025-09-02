//! Common helpers & internal testing utilities.
//!
//! Provides formatting helpers for dimensional signatures plus internal test
//! macros (`verify_unit!`, `assert_almost_equal!`) used across unit definition
//! files. Not typically needed by downstream users directly.

use crate::model::{
    dimension::{Dimension, Dimensioned},
    unit::Unit,
};

/// Format the underlying quantity dimensions for a given unit type.
pub fn format_unit_dims<U: Unit>() -> String {
    format_dims::<<U as Unit>::Quantity>()
}

/// Format the dimension signature of a `Dimensioned` type (like quantities) as a human-readable string.
pub fn format_dims<D: Dimensioned>() -> String {
    let items: [(&str, i8); 7] = [
        ("M", D::M::to_int()),
        ("L", D::L::to_int()),
        ("T", D::T::to_int()),
        ("I", D::I::to_int()),
        ("Θ", D::Th::to_int()),
        ("N", D::N::to_int()),
        ("J", D::J::to_int()),
    ];

    let mut dim_string = String::new();
    for (dim, exp) in items {
        if exp == 0 {
            continue;
        }
        dim_string.push_str(&format!("{}^{}·", dim, exp));
    }
    if !dim_string.is_empty() {
        dim_string.pop();
    }

    dim_string
}

/// Generate two unit tests for a unit: quantity validation and value conversion
#[cfg(test)]
macro_rules! verify_unit {
    ($unit:ty, $quantity:ty, $value:expr) => {
        paste::paste! {
            #[test]
            fn [<test_ $unit:lower _ $quantity:lower _quantity>]() {
                use $crate::model::unit::{Unit};
                use std::any::TypeId;

                // Runtime type assertion - check that TypeIds are identical
                let unit_quantity_type_id = TypeId::of::<<$unit as Unit>::Quantity>();
                let expected_quantity_type_id = TypeId::of::<$quantity>();

                assert_eq!(
                    unit_quantity_type_id,
                    expected_quantity_type_id,
                    "Type mismatch: {} has quantity type ({}) that doesn't match expected {} ({})",
                    stringify!($unit),
                    $crate::common::format_dims::<<$unit as Unit>::Quantity>(),
                    stringify!($quantity),
                    $crate::common::format_dims::<$quantity>()
                );
            }

            #[test]
            fn [<test_ $unit:lower _ $quantity:lower _value>]() {
                let measure = <$unit as $crate::model::unit::Unit>::new(1.0);
                $crate::common::assert_almost_equal!(measure.into_q().value, $value);
            }
        }
    };
}

/// Assert that two floating point values are almost equal within an epsilon
#[cfg(test)]
macro_rules! assert_almost_equal {
    ($left:expr, $right:expr) => {
        $crate::common::assert_almost_equal!($left, $right, 0.1, 1e-4);
    };
    ($left:expr, $right:expr, $epsilon_abs:expr, $epsilon_rel:expr) => {
        let left_val: f64 = $left;
        let right_val: f64 = $right;
        let diff = (left_val - right_val).abs();

        assert!(
            diff < $epsilon_abs,
            "abs assertion failed: `(left ≈ right)`\n  left: `{}`\n right: `{}`\n difference: `{}` (epsilon: `{}`)",
            left_val, right_val, diff, $epsilon_abs
        );

        let max_val = left_val.abs().max(right_val.abs());
        let relative_epsilon = ($epsilon_rel as f64) * max_val;
        assert!(
            diff <= relative_epsilon,
            "rel assertion failed: `(left ≈ right)`\n  left: `{}`\n right: `{}`\n difference: `{}` (relative epsilon: `{}`, max value: `{}`)",
            left_val, right_val, diff, relative_epsilon, max_val
        );
    };
}

#[cfg(test)]
pub(crate) use assert_almost_equal;

#[cfg(test)]
pub(crate) use verify_unit;

#[cfg(test)]
mod tests {
    #[test]
    fn test_assert_almost_equal_default_epsilon() {
        assert_almost_equal!(1.0, 1.0);
        assert_almost_equal!(1.0, 1.0000000001);
    }

    #[test]
    fn test_assert_almost_equal_custom_epsilon() {
        assert_almost_equal!(1.0, 1.1, 0.2, 1);
        assert_almost_equal!(5.0, 5.05, 0.1, 1);
    }

    #[test]
    #[should_panic]
    fn test_assert_almost_equal_should_panic() {
        assert_almost_equal!(1.0, 2.0, 0.1, 1e-7);
    }
}
