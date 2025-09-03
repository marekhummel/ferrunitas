//! Basic usage examples for the Ferrunitas unit conversion library.
//!
//! This example demonstrates the fundamental concepts and operations in Ferrunitas:
//!
//! - **Units, Measures, and Quantities**: Understanding the three core types
//! - **Unit Conversions**: Multiple ways to convert between compatible units
//! - **Interoperability**: How measures and quantities work together
//! - **Cross-unit Comparisons**: Comparing values in different units
//! - **Arithmetic Operations**: Computing with both measures and quantities
//!
//! Run with: `cargo run --example basics`

use ferrunitas::system::*;
use ferrunitas::{Measure, Unit, common::format_unit_dims};

fn units_measures_and_quantities() {
    println!("--- Units, Quantities and Measures Example ---");

    // -- Units
    // Units are zero-sized-types with a fixed scale factor to the SI base unit.
    // Its main use is for creating measures, instantiation is irrelevant
    println!("Unit instance: '{:?}' '{}'", Foot, Foot);
    // However they hold information regarding their dimension vector (see examples/misc.rs)
    println!("Dimension of unit: {}", format_unit_dims::<Foot>());
    println!();

    // -- Measures
    // To express units with a value, use measures
    let l1: Measure<Foot> = Foot::new(402);
    let l2: Measure<Centimetre> = Centimetre::new(30);
    let l3: Measure<Inch> = Measure::new(6);
    let l_sum: Measure<Foot> = l1 + l2 + l3;
    println!("1st Measure in Foot: {}", l1);
    println!("2nd Measure in Centimetres: {}", l2);
    println!("3rd Measure in Inches: {}", l3);
    println!("Sum Measure in Foot (LHS unit): {}", l_sum);
    println!();

    // -- Quantities
    // Quantities are types holding a value and its dimensional information.
    // They can be created from measures, or as a result from multiplication, division or exponentiation (see below).
    let v1: Length = l1.into_q();
    let v2: Length = l2.into_q();
    // Quantities have no unit and only an inner value, which is meaningless outside the lib.
    println!("Two measures as quantities: {} and {}", v1, v2);
    println!();
}

fn conversion() {
    let ltr = Litre::new(30.64);
    let volume = ltr.into_q();
    let cm3_via_q1 = volume.as_measure::<CubicCentimetre>();
    let cm3_via_q2: Measure<CubicCentimetre> = volume.as_measure();
    let cm3_direct1 = ltr.convert::<CubicCentimetre>();
    let cm3_direct2: Measure<CubicCentimetre> = ltr.convert();
    let cm3_oneline = Volume::convert::<Litre, CubicCentimetre>(30.64);

    println!("--- Conversion Example ---");
    println!("Litres:                                              {}", ltr);
    println!("Litres to Quantity:                                  {:.6}", volume);
    println!("Litres via Quantity to Cubic Centimetres (method 1): {:.2}", cm3_via_q1);
    println!("Litres via Quantity to Cubic Centimetres (method 2): {:.2}", cm3_via_q2);
    println!("Litres to Cubic Centimetres (direct 1):              {:.2}", cm3_direct1);
    println!("Litres to Cubic Centimetres (direct 2):              {:.2}", cm3_direct2);
    println!("Volume convert in one line:                          {:.2}", cm3_oneline);
    println!("* Final Result: {:.2} == {:.2}", ltr, cm3_direct2);
    println!();
}

fn interop_unit_quantity() {
    let t: Measure<Tonne> = Tonne::new(2.0); // Mass as unit
    let t_q: Mass = t.into_q(); // Mass as quantity
    let a = MetrePerSecondSquared::new(9.81); // Acceleration as unit
    let a_q = a.into_q(); // Acceleration as quantity
    println!("--- Interop Unit and Quantity Example ---");

    // Add / subtract in either fashion, result type matches LHS !
    let t2 = Tonne::new(8.0);
    let t2_q = t2.into_q();
    let t_sum1: Measure<Tonne> = t + t2;
    let t_sum2: Mass = t_q + t2;
    let t_sum3: Measure<Tonne> = t + t2_q;
    let t_sum4: Mass = t_q + t2_q;
    println!("{} == {}   and  {} == {}", t_sum1, t_sum3, t_sum2, t_sum4);
    println!("Also: {} == {}", t_sum1, t_sum2.as_measure::<Tonne>());

    // Multiplication and division (unless scalar) always results in a quantity,
    // but works in any combination of unit and quantity
    let f_q1: Force = t * a;
    let f_q2: Force = t_q * a;
    let f_q3: Force = t * a_q;
    let f_q4: Force = t_q * a_q;
    println!("{} == {} == {} == {}", f_q1, f_q2, f_q3, f_q4);
    println!();
}

fn comparison() {
    let f = Newton::new(10.0);
    let d = Metre::new(2.0);
    let p = Watt::new(20.0);
    let t = Second::new(1.0);

    let e1: Energy = f * d;
    let e2: Energy = p * t;
    let ftlbf = e1.as_measure::<FootPoundForce>();
    let kj = e2.as_measure::<Kilojoule>();

    // Note that Ord and Eq traits are kept as expected (type and inner fields match)
    // For cross-unit equality use this method.
    let are_equal = ftlbf.is_equal_to(&kj);

    println!("--- Comparison Example ---");
    println!("Energy quantities: {} and {}", e1, e2);
    println!("Results: {} and {}, Equal?: {}", ftlbf, kj, are_equal);
    println!();
}

fn computation_quantities() {
    // Work with quantities
    let mut force: Force = Newton::new(10.0).into_q();
    let mut distance: Length = Length::from::<Metre>(5.0);
    let mut time: Time = Second::new(2.0).into_q();

    // Modify variables a bit (can add / sub quantities or units)
    force += Kilonewton::new(2.0).into_q();
    force += Kilonewton::new(1.0);
    distance -= Centimetre::new(3.0).into_q();
    distance /= 2.0;
    time += Millisecond::new(50.0).into_q();
    time *= 1.5;

    // Compute work and power.
    let work: Energy = force * distance; // W = F*d
    let power: Power = work / time; // P = W/t

    // Output
    println!("--- Computation Example (Quantities) ---");
    println!("Force: {:.3}", force);
    println!("Distance: {:.3}", distance);
    println!("Time: {:.3}", time);
    println!("Work: {:.3}", work);
    println!("Power: {:.3} (as unit: {:.3})", power, power.as_measure::<Kilowatt>());
    println!();
}

fn computation_units() {
    // Define base variable
    let mut newton = Newton::new(10.0);
    let mut metre = Metre::new(5.0) - Millimetre::new(30.0);
    let mut second = Second::new(2.0) + Minute::new(0.1);

    // Modify variables a bit (directly as units), can add / sub quantities or units
    newton += Kilonewton::new(2.0);
    newton += Force::from::<Kilonewton>(1.0);
    metre -= Centimetre::new(3.0);
    metre /= 2.0;
    second += Millisecond::new(50.0);
    second *= 1.5;

    // Compute work and power. Note that any multiplication or division of units with other units or
    // quantities will result in a quantity and requires explicit conversion into a unit.
    let work: Energy = newton * metre; // W = F*d
    let power: Power = work / second; // P = W/t

    println!("--- Computation Example (Units) ---");
    println!("Newtons: {:.3}", newton);
    println!("Metres: {:.3}", metre);
    println!("Seconds: {:.3}", second);
    println!("Kilojoules: {:.3}", work.as_measure::<Kilojoule>());
    println!("Kilowatts: {:.6}", power.as_measure::<Kilowatt>());
    println!();
}

fn main() {
    units_measures_and_quantities();

    conversion();

    interop_unit_quantity();

    comparison();

    computation_quantities();

    computation_units();
}
