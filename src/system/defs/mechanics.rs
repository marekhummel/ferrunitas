use crate::system::constants::STANDARD_GRAVITY;
use crate::system::defs::base::*;
use crate::system::defs::dimensionless::*;
use crate::system::defs::thermodynamics::JoulePerKilogramKelvin;
use crate::system::prefixes::*;
use crate::{quantity, unit};
use typenum::*;

// ===========================
// AREA
// ===========================
quantity!(Area: M Z0, L P2, T Z0, I Z0, Th Z0, N Z0, J Z0); // L²

unit!(compound: SquareMetre, "m²", [(Metre, P2)]; prefixable);
unit!(compound: SquareKilometre, "km²", [(Kilometre, P2)]);
unit!(compound: SquareCentimetre, "cm²", [(Centimetre, P2)]);
unit!(compound: SquareMillimetre, "mm²", [(Millimetre, P2)]);

unit!(derived: Are, "a", (100, SquareMetre); prefixable);
unit!(prefix: Hectare, Hecto, Are);

// ===========================
// VOLUME
// ===========================
quantity!(Volume: M Z0, L P3, T Z0, I Z0, Th Z0, N Z0, J Z0); // L³

unit!(compound: CubicMetre, "m³", [(Metre, P3)]);
unit!(compound: Litre, "l", [(Decimetre, P3)]; prefixable);
unit!(prefix: Millilitre, Milli, Litre);
unit!(prefix: Centilitre, Centi, Litre);
unit!(prefix: Decilitre, Deci, Litre);
unit!(compound: CubicCentimetre, "cm³", [(Centimetre, P3)]);
unit!(compound: CubicMillimetre, "mm³", [(Millimetre, P3)]);

// Non SI
unit!(derived: ImperialGallon, "gal", (4.54609, Litre));
unit!(derived: ImperialFluidOunce, "fl oz", (1.0 / 160.0, ImperialGallon));
unit!(derived: ImperialPint, "pt", (20, ImperialFluidOunce));
unit!(derived: ImperialQuart, "qt", (2, ImperialPint));

// Additional volume units
unit!(compound: CubicInch, "in³", [(Inch, P3)]);
unit!(derived: USGallon, "gal", (231, CubicInch));
unit!(derived: USFluidOunce, "fl oz", (1.0 / 128.0, USGallon));
unit!(derived: USTablespoon, "tbsp", (0.5, USFluidOunce));
unit!(derived: USTeaspoon, "tsp", (1.0 / 3.0, USTablespoon));
unit!(derived: USCup, "cup", (8, USFluidOunce));
unit!(derived: USPint, "pt", (2, USCup));
unit!(derived: USQuart, "qt", (2, USPint));

unit!(derived: Barrel, "bbl", (42, USGallon)); // Oil barrel
unit!(compound: BoardFoot, "bd ft", [(Foot, P2), (Inch, P1)]);

// ===========================
// VELOCITY
// ===========================
quantity!(Velocity: M Z0, L P1, T N1, I Z0, Th Z0, N Z0, J Z0); // L T⁻¹

unit!(compound: MetrePerSecond, "m/s", [(Metre, P1), (Second, N1)]);
unit!(compound: KilometrePerHour, "km/h", [(Kilometre, P1), (Hour, N1)]);
unit!(compound: MilePerHour, "mph", [(Mile, P1), (Hour, N1)]);
unit!(compound: Knot, "kn", [(NauticalMile, P1), (Hour, N1)]);

// ===========================
// ACCELERATION
// ===========================
quantity!(Acceleration: M Z0, L P1, T N2, I Z0, Th Z0, N Z0, J Z0); // L T⁻²

unit!(compound: MetrePerSecondSquared, "m/s²", [(Metre, P1), (Second, N2)]);
unit!(compound: Gal, "Gal", [(Centimetre, P1), (Second, N2)]; prefixable);

// ===========================
// WAVE NUMBER
// ===========================
quantity!(WaveNumber: M Z0, L N1, T Z0, I Z0, Th Z0, N Z0, J Z0); // L⁻¹

unit!(compound: ReciprocalMetre, "m⁻¹", [(Metre, N1)]);

// ===========================
// DENSITY
// ===========================
quantity!(Density: M P1, L N3, T Z0, I Z0, Th Z0, N Z0, J Z0); // M L⁻³

unit!(compound: KilogramPerCubicMetre, "kg/m³", [(Kilogram, P1), (CubicMetre, N1)]);
unit!(compound: GramPerCubicCentimetre, "g/cm³", [(Gram, P1), (CubicCentimetre, N1)]);

// ===========================
// SPECIFIC VOLUME
// ===========================
quantity!(SpecificVolume: M N1, L P3, T Z0, I Z0, Th Z0, N Z0, J Z0); // L³ M⁻¹

unit!(compound: CubicMetrePerKilogram, "m³/kg", [(CubicMetre, P1), (Kilogram, N1)]);
unit!(compound: CubicCentimetrePerGram, "cm³/g", [(CubicCentimetre, P1), (Gram, N1)]);

// ===========================
// FORCE
// ===========================
quantity!(Force: M P1, L P1, T N2, I Z0, Th Z0, N Z0, J Z0); // M L T⁻²

unit!(compound: Newton, "N", [(Kilogram, P1), (Metre, P1), (Second, N2)]; prefixable);
unit!(prefix: Kilonewton, Kilo, Newton);
unit!(prefix: Meganewton, Mega, Newton);

unit!(compound: Dyne, "dyn", [(Gram, P1), (Centimetre, P1), (Second, N2)]);
unit!(compound: KilogramForce, "kgf", [(Kilogram, P1), (constant STANDARD_GRAVITY, MetrePerSecondSquared, P1)]);
unit!(compound: PoundForce, "lbf", [(Pound, P1), (constant STANDARD_GRAVITY, MetrePerSecondSquared, P1)]);

// ===========================
// PRESSURE
// ===========================
quantity!(Pressure: M P1, L N1, T N2, I Z0, Th Z0, N Z0, J Z0); // M L⁻¹ T⁻²

unit!(compound: Pascal, "Pa", [(Newton, P1), (Metre, N2)]; prefixable);
unit!(prefix: Kilopascal, Kilo, Pascal);
unit!(prefix: Megapascal, Mega, Pascal);
unit!(prefix: Gigapascal, Giga, Pascal);

unit!(compound: PoundPerSquareInch, "psi", [(PoundForce, P1), (Inch, N2)]);
unit!(compound: PoundPerSquareFoot, "psf", [(PoundForce, P1), (Foot, N2)]);
unit!(compound: KilogramForcePerSquareCentimetre, "kgf/cm²", [(KilogramForce, P1), (Centimetre, N2)]);

unit!(derived: Bar, "bar", (1e5, Pascal));
unit!(derived: Atmosphere, "atm", (101_325, Pascal));
unit!(derived: TechnicalAtmosphere, "at", (1, KilogramForcePerSquareCentimetre));
unit!(derived: Torr, "Torr", (1.0 / 760.0, Atmosphere));

unit!(compound: MillimetreOfMercury, "mmHg", [(13595.1, KilogramPerCubicMetre, P1), (Millimetre, P1), (constant STANDARD_GRAVITY, MetrePerSecondSquared, P1)]);
unit!(compound: InchOfMercury, "inHg", [(13595.1, KilogramPerCubicMetre, P1), (Inch, P1), (constant STANDARD_GRAVITY, MetrePerSecondSquared, P1)]);
unit!(compound: MillimetreOfWater, "mmH2O", [(999.9720, KilogramPerCubicMetre, P1), (Millimetre, P1), (constant STANDARD_GRAVITY, MetrePerSecondSquared, P1)]);
unit!(compound: InchOfWater, "inH2O", [(999.9720, KilogramPerCubicMetre, P1), (Inch, P1), (constant STANDARD_GRAVITY, MetrePerSecondSquared, P1)]);

// ===========================
// ENERGY
// ===========================
quantity!(Energy: M P1, L P2, T N2, I Z0, Th Z0, N Z0, J Z0); // M L² T⁻²

unit!(compound: Joule, "J", [(Newton, P1), (Metre, P1)]; prefixable);
unit!(prefix: Kilojoule, Kilo, Joule);
unit!(prefix: Megajoule, Mega, Joule);
unit!(prefix: Gigajoule, Giga, Joule);

unit!(compound: WattHour, "Wh", [(Watt, P1), (Hour, P1)]; prefixable);
unit!(prefix: KilowattHour, Kilo, WattHour);
unit!(prefix: MegawattHour, Mega, WattHour);
unit!(prefix: GigawattHour, Giga, WattHour);

unit!(derived: Calorie, "cal", (4.184, Joule); prefixable);
unit!(prefix: Kilocalorie, Kilo, Calorie);

unit!(derived: ElectronVolt, "eV", (1.602_176_634e-19, Joule); prefixable);
unit!(prefix: KiloElectronVolt, Kilo, ElectronVolt);
unit!(prefix: MegaElectronVolt, Mega, ElectronVolt);
unit!(prefix: GigaElectronVolt, Giga, ElectronVolt);

unit!(compound: BritishThermalUnit, "BTU", [(4186.8, JoulePerKilogramKelvin, P1), (Pound, P1), (5.0 / 9.0, Kelvin, P1)]; prefixable);
unit!(compound: Erg, "erg", [(Dyne, P1), (Centimetre, P1)]);
unit!(compound: FootPoundForce, "ft⋅lbf", [(Foot, P1), (PoundForce, P1)]);
unit!(derived: Therm, "thm", (1e5, BritishThermalUnit));
unit!(derived: Quad, "quad", (1e15, BritishThermalUnit));

// ===========================
// POWER
// ===========================
quantity!(Power: M P1, L P2, T N3, I Z0, Th Z0, N Z0, J Z0); // M L² T⁻³

unit!(compound: Watt, "W", [(Joule, P1), (Second, N1)]; prefixable);
unit!(prefix: Kilowatt, Kilo, Watt);
unit!(prefix: Megawatt, Mega, Watt);
unit!(prefix: Gigawatt, Giga, Watt);
unit!(prefix: Milliwatt, Milli, Watt);

unit!(compound: Horsepower, "PS", [(75, KilogramForce, P1), (MetrePerSecond, P1)]);
unit!(compound: ImperialHorsepower, "bhp", [(550, FootPoundForce, P1), (Second, N1)]);
// ===========================
// FREQUENCY
// ===========================
quantity!(Frequency: M Z0, L Z0, T N1, I Z0, Th Z0, N Z0, J Z0); // T⁻¹

unit!(compound: Hertz, "Hz", [(Second, N1)]; prefixable);
unit!(prefix: Kilohertz, Kilo, Hertz);
unit!(prefix: Megahertz, Mega, Hertz);
unit!(prefix: Gigahertz, Giga, Hertz);
unit!(prefix: Terahertz, Tera, Hertz);

// ===========================
// ANGULAR VELOCITY
// ===========================
quantity!(AngularVelocity: M Z0, L Z0, T N1, I Z0, Th Z0, N Z0, J Z0); // T⁻¹

unit!(compound: RadianPerSecond, "rad/s", [(Radian, P1), (Second, N1)]);
unit!(compound: DegreePerSecond, "°/s", [(Degree, P1), (Second, N1)]);
unit!(compound: RevolutionPerMinute, "rpm", [(Radian, P1), (Minute, N1)]);

// ===========================
// ANGULAR ACCELERATION
// ===========================
quantity!(AngularAcceleration: M Z0, L Z0, T N2, I Z0, Th Z0, N Z0, J Z0); // T⁻²

unit!(compound: RadianPerSecondSquared, "rad/s²", [(Radian, P1), (Second, N2)]);

// ===========================
// TORQUE / MOMENT
// ===========================
quantity!(Torque: M P1, L P2, T N2, I Z0, Th Z0, N Z0, J Z0); // M L² T⁻²

unit!(compound: NewtonMetre, "N·m", [(Newton, P1), (Metre, P1)]);
unit!(compound: FootPound, "ft·lbf", [(Foot, P1), (PoundForce, P1)]);

// ===========================
// MOMENT OF INERTIA
// ===========================
quantity!(MomentOfInertia: M P1, L P2, T Z0, I Z0, Th Z0, N Z0, J Z0); // M L²

unit!(compound: KilogramSquareMetre, "kg·m²", [(Kilogram, P1), (SquareMetre, P1)]);

// ===========================
// DYNAMIC VISCOSITY
// ===========================
quantity!(DynamicViscosity: M P1, L N1, T N1, I Z0, Th Z0, N Z0, J Z0); // M L⁻¹ T⁻¹

unit!(compound: PascalSecond, "Pa·s", [(Pascal, P1), (Second, P1)]);
unit!(compound: Poise, "P", [(Dyne, P1), (Second, P1), (Centimetre, N2)]; prefixable);
unit!(prefix: Centipoise, Centi, Poise);

// ===========================
// KINEMATIC VISCOSITY
// ===========================
quantity!(KinematicViscosity: M Z0, L P2, T N1, I Z0, Th Z0, N Z0, J Z0); // L² T⁻¹

unit!(compound: SquareMetrePerSecond, "m²/s", [(SquareMetre, P1), (Second, N1)]);
unit!(compound: Stokes, "St", [(SquareCentimetre, P1), (Second, N1)]; prefixable);
unit!(prefix: Centistokes, Centi, Stokes);

// ===========================
// SURFACE TENSION
// ===========================
quantity!(SurfaceTension: M P1, L Z0, T N2, I Z0, Th Z0, N Z0, J Z0); // M T⁻²

unit!(compound: NewtonPerMetre, "N/m", [(Newton, P1), (Metre, N1)]);
unit!(compound: DynePerCentimetre, "dyn/cm", [(Dyne, P1), (Centimetre, N1)]);

// ===========================
// ACTION
// ===========================
quantity!(Action: M P1, L P2, T N1, I Z0, Th Z0, N Z0, J Z0); // M L² T⁻¹

unit!(compound: JouleSecond, "J⋅s", [(Joule, P1), (Second, P1)]);

// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::verify_unit;

    // AREA
    verify_unit!(SquareMetre, Area, 1.0);
    verify_unit!(SquareKilometre, Area, 1e6);
    verify_unit!(SquareCentimetre, Area, 1e-4);
    verify_unit!(SquareMillimetre, Area, 1e-6);
    verify_unit!(Are, Area, 100.0);
    verify_unit!(Hectare, Area, 10000.0);

    // VOLUME
    verify_unit!(CubicMetre, Volume, 1.0);
    verify_unit!(Litre, Volume, 1e-3);
    verify_unit!(Millilitre, Volume, 1e-6);
    verify_unit!(Centilitre, Volume, 1e-5);
    verify_unit!(Decilitre, Volume, 1e-4);
    verify_unit!(CubicCentimetre, Volume, 1e-6);
    verify_unit!(CubicMillimetre, Volume, 1e-9);
    verify_unit!(ImperialGallon, Volume, 4.54609e-3);
    verify_unit!(ImperialFluidOunce, Volume, 2.8412e-5);
    verify_unit!(ImperialPint, Volume, 5.6826e-4);
    verify_unit!(ImperialQuart, Volume, 1.1365e-3);
    verify_unit!(CubicInch, Volume, 1.6387e-5);
    verify_unit!(USGallon, Volume, 3.7854e-3);
    verify_unit!(USFluidOunce, Volume, 2.9574e-5);
    verify_unit!(USTablespoon, Volume, 1.4787e-5);
    verify_unit!(USTeaspoon, Volume, 4.9289e-6);
    verify_unit!(USCup, Volume, 2.3659e-4);
    verify_unit!(USPint, Volume, 4.7318e-4);
    verify_unit!(USQuart, Volume, 9.4635e-4);
    verify_unit!(Barrel, Volume, 0.159);
    verify_unit!(BoardFoot, Volume, 2.3597e-3);

    // VELOCITY
    verify_unit!(MetrePerSecond, Velocity, 1.0);
    verify_unit!(KilometrePerHour, Velocity, 0.2778);
    verify_unit!(MilePerHour, Velocity, 0.4470);
    verify_unit!(Knot, Velocity, 0.5144);

    // ACCELERATION
    verify_unit!(MetrePerSecondSquared, Acceleration, 1.0);
    verify_unit!(Gal, Acceleration, 0.01);

    // WAVE NUMBER
    verify_unit!(ReciprocalMetre, WaveNumber, 1.0);

    // DENSITY
    verify_unit!(KilogramPerCubicMetre, Density, 1.0);
    verify_unit!(GramPerCubicCentimetre, Density, 1e3);

    // SPECIFIC VOLUME
    verify_unit!(CubicMetrePerKilogram, SpecificVolume, 1.0);
    verify_unit!(CubicCentimetrePerGram, SpecificVolume, 1e-3);

    // FORCE
    verify_unit!(Newton, Force, 1.0);
    verify_unit!(Kilonewton, Force, 1e3);
    verify_unit!(Meganewton, Force, 1e6);
    verify_unit!(Dyne, Force, 1e-5);
    verify_unit!(KilogramForce, Force, 9.80665);
    verify_unit!(PoundForce, Force, 4.44822);

    // PRESSURE
    verify_unit!(Pascal, Pressure, 1.0);
    verify_unit!(Kilopascal, Pressure, 1e3);
    verify_unit!(Megapascal, Pressure, 1e6);
    verify_unit!(Gigapascal, Pressure, 1e9);
    verify_unit!(Bar, Pressure, 1e5);
    verify_unit!(Atmosphere, Pressure, 1.01325e5);
    verify_unit!(TechnicalAtmosphere, Pressure, 9.80665e4);
    verify_unit!(Torr, Pressure, 133.322368);
    verify_unit!(MillimetreOfMercury, Pressure, 133.322_387_415);
    verify_unit!(MillimetreOfWater, Pressure, 9.80665);
    verify_unit!(InchOfMercury, Pressure, 3.386389e3);
    verify_unit!(InchOfWater, Pressure, 2.490889e2);
    verify_unit!(PoundPerSquareInch, Pressure, 6.8947573e3);
    verify_unit!(PoundPerSquareFoot, Pressure, 4.788025899e1);
    verify_unit!(KilogramForcePerSquareCentimetre, Pressure, 9.80665e4);

    // ENERGY
    verify_unit!(Joule, Energy, 1.0);
    verify_unit!(Kilojoule, Energy, 1e3);
    verify_unit!(Megajoule, Energy, 1e6);
    verify_unit!(Gigajoule, Energy, 1e9);
    verify_unit!(WattHour, Energy, 3.6e3);
    verify_unit!(KilowattHour, Energy, 3.6e6);
    verify_unit!(MegawattHour, Energy, 3.6e9);
    verify_unit!(GigawattHour, Energy, 3.6e12);
    verify_unit!(Calorie, Energy, 4.184);
    verify_unit!(Kilocalorie, Energy, 4.184e3);
    verify_unit!(ElectronVolt, Energy, 1.602176634e-19);
    verify_unit!(KiloElectronVolt, Energy, 1.602176634e-16);
    verify_unit!(MegaElectronVolt, Energy, 1.602176634e-13);
    verify_unit!(GigaElectronVolt, Energy, 1.602176634e-10);
    verify_unit!(BritishThermalUnit, Energy, 1.05505585e3);
    verify_unit!(Erg, Energy, 1e-7);
    verify_unit!(FootPoundForce, Energy, 1.3558);
    verify_unit!(Therm, Energy, 1.05505585262e8);
    verify_unit!(Quad, Energy, 1.05505585262e18);

    // POWER
    verify_unit!(Watt, Power, 1.0);
    verify_unit!(Kilowatt, Power, 1e3);
    verify_unit!(Megawatt, Power, 1e6);
    verify_unit!(Gigawatt, Power, 1e9);
    verify_unit!(Milliwatt, Power, 1e-3);
    verify_unit!(Horsepower, Power, 735.49875);
    verify_unit!(ImperialHorsepower, Power, 745.699871);

    // FREQUENCY
    verify_unit!(Hertz, Frequency, 1.0);
    verify_unit!(Kilohertz, Frequency, 1000.0);
    verify_unit!(Megahertz, Frequency, 1e6);
    verify_unit!(Gigahertz, Frequency, 1e9);
    verify_unit!(Terahertz, Frequency, 1e12);

    // ANGULAR VELOCITY
    verify_unit!(RadianPerSecond, AngularVelocity, 1.0);
    verify_unit!(DegreePerSecond, AngularVelocity, 0.01745329251);
    verify_unit!(RevolutionPerMinute, AngularVelocity, 0.01666667);

    // ANGULAR ACCELERATION
    verify_unit!(RadianPerSecondSquared, AngularAcceleration, 1.0);

    // TORQUE
    verify_unit!(NewtonMetre, Torque, 1.0);
    verify_unit!(FootPound, Torque, 1.3558);

    // MOMENT OF INERTIA
    verify_unit!(KilogramSquareMetre, MomentOfInertia, 1.0);

    // DYNAMIC VISCOSITY
    verify_unit!(PascalSecond, DynamicViscosity, 1.0);
    verify_unit!(Poise, DynamicViscosity, 1e-1);
    verify_unit!(Centipoise, DynamicViscosity, 1e-3);

    // KINEMATIC VISCOSITY
    verify_unit!(SquareMetrePerSecond, KinematicViscosity, 1.0);
    verify_unit!(Stokes, KinematicViscosity, 1e-4);
    verify_unit!(Centistokes, KinematicViscosity, 1e-6);

    // SURFACE TENSION
    verify_unit!(NewtonPerMetre, SurfaceTension, 1.0);
    verify_unit!(DynePerCentimetre, SurfaceTension, 1e-3);

    // ACTION
    verify_unit!(JouleSecond, Action, 1.0);
}
