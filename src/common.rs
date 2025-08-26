/// Assert that two floating point values are almost equal within an epsilon
///
/// # Arguments
/// * `left` - The left value to compare
/// * `right` - The right value to compare
/// * `epsilon` - Optional epsilon for comparison (defaults to 1e-10)
///
/// # Examples
/// ```
/// ferrunitas::assert_almost_equal!(1.0, 1.0000000001);
/// ferrunitas::assert_almost_equal!(1.0, 1.1, 0.2, 0.1);
/// ```
#[macro_export]
macro_rules! assert_almost_equal {
    ($left:expr, $right:expr) => {
        $crate::assert_almost_equal!($left, $right, 0.1, 1e-4);
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

/// Generate two unit tests for a unit: quantity validation and value conversion
///
/// # Arguments
/// * `unit` - The unit type to test
/// * `quantity` - The expected quantity type
/// * `value` - The raw value to test conversion with
///
/// # Examples
/// ```
/// ferrunitas::verify_unit!(Meter, Length, 5.0);
/// ```
/// This generates:
/// - `test_meter_quantity()` - validates Meter has Length quantity
/// - `test_meter_value()` - validates Meter(5.0).into() == Length(5.0)
#[macro_export]
macro_rules! verify_unit {
    ($unit:ty, $quantity:ty, $value:expr) => {
        paste::paste! {
            #[test]
            fn [<test_ $unit:lower _ $quantity:lower _quantity>]() {
                use $crate::model::unit::{Unit};
                use std::any::TypeId;
                use $crate::format_quantity_dims;

                // Runtime type assertion - check that TypeIds are identical
                let unit_quantity_type_id = TypeId::of::<<$unit as Unit>::Quantity>();
                let expected_quantity_type_id = TypeId::of::<$quantity>();

                assert_eq!(
                    unit_quantity_type_id,
                    expected_quantity_type_id,
                    "Type mismatch: {} has quantity type ({}) that doesn't match expected {} ({})",
                    stringify!($unit),
                    format_quantity_dims!(<$unit as Unit>::Quantity),
                    stringify!($quantity),
                    format_quantity_dims!($quantity)
                );
            }

            #[test]
            fn [<test_ $unit:lower _ $quantity:lower _value>]() {
                use $crate::model::unit::Unit;
                use $crate::model::quantity::QuantityMarker;

                let unit = $unit::new(1.0);
                let quantity = unit.into_q();

                $crate::assert_almost_equal!(quantity.raw_value(), $value);
            }
        }
    };
}

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
