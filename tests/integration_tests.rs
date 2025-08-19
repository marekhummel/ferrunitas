// Integration tests for the Ferrunitas library
// These tests validate the public API as it would be used by external consumers

use ferrunitas::*;

// Tests

#[test]
fn test_basic_unit_conversions() {
    // Test mass conversions
    let kg_mass: Mass = Kilogram(1.0).into();
    let gram_unit = Gram::from_quantity(kg_mass);
    assert_eq!(gram_unit.0, 1000.0);

    let pound_mass: Mass = Pound(1.0).into();
    let gram_from_pound = Gram::from_quantity(pound_mass);
    assert!((gram_from_pound.0 - 453.592).abs() < 1e-3);

    // Test length conversions
    let km_length: Length = Kilometer(1.0).into();
    let meter_unit = Meter::from_quantity(km_length);
    assert_eq!(meter_unit.0, 1000.0);

    let mile_length: Length = Mile(1.0).into();
    let meter_from_mile = Meter::from_quantity(mile_length);
    assert!((meter_from_mile.0 - 1609.34).abs() < 1e-2);
}

#[test]
fn test_round_trip_conversions() {
    // Mass round trip
    let original_pounds = 25.5;
    let pound_unit = Pound(original_pounds);
    let mass: Mass = pound_unit.into();
    let back_to_pounds = Pound::from_quantity(mass);
    assert!((back_to_pounds.0 - original_pounds).abs() < 1e-10);

    // Length round trip
    let original_km = 42.195;
    let km_unit = Kilometer(original_km);
    let length: Length = km_unit.into();
    let back_to_km = Kilometer::from_quantity(length);
    assert!((back_to_km.0 - original_km).abs() < 1e-10);

    // Time round trip
    let original_hours = 2.5;
    let hour_unit = Hour(original_hours);
    let time: Time = hour_unit.into();
    let back_to_hours = Hour::from_quantity(time);
    assert!((back_to_hours.0 - original_hours).abs() < 1e-10);
}

#[test]
fn test_dimensional_arithmetic() {
    let mass = Mass::new(10.0); // 10 grams
    let length = Length::new(5.0); // 5 meters
    let time = Time::new(2.0); // 2 seconds

    // Test multiplication creates correct dimensions
    let velocity = length / time; // Should be Velocity
    assert_eq!(velocity.value(), 2.5); // 5/2 = 2.5 m/s

    let acceleration = velocity / time; // Should be Acceleration
    assert_eq!(acceleration.value(), 1.25); // 2.5/2 = 1.25 m/s²

    let force = mass * acceleration; // Should be Force
    assert_eq!(force.value(), 12.5); // 10 * 1.25 = 12.5 N

    let energy = force * length; // Should be Energy
    assert_eq!(energy.value(), 62.5); // 12.5 * 5 = 62.5 J

    let power = energy / time; // Should be Power
    assert_eq!(power.value(), 31.25); // 62.5 / 2 = 31.25 W
}

#[test]
fn test_prefix_system() {
    // Test that prefixed units work correctly
    let km: Length = Kilometer(5.0).into();
    let cm_unit = Centimeter::from_quantity(km);
    assert_eq!(cm_unit.0, 500000.0); // 5 km = 500,000 cm

    let kg: Mass = Kilogram(2.5).into();
    let gram_unit = Gram::from_quantity(kg);
    assert_eq!(gram_unit.0, 2500.0); // 2.5 kg = 2500 g

    // Test millimeter
    let meter: Length = Meter(0.5).into();
    let mm_unit = Millimeter::from_quantity(meter);
    assert_eq!(mm_unit.0, 500.0); // 0.5 m = 500 mm
}

#[test]
fn test_derived_units() {
    // Test non-SI units
    let inch_length: Length = Inch(12.0).into();
    let meter_unit = Meter::from_quantity(inch_length);
    assert!((meter_unit.0 - 0.3048).abs() < 1e-4); // 12 inches = 1 foot = 0.3048 m

    let foot_length: Length = Foot(3.0).into();
    let meter_from_foot = Meter::from_quantity(foot_length);
    assert!((meter_from_foot.0 - 0.9144).abs() < 1e-4); // 3 feet = 0.9144 m

    let ounce_mass: Mass = Ounce(16.0).into();
    let gram_from_ounce = Gram::from_quantity(ounce_mass);
    assert!((gram_from_ounce.0 - 453.592).abs() < 1e-3); // 16 oz = 1 lb ≈ 453.592 g
}

#[test]
fn test_time_units() {
    // Test minute to second conversion
    let minute_time: Time = Minute(5.0).into();
    let second_unit = Second::from_quantity(minute_time);
    assert_eq!(second_unit.0, 300.0); // 5 minutes = 300 seconds

    // Test hour to second conversion
    let hour_time: Time = Hour(1.5).into();
    let second_from_hour = Second::from_quantity(hour_time);
    assert_eq!(second_from_hour.0, 5400.0); // 1.5 hours = 5400 seconds

    // Test minute to hour conversion
    let minute_time: Time = Minute(90.0).into();
    let hour_unit = Hour::from_quantity(minute_time);
    assert_eq!(hour_unit.0, 1.5); // 90 minutes = 1.5 hours
}

#[test]
fn test_quantity_arithmetic() {
    // Test addition and subtraction
    let mass1 = Mass::new(100.0);
    let mass2 = Mass::new(50.0);
    let total = mass1 + mass2;
    assert_eq!(total.value(), 150.0);

    let difference = mass1 - mass2;
    assert_eq!(difference.value(), 50.0);

    // Test scalar multiplication and division
    let length = Length::new(10.0);
    let doubled = length * 2.0;
    assert_eq!(doubled.value(), 20.0);

    let halved = length / 2.0;
    assert_eq!(halved.value(), 5.0);
}

#[test]
fn test_display_formatting() {
    let mass = Mass::new(123.456);
    let formatted = format!("{}", mass);
    assert_eq!(formatted, "123.456");

    let length = Length::new(789.0); // Use a simpler number
    let formatted = format!("{:.2}", length.value());
    assert_eq!(formatted, "789.00");
}

#[test]
fn test_zero_and_negative_values() {
    // Test that zero values work correctly
    let zero_mass = Mass::new(0.0);
    let zero_kg = Kilogram::from_quantity(zero_mass);
    assert_eq!(zero_kg.0, 0.0);

    let negative_velocity = Velocity::new(-5.0);
    let time = Time::new(2.0);
    let acceleration = negative_velocity / time;
    assert_eq!(acceleration.value(), -2.5); // Deceleration
}

#[test]
fn test_large_and_small_values() {
    // Test very large values
    let large_distance: Length = Kilometer(1000000.0).into(); // 1 million km
    let meter_unit = Meter::from_quantity(large_distance);
    assert_eq!(meter_unit.0, 1e9); // 1 billion meters

    // Test very small values
    let small_length: Length = Millimeter(0.001).into(); // 0.001 mm
    let meter_from_small = Meter::from_quantity(small_length);
    assert_eq!(meter_from_small.0, 1e-6); // 1 micrometer
}

#[test]
fn test_copy_and_clone() {
    let original = Mass::new(42.0);

    // Test Copy trait
    let copied = original; // This should work due to Copy trait
    assert_eq!(copied.value(), 42.0);
    assert_eq!(original.value(), 42.0); // Original should still be usable

    // Test Clone trait
    let cloned = original.clone();
    assert_eq!(cloned.value(), 42.0);
}

#[test]
fn test_debug_formatting() {
    let mass = Mass::new(100.0);
    let debug_str = format!("{:?}", mass);
    assert!(debug_str.contains("100")); // Should contain the value

    // Note: Some units may not implement Debug, which is fine for the public API
    // The important thing is that the core quantities do
}
