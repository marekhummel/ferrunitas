// Example usage of the Ferrunitas library
// This serves as both documentation and demonstration of the library's capabilities

use ferrunitas::{
    model::{
        quantity::Quantity,
        unit::{PrefixedUnit, Unit},
    },
    system::defs::*,
};

// ============================================================================
// Physics Functions with Compile-Time Type Safety
// ============================================================================

fn add_masses<M1, M2>(mass1: M1, mass2: M2) -> Mass
where
    M1: Unit<Quantity = Mass> + std::ops::Add<M2, Output = M1>,
    M2: Unit<Quantity = Mass>,
{
    (mass1 + mass2).into()
}

fn calculate_kinetic_energy(
    mass: impl Unit<Quantity = Mass>,
    velocity: impl Unit<Quantity = Velocity>,
) -> Energy {
    let v = velocity.into();
    0.5 * mass.into() * v * v // KE = ½mv² - type checked at compile time!
}

fn calculate_work<F, L>(force: F, distance: L) -> Energy
where
    F: Unit<Quantity = Force>,
    L: Unit<Quantity = Length>,
{
    force.into() * distance.into() // W = F⋅d - type checked at compile time!
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
    mass.into() * acceleration.into() // F = ma - type checked at compile time!
}

fn calculate_velocity(
    distance: impl Unit<Quantity = Length>,
    time: impl Unit<Quantity = Time>,
) -> Velocity {
    distance.into() / time.into() // v = d/t - type checked at compile time!
}

fn calculate_acceleration(
    velocity_change: impl Unit<Quantity = Velocity>,
    time: impl Unit<Quantity = Time>,
) -> Acceleration {
    velocity_change.into() / time.into() // a = Δv/t - type checked at compile time!
}

/// Demonstrate dimensional analysis prevents errors at compile time
///
/// This function shows how mixing incompatible dimensions results in
/// compilation errors, making physics calculations safer.
fn demonstrate_compile_time_safety() {
    println!("=== Compile-Time Dimensional Analysis Demo ===");

    // These work fine - dimensionally consistent
    let mass = Gram(10.0); // 10 grams
    let velocity = MeterPerSecond(5.0); // 5 m/s

    let kinetic_energy = calculate_kinetic_energy(mass, velocity);
    println!("KE = ½mv² = {:.1}", kinetic_energy.as_unit::<Joule>());

    let force = Newton(15.0); // 15 N
    let distance = Meter(3.0); // 3 m
    let work = calculate_work(force, distance);
    println!("W = F⋅d = {:.1}", work.as_unit::<Joule>());

    let time = Second(2.0); // 2 s
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
    let mass = Gram(4.0); // 4 grams
    let velocity = MeterPerSecond(6.0); // 6 m/s
    let ke = calculate_kinetic_energy(mass, velocity);
    assert_eq!(ke.as_unit::<Joule>().0, 72.0); // ½ * 4 * 6² = 72 g⋅m²/s²

    // Test work: W = F⋅d
    let force = Newton(12.0); // 12 N
    let distance = Meter(3.5); // 3.5 m
    let work = calculate_work(force, distance);
    assert_eq!(work.as_unit::<Joule>().0, 42.0); // 12 * 3.5 = 42 J
    let area1 = distance * distance;

    let length: Length = distance.into();
    let area2 = length * length; // Also Length²
    println!("Area:\n{:.4}\n{}", area1, area2);

    // Test power: P = W/t
    let work = Joule(150.0); // 150 J
    let time = Second(10.0); // 10 s
    let power = calculate_power_from_work_and_time(work, time);
    assert_eq!(power.as_unit::<Watt>().0, 15.0); // 150 / 10 = 15 W

    // Test force: F = ma
    let mass = Gram(8.0); // 8 kg
    let acceleration = MeterPerSecondSquared(2.5); // 2.5 m/s²
    let force = calculate_force(mass, acceleration);
    assert_eq!(force.as_unit::<Newton>().0, 20.0); // 8 * 2.5 = 20 N

    // Test velocity: v = d/t
    let distance = Meter(200.0); // 200 m
    let time = Second(25.0); // 25 s
    let velocity = calculate_velocity(distance, time);
    assert_eq!(velocity.as_unit::<MeterPerSecond>().0, 8.0); // 200 / 25 = 8 m/s

    // Test acceleration: a = Δv/t
    let velocity_change = MeterPerSecond(30.0); // 30 m/s change
    let time = Second(6.0); // 6 s
    let acceleration = calculate_acceleration(velocity_change, time);
    assert_eq!(acceleration.as_unit::<MeterPerSecondSquared>().0, 5.0); // 30 / 6 = 5 m/s²
}

fn main() {
    println!("=== Ferrunitas Library Demo ===\n");

    // Show basic conversions using the library
    println!("Forward conversions (Unit -> Quantity):");
    let lb1 = Gram(53.0);
    let lb2 = Pound(10.0);
    let mass1: Mass = lb1.into();
    let mass2: Mass = lb2.into();
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
        "Inline Convert: {:.2}, {:.2}",
        lb1.convert::<Pound>(),
        lb2.convert::<Gram>()
    );

    type Kilogram = PrefixedUnit<Kilo, Gram>;
    let x: Kilogram = Kilogram::new(5.0);

    // Backward conversion: Quantity -> Unit
    println!("\nBackward conversions (Quantity -> Unit):");
    let grams = Gram(5000.0); // 5000 grams
    let mass: Mass = grams.into();
    // let as_kg = Kilogram::from(mass);
    let as_pounds = Pound::from(mass);
    let as_grams = Gram::from(mass);
    // println!("5000 g = {:.2} kg", as_kg.0);
    println!("5000 g = {:.2}", as_pounds);
    println!("5000 g = {:.0}", as_grams);

    // Physics calculations using the library
    println!("\nPhysics calculations:");
    let mass = Gram(5000.0); // 5 kg in grams
    let velocity = MeterPerSecond(10.0); // 10 m/s
    let kinetic_energy = calculate_kinetic_energy(mass, velocity);
    println!(
        "Kinetic energy of 5kg object at 10m/s: {:.1} J",
        kinetic_energy.as_unit::<Joule>()
    );

    // Demonstrate compile-time safety
    println!();
    demonstrate_compile_time_safety();
    println!();
    test_physics_functions();
}
