use ferrunitas::{model::quantity::QuantityMarker, system::defs::*};
#[test]
fn recipe_example() {
    // println!("=== Recipe Conversion ===");

    // Start with pounds (common in US recipes)
    let recipe_flour = Pound(2.5);

    // Convert to quantity for calculations
    let flour_mass: Mass = recipe_flour.into();

    // Convert to grams (common in metric recipes)
    let as_grams = Gram::from(flour_mass);

    // println!("Recipe: 2.5 lb flour = {:.0} g", as_grams.0);
    assert_eq!(as_grams.0, 1133.98);
}

/// Example demonstrating distance conversions
#[test]
fn distance_example() {
    // println!("=== Distance Planning ===");

    // Marathon distance in kilometers
    let marathon = Meter(42195.0);
    let distance: Length = marathon.into();

    // Convert to miles for US runners
    let in_miles = Mile::from(distance);

    // println!("Marathon: 42.195 km = {:.2} miles", in_miles.0);
    assert!((in_miles.0 - 26.219).abs() < 0.01);
}

/// Example demonstrating time conversions
#[test]
fn time_example() {
    // println!("=== Time Scheduling ===");

    let meeting = Minute(90.0); // 90 minute meeting
    let time: Time = meeting.into();
    let in_hours = Hour::from(time);

    // println!("Meeting: 90 minutes = {:.1} hours", in_hours.0);
    assert_eq!(in_hours.0, 1.5);
}

/// Example demonstrating bulk calculations
#[test]
fn bulk_calculation_example() {
    // println!("=== Mass Calculations ===");

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
    let total_pounds = Pound::from(total_mass);
    let total_ounces = Ounce::from(total_mass);

    // println!("Total ingredients:");
    // println!("  {:.0} g", total_mass.value());
    // println!("  {:.2} lb", total_pounds.0);
    // println!("  {:.1} oz", total_ounces.0);
    assert_eq!(total_mass.as_unit::<Gram>().0, 950.0);
    assert!((total_pounds.0 - 2.0943931991745885).abs() < 0.01);
    assert!((total_ounces.0 - 33.51).abs() < 0.01);
}

/// Example showing round-trip conversions
#[test]
fn round_trip_example() {
    // println!("=== Round-trip Conversions ===");

    // Start with an arbitrary unit value
    let original = Pound(15.5);
    // println!("Original: {:.1} lb", original.0);

    // Convert to quantity
    let as_quantity: Mass = original.into();
    println!("As quantity: {:.2}", as_quantity);
    assert!((as_quantity.as_unit::<Gram>().0 - 7030.68).abs() < 0.01);

    // Convert to different units
    // let as_kg = Kilogram::from(as_quantity);
    let as_oz = Ounce::from(as_quantity);

    // println!("As kilograms: {:.3} kg", as_kg.0);
    // println!("As ounces: {:.1} oz", as_oz.0);
    // assert!((as_kg.0 - 7.031).abs() < 0.01);
    assert!((as_oz.0 - 248.0).abs() < 0.01);

    // Convert back to original unit
    let back_to_pounds = Pound::from(as_quantity);
    // println!("Back to pounds: {:.1} lb", back_to_pounds.0);
    assert!((back_to_pounds.0 - 15.5).abs() < 0.01);

    // Verify precision
    let difference = (original.0 - back_to_pounds.0).abs();
    // println!("Precision check: difference = {:.10}", difference);
    assert_eq!(difference, 0.0);
}

/// Example showing the prefix system advantages
#[test]
fn prefix_system_example() {
    // println!("=== Prefix System Demonstration ===");

    // Show that prefixes use constants now (more efficient than functions)
    // println!("Kilo prefix factor: {}", Kilo::FACTOR);
    // println!("Centi prefix factor: {}", Centi::FACTOR);
    // assert_eq!(Kilo::FACTOR, 1000.0);
    // assert_eq!(Centi::FACTOR, 0.01);

    // Show consistent conversions using prefixes
    // let km = Kilometer(1.0);
    // let kg = Kilogram(1.0);
    // let cm = Centimeter(100.0);

    // let distance: Length = km.into();
    // let mass: Mass = kg.into();
    // let length: Length = cm.into();

    // println!("1 km = {} m (using Kilo prefix)", distance.value());
    // println!("1 kg = {} g (using Kilo prefix)", mass.value());
    // println!("100 cm = {} m (using Centi prefix)", length.value());
    // assert_eq!(distance.value(), 1000.0);
    // assert_eq!(mass.value(), 1000.0);
    // assert_eq!(length.value(), 1.0);

    // println!("All kilo-units use factor: {}", Kilo::FACTOR);
    // println!("All centi-units use factor: {}", Centi::FACTOR);

    // // Show how easy it is to add new prefixes
    // println!("\n--- Easy Prefix Extension ---");
    // println!("Adding new prefixes is now just one line:");
    // println!("define_prefix!(Nano, 0.000000001, \"n\", \"nano\");");
    // println!("define_prefix!(Tera, 1e12, \"T\", \"tera\");");
    // println!("All prefixes use constants (no function calls)!");
}
