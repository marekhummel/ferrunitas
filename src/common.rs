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
    use super::*;
    use crate::model::dimension::*;
    use crate::model::quantity::Quantity;
    use typenum::*;

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

    #[test]
    fn test_format_dims_single_dimensions() {
        // Test all 7 individual dimensions with positive exponents

        // Mass: M^1
        type Mass = Quantity<DimensionVector<P1, Z0, Z0, Z0, Z0, Z0, Z0>, ()>;
        assert_eq!(format_dims::<Mass>(), "M^1");

        // Length: L^1
        type Length = Quantity<DimensionVector<Z0, P1, Z0, Z0, Z0, Z0, Z0>, ()>;
        assert_eq!(format_dims::<Length>(), "L^1");

        // Time: T^1
        type Time = Quantity<DimensionVector<Z0, Z0, P1, Z0, Z0, Z0, Z0>, ()>;
        assert_eq!(format_dims::<Time>(), "T^1");

        // Electric Current: I^1
        type ElectricCurrent = Quantity<DimensionVector<Z0, Z0, Z0, P1, Z0, Z0, Z0>, ()>;
        assert_eq!(format_dims::<ElectricCurrent>(), "I^1");

        // Thermodynamic Temperature: Θ^1
        type Temperature = Quantity<DimensionVector<Z0, Z0, Z0, Z0, P1, Z0, Z0>, ()>;
        assert_eq!(format_dims::<Temperature>(), "Θ^1");

        // Amount of Substance: N^1
        type AmountOfSubstance = Quantity<DimensionVector<Z0, Z0, Z0, Z0, Z0, P1, Z0>, ()>;
        assert_eq!(format_dims::<AmountOfSubstance>(), "N^1");

        // Luminous Intensity: J^1
        type LuminousIntensity = Quantity<DimensionVector<Z0, Z0, Z0, Z0, Z0, Z0, P1>, ()>;
        assert_eq!(format_dims::<LuminousIntensity>(), "J^1");
    }

    #[test]
    fn test_format_dims_different_exponents() {
        // Test single dimensions with negative exponents

        // I^-1 (Inverse Electric Current)
        type InverseCurrent = Quantity<DimensionVector<Z0, Z0, Z0, N1, Z0, Z0, Z0>, ()>;
        assert_eq!(format_dims::<InverseCurrent>(), "I^-1");

        // Θ^-2 (Inverse Temperature Squared)
        type InverseTempSquared = Quantity<DimensionVector<Z0, Z0, Z0, Z0, N2, Z0, Z0>, ()>;
        assert_eq!(format_dims::<InverseTempSquared>(), "Θ^-2");

        // N^-3 (Inverse Amount of Substance Cubed)
        type InverseSubstanceCubed = Quantity<DimensionVector<Z0, Z0, Z0, Z0, Z0, N3, Z0>, ()>;
        assert_eq!(format_dims::<InverseSubstanceCubed>(), "N^-3");

        // I^2 (Electric Current Squared)
        type CurrentSquared = Quantity<DimensionVector<Z0, Z0, Z0, P2, Z0, Z0, Z0>, ()>;
        assert_eq!(format_dims::<CurrentSquared>(), "I^2");

        // Θ^3 (Temperature Cubed)
        type TemperatureCubed = Quantity<DimensionVector<Z0, Z0, Z0, Z0, P3, Z0, Z0>, ()>;
        assert_eq!(format_dims::<TemperatureCubed>(), "Θ^3");

        // J^4 (Luminous Intensity to Fourth Power)
        type IntensityFourthPower = Quantity<DimensionVector<Z0, Z0, Z0, Z0, Z0, Z0, P4>, ()>;
        assert_eq!(format_dims::<IntensityFourthPower>(), "J^4");
    }

    #[test]
    fn test_format_dims_compound_dimensions() {
        // Test combinations of multiple dimensions

        // Electric Resistance: M^1·L^2·T^-3·I^-2 (Ohm)
        type ElectricResistance = Quantity<DimensionVector<P1, P2, N3, N2, Z0, Z0, Z0>, ()>;
        assert_eq!(format_dims::<ElectricResistance>(), "M^1·L^2·T^-3·I^-2");

        // Heat Capacity: M^1·L^2·T^-2·Θ^-1
        type HeatCapacity = Quantity<DimensionVector<P1, P2, N2, Z0, N1, Z0, Z0>, ()>;
        assert_eq!(format_dims::<HeatCapacity>(), "M^1·L^2·T^-2·Θ^-1");

        // Molar Volume: L^3·N^-1
        type MolarVolume = Quantity<DimensionVector<Z0, P3, Z0, Z0, Z0, N1, Z0>, ()>;
        assert_eq!(format_dims::<MolarVolume>(), "L^3·N^-1");

        // Luminous Flux: J^1 (already covered in single dims, but shows steradian context)
        type LuminousFlux = Quantity<DimensionVector<Z0, Z0, Z0, Z0, Z0, Z0, P1>, ()>;
        assert_eq!(format_dims::<LuminousFlux>(), "J^1");

        // Electric Field: M^1·L^1·T^-3·I^-1
        type ElectricField = Quantity<DimensionVector<P1, P1, N3, N1, Z0, Z0, Z0>, ()>;
        assert_eq!(format_dims::<ElectricField>(), "M^1·L^1·T^-3·I^-1");

        // Magnetic Field: M^1·T^-2·I^-1
        type MagneticField = Quantity<DimensionVector<P1, Z0, N2, N1, Z0, Z0, Z0>, ()>;
        assert_eq!(format_dims::<MagneticField>(), "M^1·T^-2·I^-1");

        // Catalytic Activity: N^1·T^-1 (amount per time)
        type CatalyticActivity = Quantity<DimensionVector<Z0, Z0, N1, Z0, Z0, P1, Z0>, ()>;
        assert_eq!(format_dims::<CatalyticActivity>(), "T^-1·N^1");

        // All dimensions involved: M^1·L^2·T^-1·I^1·Θ^1·N^-1·J^2
        type ComplexQuantity = Quantity<DimensionVector<P1, P2, N1, P1, P1, N1, P2>, ()>;
        assert_eq!(
            format_dims::<ComplexQuantity>(),
            "M^1·L^2·T^-1·I^1·Θ^1·N^-1·J^2"
        );
    }

    #[test]
    fn test_format_dims_dimensionless() {
        // Test dimensionless quantity (all exponents are zero)
        type Dimensionless = Quantity<DimensionVector<Z0, Z0, Z0, Z0, Z0, Z0, Z0>, ()>;
        assert_eq!(format_dims::<Dimensionless>(), "1");

        // Also test with DimensionZero alias
        use crate::model::dimension::DimensionZero;
        type AlsoDimensionless = Quantity<DimensionZero, ()>;
        assert_eq!(format_dims::<AlsoDimensionless>(), "1");
    }

    #[test]
    fn test_format_unit_dims_coherence() {
        // Test that format_unit_dims produces the same output as format_dims
        // for the unit's associated quantity

        use crate::system::*;
        fn verify_format<U: Unit, Q: QuantityMarker>() {
            assert_eq!(format_unit_dims::<U>(), format_dims::<Q>());
        }

        // Basic units
        verify_format::<Metre, Length>();
        verify_format::<Kilogram, Mass>();
        verify_format::<Second, Time>();
        verify_format::<Kelvin, Temperature>();
        verify_format::<Ampere, ElectricCurrent>();
        verify_format::<Mole, AmountOfSubstance>();
        verify_format::<Candela, LuminousIntensity>();

        // Derived units
        verify_format::<Newton, Force>();
        verify_format::<Pascal, Pressure>();
        verify_format::<Joule, Energy>();
        verify_format::<Watt, Power>();

        // Compound units
        verify_format::<MetrePerSecond, Velocity>();
        verify_format::<MetrePerSecondSquared, Acceleration>();

        // Prefixed units should have same dimensions as base units
        verify_format::<Kilometre, Length>();
        verify_format::<Centimetre, Length>();
        verify_format::<Gram, Mass>();

        // Imperial units
        verify_format::<Foot, Length>();
        verify_format::<Pound, Mass>();
    }

    #[test]
    fn test_format_unit_dims_specific_outputs() {
        use crate::system::*;

        // Test specific expected outputs for common units
        assert_eq!(format_unit_dims::<Metre>(), "L^1");
        assert_eq!(format_unit_dims::<Kilogram>(), "M^1");
        assert_eq!(format_unit_dims::<Second>(), "T^1");
        assert_eq!(format_unit_dims::<Newton>(), "M^1·L^1·T^-2");
        assert_eq!(format_unit_dims::<Pascal>(), "M^1·L^-1·T^-2");
        assert_eq!(format_unit_dims::<Joule>(), "M^1·L^2·T^-2");
        assert_eq!(format_unit_dims::<Watt>(), "M^1·L^2·T^-3");
        assert_eq!(format_unit_dims::<MetrePerSecond>(), "L^1·T^-1");
        assert_eq!(format_unit_dims::<MetrePerSecondSquared>(), "L^1·T^-2");

        // Test dimensionless units if any exist
        assert_eq!(format_unit_dims::<Radian>(), "1");
        assert_eq!(format_unit_dims::<Steradian>(), "1");
    }
}
