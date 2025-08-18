// Test to demonstrate the prefix system improvements

use crate::model::prefix::*;
use crate::model::unit::FromQuantity;
use crate::system::defs::*;

pub fn prefix_system_demo() {
    println!("=== Prefix System Demonstration ===");

    // Show that Kilogram is now a PrefixedUnit<Kilo, Gram>
    let kg = Kilogram(5.0);
    println!("Kilogram is PrefixedUnit<Kilo, Gram>: {} kg", kg.0);

    // Show the prefix factor
    println!("Kilo prefix factor: {}", Kilo::factor());
    println!("Kilo prefix symbol: {}", Kilo::symbol());

    // Conversions still work the same
    let mass: Mass = kg.into();
    println!("5 kg = {} g (base units)", mass.value());

    // Show centimeter and millimeter work too
    let cm = Centimeter(100.0);
    let mm = Millimeter(1000.0);

    let length1: Length = cm.into();
    let length2: Length = mm.into();

    println!("100 cm = {} m", length1.value());
    println!("1000 mm = {} m", length2.value());

    // Show they're equivalent
    println!(
        "100 cm == 1000 mm? {}",
        (length1.value() - length2.value()).abs() < 1e-10
    );

    // Backward conversion
    let length_1m = Length::new(1.0);
    let as_cm = Centimeter::from_quantity(length_1m);
    let as_mm = Millimeter::from_quantity(length_1m);
    let as_km = Kilometer::from_quantity(length_1m);

    println!("1 m = {} cm", as_cm.0);
    println!("1 m = {} mm", as_mm.0);
    println!("1 m = {} km", as_km.0);
}

#[allow(dead_code)]
pub fn demonstrate_easy_prefix_extension() {
    println!("\n=== Easy Prefix Extension ===");

    // This shows how easy it would be to add new prefixed units:
    // prefixed_unit_type!(Micrometer, Micro, Meter);
    // impl_prefixed_unit_conversions!(Micrometer, Micro, Meter, Length);
    //
    // prefixed_unit_type!(Megagram, Mega, Gram);  // Same as metric ton
    // impl_prefixed_unit_conversions!(Megagram, Mega, Gram, Mass);

    println!("Adding new prefixed units is now just 2 lines of code!");
    println!("No more hardcoded conversion factors!");
}
