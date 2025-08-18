mod examples;
mod model;
mod system;

use model::unit::FromQuantity;
use system::defs::*;

// ============================================================================
// Physics Functions with Compile-Time Type Safety
// ============================================================================

pub fn calculate_kinetic_energy(mass: Mass, velocity: Velocity) -> Energy {
    mass * velocity * velocity * 0.5 // KE = ½mv² - type checked at compile time!
}

pub fn calculate_work(force: Force, distance: Length) -> Energy {
    force * distance // W = F⋅d - type checked at compile time!
}

pub fn calculate_power_from_work_and_time(work: Energy, time: Time) -> Power {
    work / time // P = W/t - type checked at compile time!
}

// ============================================================================
// Main Function - Bidirectional Conversion Examples
// ============================================================================

pub fn main() {
    println!("=== Bidirectional Unit Conversion Demo ===\n");

    // Forward conversion: Unit -> Quantity
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

    // Round-trip conversion
    println!("\nRound-trip conversions:");
    let original = Pound(20.0);
    let as_quantity: Mass = original.into();
    let back_to_pounds = Pound::from_quantity(as_quantity);
    println!("20 lb -> Mass -> back to lb: {:.2} lb", back_to_pounds.0);

    // Length conversions
    println!("\nLength conversions:");
    let length_in_meters = Length::new(100.0); // 100 meters
    let as_feet = Foot::from_quantity(length_in_meters);
    let as_inches = Inch::from_quantity(length_in_meters);
    println!("100 m = {:.2} ft", as_feet.0);
    println!("100 m = {:.2} in", as_inches.0);

    // Time conversions
    println!("\nTime conversions:");
    let time_in_seconds = Time::new(3600.0); // 3600 seconds = 1 hour
    let as_minutes = Minute::from_quantity(time_in_seconds);
    let as_hours = Hour::from_quantity(time_in_seconds);
    println!("3600 s = {:.0} min", as_minutes.0);
    println!("3600 s = {:.0} hr", as_hours.0);

    println!("All conversions completed successfully!");

    // Run additional examples
    println!("\n{}", "=".repeat(50));
    println!("Additional Usage Examples:");
    println!("{}", "=".repeat(50));
    examples::recipe_example();
    println!();
    examples::distance_example();
    println!();
    examples::time_example();
    println!();
    examples::bulk_calculation_example();
    println!();
    examples::round_trip_example();
    println!();
    examples::prefix_system_example();
}
