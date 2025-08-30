#![allow(dead_code, unused_variables)]

use crate::model::macros::{prefix, unit};
use crate::model::quantity::Quantity;
use crate::model::unit::Unit;
use typenum::*;

// Prefixes
prefix!(Pico, 1e-12, "p");
prefix!(Nano, 1e-9, "n");
prefix!(Micro, 1e-6, "μ");
prefix!(Milli, 1e-3, "m");
prefix!(Centi, 1e-2, "c");

prefix!(Kilo, 1e3, "k");
prefix!(Mega, 1e6, "M");

// MASS
pub type Mass = Quantity<P1, Z0, Z0, Z0, Z0, Z0, Z0>;

unit!(base: Gram, "g", Mass; prefixable, factor = 0.001,);
unit!(prefix: Kilogram, Kilo, Gram);
unit!(prefix: Milligram, Milli, Gram);
unit!(derived: Tonne, "t", (1000, Kilogram); prefixable);
unit!(derived: Ounce, "oz", (28.349_523_125, Gram));
unit!(derived: Pound, "lb", (16, Ounce));
unit!(derived: Stone, "st", (14, Pound));

// LENGTH
pub type Length = Quantity<Z0, P1, Z0, Z0, Z0, Z0, Z0>;

unit!(base: Metre, "m", Length; prefixable,);

unit!(prefix: Kilometre, Kilo, Metre);
unit!(prefix: Centimetre, Centi, Metre);
unit!(prefix: Millimetre, Milli, Metre);

unit!(derived: Inch, "in", (2.54, Centimetre));
unit!(derived: Foot, "ft", (12, Inch));
unit!(derived: Yard, "yd", (3, Foot));
unit!(derived: Mile, "mi", (1760, Yard));

unit!(derived: NauticalMile, "NM", (1852, Metre); prefixable);

// Time
pub type Time = Quantity<Z0, Z0, P1, Z0, Z0, Z0, Z0>;

unit!(base: Second, "s", Time; prefixable,);

unit!(prefix: Millisecond, Milli, Second);

unit!(derived: Minute, "min", (60, Second));
unit!(derived: Hour, "h", (60, Minute));
unit!(derived: Day, "d", (24, Hour));

// Speed
pub type Velocity = Quantity<Z0, P1, N1, Z0, Z0, Z0, Z0>; // L T⁻¹

unit!(compound: MetrePerSecond, "m/s", [(Metre, P1), (Second, N1)]);
unit!(compound: KilometrePerHour, "km/h", [(Kilometre, P1), (Hour, N1)]);
unit!(compound: MilePerHour, "mph", [(Mile, P1), (Hour, N1)]);
unit!(compound: Knot, "kn", [(NauticalMile, P1), (Hour, N1)]);

fn computation_quantities() {
    // Define inputs
    let metre = Metre::new(5.0);
    let millimetre = Millimetre::new(30.0);
    let second = Second::new(2.0);
    let minute = Minute::new(0.1);

    // Work with quantities
    let mut distance: Length = metre.into_q() + millimetre.into_q();
    let mut time: Time = second.into_q() + minute.into_q();

    // Modify variables a bit
    distance += Centimetre::new(7.0).into_q();
    distance -= Centimetre::new(3.0).into_q();
    distance /= 2.0;
    time += Millisecond::new(50.0).into_q();
    time *= 1.5;

    // Compute work and power.
    let whatever = distance * time;
    let speed: Velocity = distance / time; // v = d/t

    // Output
    println!("--- Computation Example (Quantities) ---");
    println!("Distance: {:.3}", distance);
    println!("Time: {:.3}", time);
    println!(
        "Speed: {:.3} (as unit: {:.3})",
        speed,
        speed.as_unit::<KilometrePerHour>()
    );
    println!();
}

fn computation_units() {
    // Define base variable
    let mut metre = Metre::new(5.0) - Millimetre::new(30.0);
    let mut second = Second::new(2.0) + Minute::new(0.1);

    // Modify variables a bit (directly as units)
    metre += Centimetre::new(7.0);
    metre -= Centimetre::new(3.0);
    metre /= 2.0;
    second += Millisecond::new(50.0);
    second *= 1.5;

    // Compute work and power. Note that any multiplication or division of units with other units or
    // quantities will result in a quantity and requires explicit conversion into a unit.
    let speed: Velocity = metre / second; // v = d/t

    println!("--- Computation Example (Units) ---");
    println!("Metres: {:.3}", metre);
    println!("Seconds: {:.3}", second);
    println!(
        "Speed: {:.3} = {:.3}",
        speed,
        speed.as_unit::<KilometrePerHour>()
    );
}

fn main() {
    computation_quantities();
    computation_units();
}
