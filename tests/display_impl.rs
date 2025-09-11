//! Integration tests for Display implementations of Unit, Quantity, and Measure types.
//!
//! This module tests that the Display trait is correctly implemented for all major types
//! in the ferrunitas library, ensuring consistent and meaningful output formatting.

use ferrunitas::Unit;
use ferrunitas::system::*;

#[test]
fn test_unit_display() {
    // Test basic SI units
    assert_eq!(format!("{}", Metre), "m");
    assert_eq!(format!("{}", Kilogram), "kg");
    assert_eq!(format!("{}", Second), "s");
    assert_eq!(format!("{}", Ampere), "A");
    assert_eq!(format!("{}", Kelvin), "K");
    assert_eq!(format!("{}", Mole), "mol");
    assert_eq!(format!("{}", Candela), "cd");

    // Test derived units
    assert_eq!(format!("{}", Newton), "N");
    assert_eq!(format!("{}", Pascal), "Pa");
    assert_eq!(format!("{}", Joule), "J");
    assert_eq!(format!("{}", Watt), "W");
    assert_eq!(format!("{}", Volt), "V");
    assert_eq!(format!("{}", Ohm), "Ω");

    // Test prefixed units
    assert_eq!(format!("{}", Kilometre), "km");
    assert_eq!(format!("{}", Centimetre), "cm");
    assert_eq!(format!("{}", Millimetre), "mm");
    assert_eq!(format!("{}", Gram), "g");
    assert_eq!(format!("{}", Milligram), "mg");

    // Test compound units
    assert_eq!(format!("{}", MetrePerSecond), "m/s");
    assert_eq!(format!("{}", MetrePerSecondSquared), "m/s²");
    assert_eq!(format!("{}", KilogramPerCubicMetre), "kg/m³");
}

#[test]
fn test_measure_display() {
    // Test basic measures with integer values
    let distance = 100 * Metre;
    assert_eq!(format!("{}", distance), "100 m");

    let mass = Kilogram::new(5i16);
    assert_eq!(format!("{}", mass), "5 kg");

    let time = Second::new(10i16);
    assert_eq!(format!("{}", time), "10 s");

    // Test measures with decimal values
    let length = Centimetre::new(42.5);
    assert_eq!(format!("{}", length), "42.5 cm");

    let weight = Gram::new(123.456);
    assert_eq!(format!("{}", weight), "123.456 g");

    // Test measures with scientific notation
    let small_distance = Millimetre::new(0.001);
    assert_eq!(format!("{}", small_distance), "0.001 mm");

    let large_distance = Kilometre::new(1000000.0);
    assert_eq!(format!("{}", large_distance), "1000000 km");

    // Test derived unit measures
    let force = Newton::new(9.81);
    assert_eq!(format!("{}", force), "9.81 N");

    let energy = Joule::new(1000.0);
    assert_eq!(format!("{}", energy), "1000 J");

    let power = Watt::new(750.0);
    assert_eq!(format!("{}", power), "750 W");

    // Test compound unit measures
    let velocity = MetrePerSecond::new(30.0);
    assert_eq!(format!("{}", velocity), "30 m/s");

    let acceleration = MetrePerSecondSquared::new(9.81);
    assert_eq!(format!("{}", acceleration), "9.81 m/s²");

    let density = KilogramPerCubicMetre::new(1000.0);
    assert_eq!(format!("{}", density), "1000 kg/m³");
}

#[test]
fn test_quantity_display() {
    // Test basic quantities - they should display as dimensional vectors
    let length = Metre::new(100.0).into_q();
    assert_eq!(format!("{}", length), "100 [L^1]");

    let mass = Kilogram::new(5.0).into_q();
    assert_eq!(format!("{}", mass), "5 [M^1]");

    let time = Second::new(10.0).into_q();
    assert_eq!(format!("{}", time), "10 [T^1]");

    // Test derived quantities - they should display as dimensional vectors
    let force = Newton::new(9.81).into_q();
    assert_eq!(format!("{}", force), "9.81 [M^1·L^1·T^-2]");

    let energy = Joule::new(1000.0).into_q();
    assert_eq!(format!("{}", energy), "1000 [M^1·L^2·T^-2]");

    let power = Watt::new(750.0).into_q();
    assert_eq!(format!("{}", power), "750 [M^1·L^2·T^-3]");

    // Test quantities created through arithmetic
    let distance = Metre::new(100.0).into_q();
    let time_duration = Second::new(10.0).into_q();
    let velocity = distance / time_duration;
    assert_eq!(format!("{}", velocity), "10 [L^1·T^-1]");

    let mass_value = Kilogram::new(2.0).into_q();
    let acceleration = velocity / time_duration;
    let force_calc = mass_value * acceleration;
    assert_eq!(format!("{}", force_calc), "2 [M^1·L^1·T^-2]");
}

#[test]
fn test_display_with_different_precisions() {
    // Test formatting with different decimal precisions
    let precise_value = Metre::new(1.2345678);
    assert_eq!(format!("{}", precise_value), "1.2345678 m");
    assert_eq!(format!("{:.2}", precise_value), "1.23 m");
    assert_eq!(format!("{:.5}", precise_value), "1.23457 m");

    // Test large and small values (basic formatting only)
    let large_value = Kilometre::new(1230000.0);
    assert_eq!(format!("{}", large_value), "1230000 km");

    let small_value = Millimetre::new(0.00123);
    assert_eq!(format!("{}", small_value), "0.00123 mm");
}

#[test]
fn test_display_zero_and_negative_values() {
    // Test zero values
    let zero_distance = Metre::new(0.0);
    assert_eq!(format!("{}", zero_distance), "0 m");

    let zero_mass = Kilogram::new(0i16);
    assert_eq!(format!("{}", zero_mass), "0 kg");

    // Test negative values
    let negative_temp = Kelvin::new(-10.0);
    assert_eq!(format!("{}", negative_temp), "-10 K");

    let negative_velocity = MetrePerSecond::new(-25.5);
    assert_eq!(format!("{}", negative_velocity), "-25.5 m/s");
}

#[test]
#[cfg(feature = "quantity_tags")]
fn test_display_with_quantity_tags() {
    // Test that measures with different unit types display their unit symbols correctly
    // even when they might be dimensionally similar
    let ratio = Metre::new(100.0).into_q() / Metre::new(50.0).into_q();
    assert_eq!(format!("{}", ratio.specify::<Angle>()), "2 [1] as Angle");

    // Test that different units maintain their distinct display
    let metres = Metre::new(1.0);
    let feet = Foot::new(1.0);

    assert_eq!(format!("{}", metres), "1 m");
    assert_eq!(format!("{}", feet), "1 ft");
    assert_ne!(format!("{}", metres), format!("{}", feet));
}
