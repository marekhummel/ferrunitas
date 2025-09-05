//! Miscellaneous utilities and debugging features.
//!
//! This example showcases various utility functions and debugging capabilities
//! provided by Ferrunitas, along with import patterns for different use cases.
//!
//! Features demonstrated:
//! - **Dimensional Introspection**: Viewing the dimensional signature of quantities and units
//! - **Import Patterns**: Different ways to import and organize Ferrunitas types
//! - **Debugging Tools**: Utilities for understanding and debugging dimensional types
//! - **Custom Definitions**: Quick examples of macro usage
//!
//! This is useful for understanding the internal structure of the type system
//! and for debugging dimensional analysis issues.
//!
//! Run with: `cargo run --example misc`

// For dimensional formatting
use ferrunitas::common::{format_dims, format_unit_dims};
use ferrunitas::system::*;
use ferrunitas::{
    Measure, Unit,
    typenum_consts::{N1, P1},
    unit,
};

fn dimensions() {
    println!("=== Dimensional Introspection ===");
    // If curious, one can print out the dimensions of a quantity / unit
    println!("Force dimensions:           {}", format_dims::<Force>());
    println!("Acceleration dimensions:    {}", format_dims::<Acceleration>());
    println!("Entropy dimensions:         {}", format_dims::<Entropy>());
    println!();
    println!("MetrePerSecond dimensions:  {}", format_unit_dims::<MetrePerSecond>());
    println!("Farad dimensions:           {}", format_unit_dims::<Farad>());
    println!("MolePerLitre dimensions:    {}", format_unit_dims::<MolePerLitre>());
}

fn demonstrate_large_number_precision_issues() {
    // IEEE 754 double precision (f64) has limitations with very large numbers.
    // This affects conversions and arithmetic with big prefixes.
    println!("\n=== Large Number Precision Caveats ===");

    // --Demonstrate precision loss with very large binary prefixes
    println!("--- Prefix Precision Issues ---");
    // Yobibyte is 2^80 bytes, as a power of two this works
    let yobibyte_data = Yobibyte::new(1.0);
    let bit_data = yobibyte_data.convert::<Bit>();
    println!("1 Yobibyte  = {:.0} bits", bit_data.value());
    println!("Expected    = {:.0} bits", (1u128 << 80) as f64 * 8.0);

    // Yottabyte is 10^24 bytes, this will fails
    let yottabyte_data = Yottabyte::new(1.0);
    let bit_data = yottabyte_data.convert::<Bit>();
    println!("1 Yottabyte = {:.0} bits", bit_data.value());
    println!("Expected    = {:.0} bits", 10u128.pow(24) * 8);

    // -- Demonstrate precision loss in arithmetic due to f64
    // At this magnitude a difference of 1 is no longer captured, as the mantissa has only 53 bits
    // Thus, over 2**53 some integers are no longer representable in f64
    println!("\n--- Arithmetic Precision Loss ---");
    let large_data = Exabyte::new(1000.0); // 1 Zettabyte worth
    let small_data = Byte::new(1.0); // 1 Byte
    let sum = large_data + small_data;
    println!("1000 Exabytes + 1 Byte:");
    println!("Result   = {:.0} bytes", sum.convert::<Byte>().value());
    println!("Expected = {:.0} bytes", 10u128.pow(21) + 1);
}

fn offest_limited_usage() {
    println!("\n=== Offset Units Limited Usage ===");
    // Offset units are limited in usage, as arithmetic operations do not make sense as well as compound units
    // (e.g. adding two temperatures in Celsius is not physically meaningful)
    let temp_c = DegreeCelsius::new(25.0);
    let temp_f: Measure<DegreeFahrenheit> = temp_c.convert();
    println!("{:.2} == {:.2}", temp_c, temp_f);

    // All compiles, but beware that this makes sense to you as the user
    let temp_c2 = DegreeCelsius::new(30.0);
    let temp_f2 = DegreeFahrenheit::new(14.0);
    let delta_temp = temp_c2 - temp_f2;
    let delta_k = delta_temp.convert::<Kelvin>();
    println!("Temperature difference: {:.2} K", delta_k.value());

    // When used with compound units, the offset is lost, as we are only talking about temperature differences
    unit!(compound: JoulePerFahrenheit, "J/°F", [(Joule, P1), (DegreeFahrenheit, N1)]);
    let jpk = JoulePerKelvin::new(2.0);
    let jpf = jpk.convert::<JoulePerFahrenheit>();
    println!("{:.2} == {:.2}", jpk, jpf);
}

fn main() {
    dimensions();
    demonstrate_large_number_precision_issues();
    offest_limited_usage();
}
