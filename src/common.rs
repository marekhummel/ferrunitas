//! Common helpers & internal testing utilities.
//!
//! Provides formatting helpers for dimensional signatures plus internal test
//! macros (`verify_unit!`, `assert_almost_equal!`) used across unit definition
//! files. Not typically needed by downstream users directly.

use crate::model::{
    dimension::{Dimension, Dimensioned},
    quantity::QuantityMarker,
    unit::Unit,
};

/// Format the underlying quantity dimensions for a given unit type.
pub fn format_unit_dims<U: Unit>() -> String {
    format_dims::<<U as Unit>::Quantity>()
}

/// Format the dimension signature of a `Dimensioned` type (like quantities) as a human-readable string.
pub fn format_dims<Q: QuantityMarker>() -> String {
    let items: [(&str, i8); 7] = [
        ("M", <Q::DimensionVector as Dimensioned>::M::to_int()),
        ("L", <Q::DimensionVector as Dimensioned>::L::to_int()),
        ("T", <Q::DimensionVector as Dimensioned>::T::to_int()),
        ("I", <Q::DimensionVector as Dimensioned>::I::to_int()),
        ("Θ", <Q::DimensionVector as Dimensioned>::Th::to_int()),
        ("N", <Q::DimensionVector as Dimensioned>::N::to_int()),
        ("J", <Q::DimensionVector as Dimensioned>::J::to_int()),
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
    } else {
        dim_string.push('1');
    }

    dim_string
}

/// Generate two unit tests for a unit: quantity validation and value conversion
#[cfg(test)]
macro_rules! verify_unit {
    // Default
    ($unit:ty, $quantity:ty, $value:expr) => {
        paste::paste! {
            #[test]
            fn [<test_ $unit:lower _ $quantity:lower _value>]() {
                assert_eq!(<$unit as $crate::model::unit::Unit>::OFFSET, 0.0, "Offset is set, use respective branch");

                let measure = <$unit as $crate::model::unit::Unit>::new(1.0);
                $crate::common::assert_almost_equal!(measure.into_q().value, $value, "Value");
            }
        }

        verify_unit!($unit, $quantity);
    };

    // With offset
    ($unit:ty, $quantity:ty; offset_tests [$(($base_val:expr, $unit_val:expr)),* $(,)?]) => {
        paste::paste! {
            #[test]
            fn [<test_ $unit:lower _ $quantity:lower _value_offset>]() {
                use $crate::model::measure::Measure;
                assert_ne!(<$unit as $crate::model::unit::Unit>::OFFSET, 0.0, "Offset is not set, use respective branch");

                $(
                    // Test Base -> Unit conversion
                    let base = <$quantity as $crate::model::quantity::QuantityMarker>::new($base_val);
                    let unit_measure: Measure<$unit> = base.as_measure();
                    $crate::common::assert_almost_equal!(unit_measure.value(), $unit_val, "Base to Unit");

                    // Test Unit -> Kelvin conversion
                    let unit_measure = <$unit as $crate::model::unit::Unit>::new($unit_val);
                    let base = unit_measure.into_q();
                    $crate::common::assert_almost_equal!(base.value, $base_val, "Unit to Base");
                )*
            }
        }

        verify_unit!($unit, $quantity);
    };

    // Just quantity
    ($unit:ty, $quantity:ty) => {
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
        }
    }
}

/// Assert that two floating point values are almost equal within an epsilon
#[cfg(test)]
macro_rules! assert_almost_equal {
    ($left:expr, $right:expr, $desc:literal) => {
        $crate::common::assert_almost_equal!($left, $right, 1e-5, 1e-8, $desc);
    };
    ($left:expr, $right:expr, $epsilon_abs:expr, $epsilon_rel:expr, $desc:literal) => {
        let left_val: f64 = $left;
        let right_val: f64 = $right;
        let diff = (left_val - right_val).abs();
        let max_val = left_val.abs().max(right_val.abs());
        let epsilon = $epsilon_abs + ($epsilon_rel as f64) * max_val;

        assert!(
            diff <= epsilon,
            "{}: Assertion failed: `(left ≈ right)`\n  left: `{}`\n right: `{}`\n difference: `{}` (total epsilon: `{}`)",
            $desc, left_val, right_val, diff, epsilon
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
        assert_almost_equal!(1.0, 1.0, "Test 1");
        assert_almost_equal!(1.0, 1.0000000001, "Test 2");
    }

    #[test]
    fn test_assert_almost_equal_custom_epsilon() {
        assert_almost_equal!(1.0, 1.1, 0.2, 1, "Test 3");
        assert_almost_equal!(5.0, 5.05, 0.1, 1, "Test 4");
    }

    #[test]
    #[should_panic]
    fn test_assert_almost_equal_should_panic() {
        assert_almost_equal!(1.0, 2.0, 0.1, 1e-7, "Test 5");
    }
}
