// Bidirectional Unit Conversion Examples
// =====================================
//
// This file shows how to use the new bidirectional conversion system.
// The system allows both:
// 1. Unit -> Quantity conversion using .into()
// 2. Quantity -> Unit conversion using Unit::from_quantity()

use crate::model::unit::FromQuantity;
use crate::system::defs::*;

/// Example demonstrating recipe conversions
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
