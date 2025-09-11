//! This file is only for internal testing purposes.
//! It is not linked in the crate and thus has no meaning to users.

#![allow(dead_code, unused_variables)]

extern crate std;
use std::*;

use crate::model::unit::Unit;
use crate::{prefix, quantity, unit};
use typenum::*;

// Prefixes
prefix!(Pico, 1e-12, "p");
prefix!(Nano, 1e-9, "n");
prefix!(Micro, 1e-6, "μ");
prefix!(Milli, 1e-3, "m");
prefix!(Centi, 1e-2, "c");

prefix!(Kilo, 1e3, "k");
prefix!(Mega, 1e6, "M");
prefix!(Exa, 1e18, "E");
prefix!(Yotta, 1e24, "Y");
prefix!(Yobi, 1u128 << 80, "Yi");

// Quantities
quantity!(Mass: M P1, L Z0, T Z0, I Z0, Th Z0, N Z0, J Z0);
quantity!(Length: M Z0, L P1, T Z0, I Z0, Th Z0, N Z0, J Z0);
quantity!(Time: M Z0, L Z0, T P1, I Z0, Th Z0, N Z0, J Z0);
quantity!(Velocity: M Z0, L P1, T N1, I Z0, Th Z0, N Z0, J Z0);
quantity!(ElectricCurrent: M Z0, L Z0, T Z0, I P1, Th Z0, N Z0, J Z0);
quantity!(Temperature: M Z0, L Z0, T Z0, I Z0, Th P1, N Z0, J Z0);
quantity!(AmountOfSubstance: M Z0, L Z0, T Z0, I Z0, Th Z0, N P1, J Z0);
quantity!(LuminousIntensity: M Z0, L Z0, T Z0, I Z0, Th Z0, N Z0, J P1);
quantity!(Volume: M Z0, L P3, T Z0, I Z0, Th Z0, N Z0, J Z0); // L³
quantity!(Acceleration: M Z0, L P1, T N2, I Z0, Th Z0, N Z0, J Z0); // L T⁻²
quantity!(Force: M P1, L P1, T N2, I Z0, Th Z0, N Z0, J Z0); // M L T⁻²
quantity!(Pressure: M P1, L N1, T N2, I Z0, Th Z0, N Z0, J Z0); // M L⁻¹ T⁻²
quantity!(Energy: M P1, L P2, T N2, I Z0, Th Z0, N Z0, J Z0); // M L² T⁻²
quantity!(Power: M P1, L P2, T N3, I Z0, Th Z0, N Z0, J Z0); // M L² T⁻³
quantity!(Entropy: M P1, L P2, T N2, I Z0, Th N1, N Z0, J Z0); // M L² T⁻² Θ⁻¹
quantity!(Dimensionless: M Z0, L Z0, T Z0, I Z0, Th Z0, N Z0, J Z0); // Dimensionless
quantity!(Angle: M Z0, L Z0, T Z0, I Z0, Th Z0, N Z0, J Z0; marked); // Dimensionless
quantity!(SolidAngle: M Z0, L Z0, T Z0, I Z0, Th Z0, N Z0, J Z0; marked); // Dimensionless
quantity!(Information: M Z0, L Z0, T Z0, I Z0, Th Z0, N Z0, J Z0; marked); // Dimensionless

// Units
unit!(base: Gram, "g", Mass; prefixable, factor = 0.001);
unit!(prefix: Kilogram, Kilo, Gram);
unit!(prefix: Milligram, Milli, Gram);
unit!(derived: Tonne, "t", (1000, Kilogram); prefixable);
unit!(derived: Ounce, "oz", (28.349_523_125, Gram));
unit!(derived: Pound, "lb", (16, Ounce));

unit!(base: Metre, "m", Length; prefixable);
unit!(prefix: Kilometre, Kilo, Metre);
unit!(prefix: Centimetre, Centi, Metre);
unit!(prefix: Millimetre, Milli, Metre);
unit!(derived: Inch, "in", (2.54, Centimetre));
unit!(derived: Foot, "ft", (12, Inch));
unit!(derived: NauticalMile, "NM", (1852, Metre); prefixable);

unit!(base: Second, "s", Time; prefixable);
unit!(prefix: Millisecond, Milli, Second);
unit!(derived: Minute, "min", (60, Second));
unit!(derived: Hour, "h", (60, Minute));

unit!(compound: MetrePerSecond, "m/s", [(Metre, P1), (Second, N1)]);
unit!(compound: KilometrePerHour, "km/h", [(Kilometre, P1), (Hour, N1)]);
unit!(compound: Knot, "kn", [(NauticalMile, P1), (Hour, N1)]);

unit!(base: Ampere, "A", ElectricCurrent; prefixable);
unit!(base: Mole, "mol", AmountOfSubstance; prefixable);
unit!(base: Candela, "cd", LuminousIntensity; prefixable);
unit!(base: Kelvin, "K", Temperature; prefixable);
unit!(derived: DegreeCelsius, "°C", (1.0, 273.15, Kelvin));
unit!(derived: DegreeFahrenheit, "°F", (5.0 / 9.0, -32.0 * 5.0/9.0, DegreeCelsius));

unit!(compound: SquareMetre, "m²", [(Metre, P2)]; prefixable);
unit!(compound: SquareCentimetre, "cm²", [(Centimetre, P2)]);
unit!(compound: CubicMetre, "m³", [(Metre, P3)]);
unit!(derived: Litre, "L", (0.001, CubicMetre); prefixable);
unit!(compound: CubicCentimetre, "cm³", [(Centimetre, P3)]);
unit!(compound: MetrePerSecondSquared, "m/s²", [(Metre, P1), (Second, N2)]);
unit!(compound: Newton, "N", [(Kilogram, P1), (Metre, P1), (Second, N2)]; prefixable);
unit!(prefix: Kilonewton, Kilo, Newton);
unit!(compound: Pascal, "Pa", [(Newton, P1), (Metre, N2)]; prefixable);
unit!(compound: Joule, "J", [(Newton, P1), (Metre, P1)]; prefixable);
unit!(prefix: Kilojoule, Kilo, Joule);
unit!(derived: FootPoundForce, "ft⋅lbf", (1.355_817_948, Joule));
unit!(compound: Watt, "W", [(Joule, P1), (Second, N1)]; prefixable);
unit!(prefix: Milliwatt, Milli, Watt);
unit!(prefix: Kilowatt, Kilo, Watt);
unit!(derived: Horsepower, "PS", (0.735_498_75, Kilowatt));

unit!(compound: Coulomb, "C", [(Second, P1), (Ampere, P1)]; prefixable);
unit!(compound: Volt, "V", [(Watt, P1), (Ampere, N1)]; prefixable);
unit!(compound: Farad, "F", [(Coulomb, P1), (Volt, N1)]; prefixable);
unit!(compound: Ohm, "Ω", [(Volt, P1), (Ampere, N1)]; prefixable);

unit!(compound: MolePerLitre, "mol/L", [(Mole, P1), (Litre, N1)]);
unit!(compound: JoulePerKelvin, "J/K", [(Joule, P1), (Kelvin, N1)]);
unit!(compound: KilogramPerCubicMetre, "kg/m³", [(Kilogram, P1), (CubicMetre, N1)]);
unit!(compound: ReciprocalMetre, "m⁻¹", [(Metre, N1)]);
unit!(compound: Hertz, "Hz", [(Second, N1)]; prefixable);

unit!(base: Radian, "rad", Angle);
unit!(compound: Steradian, "sr", [(Radian, P2)]; marked SolidAngle);
unit!(base: Bit, "bit", Information; prefixable);
unit!(derived: Byte, "B", (8, Bit); prefixable);
unit!(prefix: Exabyte, Exa, Byte);
unit!(prefix: Yottabyte, Yotta, Byte);
unit!(prefix: Yobibyte, Yobi, Byte);

fn computation_units() {
    // Define base variable
    let mut metre: crate::Measure<Metre> = Metre::new(5.0) - Millimetre::new(30.0);
    let mut second: crate::Measure<Second> = Second::new(2.0) + Minute::new(0.1);

    // Modify variables a bit (directly as units)
    metre += Centimetre::new(7.0);
    metre -= Centimetre::new(3.0);
    metre /= 2.0;
    second += Millisecond::new(50.0);
    second *= 1.5;

    // Compute work and power. Note that any multiplication or division of units with other units or
    // quantities will result in a quantity and requires explicit conversion into a unit.
    let speed = metre / second; // v = d/t

    println!("--- Computation Example (Units) ---");
    println!("Metres: {:.3}", metre);
    println!("Seconds: {:.3}", second);
    println!(
        "Speed: {:.3} = {:.3}",
        speed,
        speed.as_measure::<KilometrePerHour>()
    );
}

fn main() {
    computation_units();
}
