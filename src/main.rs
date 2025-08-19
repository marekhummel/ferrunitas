// Example usage of the Ferrunitas library
// This serves as both documentation and demonstration of the library's capabilities

use ferrunitas::*;

// ============================================================================
// Physics Functions with Compile-Time Type Safety
// ============================================================================

fn calculate_kinetic_energy(mass: Mass, velocity: Velocity) -> Energy {
    mass * velocity * velocity * 0.5 // KE = ½mv² - type checked at compile time!
}

fn calculate_work(force: Force, distance: Length) -> Energy {
    force * distance // W = F⋅d - type checked at compile time!
}

fn calculate_power_from_work_and_time(work: Energy, time: Time) -> Power {
    work / time // P = W/t - type checked at compile time!
}

fn calculate_force(mass: Mass, acceleration: Acceleration) -> Force {
    mass * acceleration // F = ma - type checked at compile time!
}

fn calculate_velocity(distance: Length, time: Time) -> Velocity {
    distance / time // v = d/t - type checked at compile time!
}

fn calculate_acceleration(velocity_change: Velocity, time: Time) -> Acceleration {
    velocity_change / time // a = Δv/t - type checked at compile time!
}

/// Demonstrate dimensional analysis prevents errors at compile time
///
/// This function shows how mixing incompatible dimensions results in
/// compilation errors, making physics calculations safer.
fn demonstrate_compile_time_safety() {
    println!("=== Compile-Time Dimensional Analysis Demo ===");

    // These work fine - dimensionally consistent
    let mass = Mass::new(10.0); // 10 grams
    let velocity = Velocity::new(5.0); // 5 m/s

    let kinetic_energy = calculate_kinetic_energy(mass, velocity);
    println!("KE = ½mv² = {:.1} g⋅m²/s²", kinetic_energy.value());

    let force = Force::new(15.0); // 15 N
    let distance = Length::new(3.0); // 3 m
    let work = calculate_work(force, distance);
    println!("W = F⋅d = {:.1} J", work.value());

    let time = Time::new(2.0); // 2 s
    let power = calculate_power_from_work_and_time(work, time);
    println!("P = W/t = {:.1} W", power.value());

    // These won't compile - dimensionally inconsistent!
    // calculate_kinetic_energy(mass, distance);  // Error: velocity expected, not length
    // calculate_work(mass, velocity);           // Error: force and length expected
    // let invalid = mass + velocity;            // Error: can't add mass and velocity

    println!("✅ All calculations are dimensionally correct!");
    println!("💡 Try uncommenting the error lines to see compile-time protection!");
}

fn test_physics_functions() {
    // Test kinetic energy: KE = ½mv²
    let mass = Mass::new(4.0); // 4 grams
    let velocity = Velocity::new(6.0); // 6 m/s
    let ke = calculate_kinetic_energy(mass, velocity);
    assert_eq!(ke.value(), 72.0); // ½ * 4 * 6² = 72 g⋅m²/s²

    // Test work: W = F⋅d
    let force = Force::new(12.0); // 12 N
    let distance = Length::new(3.5); // 3.5 m
    let work = calculate_work(force, distance);
    assert_eq!(work.value(), 42.0); // 12 * 3.5 = 42 J

    // Test power: P = W/t
    let work = Energy::new(150.0); // 150 J
    let time = Time::new(10.0); // 10 s
    let power = calculate_power_from_work_and_time(work, time);
    assert_eq!(power.value(), 15.0); // 150 / 10 = 15 W

    // Test force: F = ma
    let mass = Mass::new(8.0); // 8 kg
    let acceleration = Acceleration::new(2.5); // 2.5 m/s²
    let force = calculate_force(mass, acceleration);
    assert_eq!(force.value(), 20.0); // 8 * 2.5 = 20 N

    // Test velocity: v = d/t
    let distance = Length::new(200.0); // 200 m
    let time = Time::new(25.0); // 25 s
    let velocity = calculate_velocity(distance, time);
    assert_eq!(velocity.value(), 8.0); // 200 / 25 = 8 m/s

    // Test acceleration: a = Δv/t
    let velocity_change = Velocity::new(30.0); // 30 m/s change
    let time = Time::new(6.0); // 6 s
    let acceleration = calculate_acceleration(velocity_change, time);
    assert_eq!(acceleration.value(), 5.0); // 30 / 6 = 5 m/s²
}

pub fn recipe_example() {
    println!("=== Recipe Conversion ===");

    // Start with pounds (common in US recipes)
    let recipe_flour = Pound(2.5);

    // Convert to quantity for calculations
    let flour_mass: Mass = recipe_flour.into();

    // Convert to grams (common in metric recipes)
    let as_grams = Gram::from_quantity(flour_mass);

    println!("Recipe: 2.5 lb flour = {:.0} g", as_grams.0);
}

/// Example demonstrating distance conversions
pub fn distance_example() {
    println!("=== Distance Planning ===");

    // Marathon distance in kilometers
    let marathon = Kilometer(42.195);
    let distance: Length = marathon.into();

    // Convert to miles for US runners
    let in_miles = Mile::from_quantity(distance);

    println!("Marathon: 42.195 km = {:.2} miles", in_miles.0);
}

/// Example demonstrating time conversions
pub fn time_example() {
    println!("=== Time Scheduling ===");

    let meeting = Minute(90.0); // 90 minute meeting
    let time: Time = meeting.into();
    let in_hours = Hour::from_quantity(time);

    println!("Meeting: 90 minutes = {:.1} hours", in_hours.0);
}

/// Example demonstrating bulk calculations
pub fn bulk_calculation_example() {
    println!("=== Mass Calculations ===");

    // Individual ingredients
    let ingredients = vec![
        Gram(500.0), // flour
        Gram(250.0), // butter
        Gram(200.0), // sugar
    ];

    // Sum all masses
    let total_mass: Mass = ingredients
        .into_iter()
        .map(|unit| unit.into())
        .fold(Mass::new(0.0), |acc, mass: Mass| acc + mass);

    // Convert to different units for display
    let total_pounds = Pound::from_quantity(total_mass);
    let total_ounces = Ounce::from_quantity(total_mass);

    println!("Total ingredients:");
    println!("  {:.0} g", total_mass.value());
    println!("  {:.2} lb", total_pounds.0);
    println!("  {:.1} oz", total_ounces.0);
}

/// Example showing round-trip conversions
pub fn round_trip_example() {
    println!("=== Round-trip Conversions ===");

    // Start with an arbitrary unit value
    let original = Pound(15.5);
    println!("Original: {:.1} lb", original.0);

    // Convert to quantity
    let as_quantity: Mass = original.into();
    println!("As quantity: {:.2} g", as_quantity.value());

    // Convert to different units
    let as_kg = Kilogram::from_quantity(as_quantity);
    let as_oz = Ounce::from_quantity(as_quantity);

    println!("As kilograms: {:.3} kg", as_kg.0);
    println!("As ounces: {:.1} oz", as_oz.0);

    // Convert back to original unit
    let back_to_pounds = Pound::from_quantity(as_quantity);
    println!("Back to pounds: {:.1} lb", back_to_pounds.0);

    // Verify precision
    let difference = (original.0 - back_to_pounds.0).abs();
    println!("Precision check: difference = {:.10}", difference);
}

/// Example showing the prefix system advantages
pub fn prefix_system_example() {
    use crate::model::prefix::*;

    println!("=== Prefix System Demonstration ===");

    // Show that prefixes use constants now (more efficient than functions)
    println!("Kilo prefix factor: {}", Kilo::FACTOR);
    println!("Centi prefix factor: {}", Centi::FACTOR);

    // Show consistent conversions using prefixes
    let km = Kilometer(1.0);
    let kg = Kilogram(1.0);
    let cm = Centimeter(100.0);

    let distance: Length = km.into();
    let mass: Mass = kg.into();
    let length: Length = cm.into();

    println!("1 km = {} m (using Kilo prefix)", distance.value());
    println!("1 kg = {} g (using Kilo prefix)", mass.value());
    println!("100 cm = {} m (using Centi prefix)", length.value());

    println!("All kilo-units use factor: {}", Kilo::FACTOR);
    println!("All centi-units use factor: {}", Centi::FACTOR);

    // Show how easy it is to add new prefixes
    println!("\n--- Easy Prefix Extension ---");
    println!("Adding new prefixes is now just one line:");
    println!("define_prefix!(Nano, 0.000000001, \"n\", \"nano\");");
    println!("define_prefix!(Tera, 1e12, \"T\", \"tera\");");
    println!("All prefixes use constants (no function calls)!");
}

fn main() {
    println!("=== Ferrunitas Library Demo ===\n");

    // Show basic conversions using the library
    println!("Forward conversions (Unit -> Quantity):");
    let mass1: Mass = Kilogram(5.0).into();
    let mass2: Mass = Pound(10.0).into();
    println!("5 kg = {:.2} g", mass1.value());
    println!("10 lb = {:.2} g", mass2.value());

    // Backward conversion: Quantity -> Unit
    println!("\nBackward conversions (Quantity -> Unit):");
    let mass_in_grams = Mass::new(5000.0); // 5000 grams
    let as_kg = Kilogram::from_quantity(mass_in_grams);
    let as_pounds = Pound::from_quantity(mass_in_grams);
    let as_grams = Gram::from_quantity(mass_in_grams);
    println!("5000 g = {:.2} kg", as_kg.0);
    println!("5000 g = {:.2} lb", as_pounds.0);
    println!("5000 g = {:.0} g", as_grams.0);

    // Physics calculations using the library
    println!("\nPhysics calculations:");
    let mass = Mass::new(5000.0); // 5 kg in grams
    let velocity = Velocity::new(10.0); // 10 m/s
    let kinetic_energy = calculate_kinetic_energy(mass, velocity);
    println!(
        "Kinetic energy of 5kg object at 10m/s: {:.1} g⋅m²/s²",
        kinetic_energy.value()
    );

    // Demonstrate compile-time safety
    println!();
    demonstrate_compile_time_safety();
    println!();
    test_physics_functions();

    // Run library examples
    println!("\n{}", "=".repeat(50));
    println!("Additional Usage Examples:");
    println!("{}", "=".repeat(50));
    recipe_example();
    println!();
    distance_example();
    println!();
    time_example();
    println!();
    bulk_calculation_example();
    println!();
    round_trip_example();
    println!();
    prefix_system_example();
}
