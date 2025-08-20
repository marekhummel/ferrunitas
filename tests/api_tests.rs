// API visibility and soundness tests
// These tests ensure that external consumers can use the library correctly

use ferrunitas::{model::quantity::QuantityMarker, system::defs::*};

#[test]
fn test_public_api_visibility() {
    // Test that all the main types are accessible
    let _mass: Mass = Mass::new(1.0);
    let _length: Length = Length::new(1.0);
    let _time: Time = Time::new(1.0);
    let _velocity: Velocity = Velocity::new(1.0);
    let _acceleration: Acceleration = Acceleration::new(1.0);
    let _force: Force = Force::new(1.0);
    let _energy: Energy = Energy::new(1.0);
    let _power: Power = Power::new(1.0);

    // Test that unit types are accessible
    let _gram = Gram(1.0);
    let _pound = Pound(1.0);
    let _ounce = Ounce(1.0);

    let _meter = Meter(1.0);
    let _inch = Inch(1.0);
    let _foot = Foot(1.0);
    let _mile = Mile(1.0);

    let _second = Second(1.0);
    let _minute = Minute(1.0);
    let _hour = Hour(1.0);
}

#[test]
fn test_from_trait() {
    // Ensure FromQuantity trait is accessible and works
    let mass = Mass::new(453.592);
    let lb = Pound::from(mass);
    assert_eq!(lb.0, 1.0);

    let length = Length::new(1609.34);
    let sm = Mile::from(length);
    assert_eq!(sm.0, 1.0);
}

#[test]
fn test_core_traits_work() {
    let mass1 = Mass::new(100.0);
    let mass2 = Mass::new(50.0);

    // Test arithmetic operations
    let sum = mass1 + mass2;
    assert_eq!(sum.as_unit::<Gram>().0, 150.0);

    let diff = mass1 - mass2;
    assert_eq!(diff.as_unit::<Gram>().0, 50.0);

    // Test scalar operations
    let doubled = mass1 * 2.0;
    assert_eq!(doubled.as_unit::<Gram>().0, 200.0);

    let halved = mass1 / 2.0;
    assert_eq!(halved.as_unit::<Gram>().0, 50.0);

    // Test dimensional multiplication
    let length = Length::new(10.0);
    let time = Time::new(2.0);
    let velocity = length / time;
    assert_eq!(velocity.as_unit::<MeterPerSecond>().0, 5.0);
}

#[test]
fn test_compile_time_dimensional_safety() {
    // These should all compile and work correctly
    let mass = Mass::new(10.0);
    let length = Length::new(5.0);
    let time = Time::new(2.0);

    // Valid dimensional operations
    let velocity = length / time;
    let acceleration = velocity / time;
    let force = mass * acceleration;
    let energy = force * length;
    let power = energy / time;

    // Verify the calculations work
    assert_eq!(velocity.as_unit::<MeterPerSecond>().0, 2.5);
    assert_eq!(acceleration.as_unit::<MeterPerSecondSquared>().0, 1.25);
    assert_eq!(force.as_unit::<Newton>().0, 12.5);
    assert_eq!(energy.as_unit::<Joule>().0, 62.5);
    assert_eq!(power.as_unit::<Watt>().0, 31.25);

    // The following should NOT compile (commented out):
    // let invalid1 = mass + length;     // Can't add mass and length
    // let invalid2 = velocity + energy; // Can't add velocity and energy
    // let invalid3 = time * power;      // Wrong dimensions
}

#[test]
fn test_precision_and_accuracy() {
    // Test that calculations maintain reasonable precision
    let original = 123.456789;
    let mass = Mass::new(original);
    assert_eq!(mass.as_unit::<Gram>().0, original);

    // Test round-trip precision
    let pound = Pound(original);
    let mass: Mass = pound.into();
    let back_to_pound = Pound::from(mass);
    let difference = (back_to_pound.0 - original).abs();
    assert!(difference < 1e-12); // Should have very high precision
}

#[test]
fn test_edge_cases() {
    // Test zero values
    let zero_mass = Mass::new(0.0);
    assert_eq!(zero_mass.as_unit::<Gram>().0, 0.0);

    // let zero_kg = Kilogram::from(zero_mass);
    // assert_eq!(zero_kg.0, 0.0);

    // Test very small values
    let tiny_mass = Mass::new(1e-10);
    assert_eq!(tiny_mass.as_unit::<Gram>().0, 1e-10);

    // Test very large values
    let huge_mass = Mass::new(1e10);
    assert_eq!(huge_mass.as_unit::<Gram>().0, 1e10);
}

#[test]
fn test_library_can_be_used_in_generic_contexts() {
    // Test that the types work in generic functions
    fn double_quantity<T>(quantity: T) -> T
    where
        T: std::ops::Mul<f64, Output = T>,
    {
        quantity * 2.0
    }

    let mass = Mass::new(50.0);
    let doubled_mass = double_quantity(mass);
    assert_eq!(doubled_mass.as_unit::<Gram>().0, 100.0);

    let length = Length::new(25.0);
    let doubled_length = double_quantity(length);
    assert_eq!(doubled_length.as_unit::<Meter>().0, 50.0);
}

#[test]
fn test_complex_calculations() {
    // Test a complex physics problem to ensure everything works together

    // A 5kg object falls for 3 seconds under gravity (9.8 m/s²)
    let mass = Mass::new(5000.0); // 5kg in grams
    let gravity = Acceleration::new(9.8); // m/s²
    let fall_time = Time::new(3.0); // seconds

    // Calculate distance fallen: d = ½at²
    let distance = gravity * fall_time * fall_time * 0.5; // TODO: Mul f64 only works from right side
    assert!((distance.as_unit::<Meter>().0 - 44.1).abs() < 0.01);

    // Calculate final velocity: v = at = d/t
    let final_velocity = distance / fall_time;

    // This should be approximately the velocity from v = at = 9.8 * 3 = 29.4 m/s
    // But since we're using d = ½at² and v = d/t, we get v = ½at²/t = ½at = ½ * 9.8 * 3 = 14.7 m/s
    assert!((final_velocity.as_unit::<MeterPerSecond>().0 - 14.7).abs() < 0.1);

    // Calculate kinetic energy: KE = ½mv²
    let kinetic_energy = mass * final_velocity * final_velocity * 0.5;
    let expected_ke = 0.5 * 5000.0 * 14.7 * 14.7; // ½ * 5kg * (14.7 m/s)²
    assert!((kinetic_energy.as_unit::<Joule>().0 - expected_ke).abs() < expected_ke * 0.01); // Within 1%

    // Calculate the force of gravity: F = ma
    let gravitational_force = mass * gravity;
    assert!((gravitational_force.as_unit::<Newton>().0 - 49000.0).abs() < 1.0); // 5kg * 9.8 m/s² = 49N (49000 g⋅m/s²)
}
