// Example usage of the Ferrunitas library
// This serves as both documentation and demonstration of the library's capabilities

use ferrunitas::{model::unit::Unit, system::defs::*};

// ============================================================================
// Physics Functions with Compile-Time Type Safety
// ============================================================================

fn calculate_kinetic_energy(
    mass: impl Unit<Quantity = Mass>,
    velocity: impl Unit<Quantity = Velocity>,
) -> Energy {
    let v = velocity.into_q();
    0.5 * mass.into_q() * v * v // KE = ½mv² - type checked at compile time!
}

fn calculate_work<F, L>(force: F, distance: L) -> Energy
where
    F: Unit<Quantity = Force>,
    L: Unit<Quantity = Length>,
{
    force.into_q() * distance.into_q() // W = F⋅d - type checked at compile time!
}

fn calculate_power_from_work_and_time(
    work: impl Unit<Quantity = Energy>,
    time: impl Unit<Quantity = Time>,
) -> Power {
    work.to_q() / time.to_q() // P = W/t - type checked at compile time!
}

fn calculate_force(
    mass: impl Unit<Quantity = Mass>,
    acceleration: impl Unit<Quantity = Acceleration>,
) -> Force {
    mass.into_q() * acceleration.into_q() // F = ma - type checked at compile time!
}

fn calculate_velocity(
    distance: impl Unit<Quantity = Length>,
    time: impl Unit<Quantity = Time>,
) -> Velocity {
    distance.into_q() / time.into_q() // v = d/t - type checked at compile time!
}

fn calculate_acceleration(
    velocity_change: impl Unit<Quantity = Velocity>,
    time: impl Unit<Quantity = Time>,
) -> Acceleration {
    velocity_change.into_q() / time.into_q() // a = Δv/t - type checked at compile time!
}

/// Demonstrate dimensional analysis prevents errors at compile time
///
/// This function shows how mixing incompatible dimensions results in
/// compilation errors, making physics calculations safer.
fn demonstrate_compile_time_safety() {
    println!("=== Compile-Time Dimensional Analysis Demo ===");

    // These work fine - dimensionally consistent
    let mass = Gram::new(10.0); // 10 grams
    let velocity = MeterPerSecond::new(5.0); // 5 m/s

    let kinetic_energy = calculate_kinetic_energy(mass, velocity);
    println!("KE = ½mv² = {:.1}", kinetic_energy.as_unit::<Joule>());

    let force = Newton::new(15.0); // 15 N
    let distance = Meter::new(3.0); // 3 m
    let work = calculate_work(force, distance);
    println!("W = F⋅d = {:.1}", work.as_unit::<Joule>());

    let time = Second::new(2.0); // 2 s
    let power = calculate_power_from_work_and_time(work.as_unit::<Joule>(), time);
    println!("P = W/t = {:.1}", power.as_unit::<Watt>());

    // These won't compile - dimensionally inconsistent!
    // calculate_kinetic_energy(mass, distance);  // Error: velocity expected, not length
    // calculate_work(mass, velocity);           // Error: force and length expected
    // let invalid = mass + velocity;            // Error: can't add mass and velocity

    println!("✅ All calculations are dimensionally correct!");
    println!("💡 Try uncommenting the error lines to see compile-time protection!");
}

fn test_physics_functions() {
    // Test kinetic energy: KE = ½mv²
    let mass = Gram::new(4.0);
    let velocity = MeterPerSecond::new(6.0);
    let ke = calculate_kinetic_energy(mass, velocity);
    assert_eq!(ke.as_unit::<Joule>().raw_value(), 0.072);

    // Test work: W = F⋅d
    let force = Newton::new(12.0);
    let distance = Meter::new(3.5);
    let work = calculate_work(force, distance);
    assert_eq!(work.as_unit::<Joule>().raw_value(), 42.0);
    let area1 = distance * distance;

    let length: Length = distance.into_q();
    let area2 = length * length; // Also Length²
    println!("Area:\n{:.4}\n{}", area1, area2);

    // Test power: P = W/t
    let work = Joule::new(150.0);
    let time = Second::new(10.0);
    let power = calculate_power_from_work_and_time(work, time);
    assert_eq!(power.as_unit::<Watt>().raw_value(), 15.0);

    // Test force: F = ma
    let mass = Gram::new(8.0);
    let acceleration = MeterPerSecondSquared::new(2.5);
    let force = calculate_force(mass, acceleration);
    assert_eq!(force.as_unit::<Newton>().raw_value(), 0.02);

    // Test velocity: v = d/t
    let distance = Meter::new(200.0);
    let time = Second::new(25.0);
    let velocity = calculate_velocity(distance, time);
    assert_eq!(velocity.as_unit::<MeterPerSecond>().raw_value(), 8.0);

    // Test acceleration: a = Δv/t
    let velocity_change = MeterPerSecond::new(30.0);
    let time = Second::new(6.0);
    let acceleration = calculate_acceleration(velocity_change, time);
    assert_eq!(
        acceleration.as_unit::<MeterPerSecondSquared>().raw_value(),
        5.0
    );
}

fn main() {
    println!("=== Ferrunitas Library Demo ===\n");

    // Show basic conversions using the library
    println!("Forward conversions (Unit -> Quantity):");
    let lb1 = Gram::new(53.0);
    let lb2 = Pound::new(10.0);
    let mass1: Mass = lb1.into_q();
    let mass2: Mass = lb2.into_q();
    // println!("5 kg = {:.2}", mass1.as_unit::<Gram>());
    println!("Single: {:.2}, {:.2}", lb1, lb2);
    println!("Sum: {:.2}", lb1 + lb2);
    println!("As Mass: {:.2}, {:.2}", mass1, mass2);
    println!("Mass Sum: {:.2}", (mass1 + mass2));
    println!(
        "Sum converted: {:.2}, {:.2}",
        (mass1 + mass2).as_unit::<Gram>(),
        (mass1 + mass2).as_unit::<Pound>()
    );
    println!(
        "Inline Sum Conv: {:.2}",
        (lb1 + lb2).to_q().as_unit::<Pound>()
    );
    println!(
        "Inline Convert: {:.5}, {:.2}",
        lb1.convert::<Pound>(),
        lb2.convert::<Gram>()
    );

    // Backward conversion: Quantity -> Unit
    println!("\nBackward conversions (Quantity -> Unit):");
    let grams = Gram::new(5000.0); // 5000 grams
    let mass: Mass = grams.into_q();
    let as_kg = mass.as_unit::<Kilogram>();
    let as_pounds = mass.as_unit::<Pound>();
    let as_grams = mass.as_unit::<Gram>();
    println!("5000 g = {:.2}", as_kg);
    println!("5000 g = {:.2}", as_pounds);
    println!("5000 g = {:.0}", as_grams);

    // Lengths
    println!("\nLengths:");
    let miles = Mile::new(1.0); // 1 mile
    let yards: Yard = miles.convert();
    let feet: Foot = miles.convert();
    let inches1: Inch = miles.convert();
    let inches2: Inch = feet.convert();
    let inches3: Inch = yards.convert();
    println!("{:.2}", miles.raw_value());
    println!("{:.2} = {:.2} = {:.2} = {:.2}", miles, yards, feet, inches1);
    assert_eq!(inches1.raw_value(), inches2.raw_value());
    assert_eq!(inches1.raw_value(), inches3.raw_value());
    println!("1 mile = {:.2}", Mile::new(1.0).convert::<Yard>());
    println!("1 yard = {:.2}", Yard::new(1.0).convert::<Foot>());
    println!("1 foot = {:.2}", Foot::new(1.0).convert::<Inch>());

    // Relative unit definitions
    let distance = Meter::new(2000.0);
    let time = Minute::new(5.0);
    let speed = distance / time;
    println!("Speed: {}", speed);
    println!("Speed in m/s: {:.5}", speed.as_unit::<MeterPerSecond>());
    println!("Speed in knots: {:.5}", speed.as_unit::<Knots>());

    // Demonstrate compile-time safety
    println!();
    demonstrate_compile_time_safety();
    println!();
    test_physics_functions();
}
