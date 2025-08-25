use crate::model::quantity::Quantity;
use crate::model::unit::Unit;
use crate::system::defs::base::*;
use crate::system::defs::dimensionless::*;
use crate::system::prefixes::*;
use crate::unit;
use typenum::*;

// ============================================================================
// AREA
// ============================================================================
pub type Area = Quantity<Z0, P2, Z0, Z0, Z0, Z0, Z0>; // L²

unit!(compound: SquareMetre, "m²", [(Metre, P2)], prefixable);
unit!(compound: SquareKilometre, "km²", [(Kilometre, P2)]);
unit!(compound: SquareCentimetre, "cm²", [(Centimetre, P2)]);
unit!(compound: SquareMillimetre, "mm²", [(Millimetre, P2)]);

unit!(derived: Are, "a", (100, SquareMetre), prefixable);
unit!(prefix: Hectare, Hecto, Are);

// ============================================================================
// VOLUME
// ============================================================================
pub type Volume = Quantity<Z0, P3, Z0, Z0, Z0, Z0, Z0>; // L³

unit!(compound: CubicMetre, "m³", [(Metre, P3)]);
unit!(compound: Litre, "L", [(Decimetre, P3)], prefixable);
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

// ============================================================================
// VELOCITY
// ============================================================================
pub type Velocity = Quantity<Z0, P1, N1, Z0, Z0, Z0, Z0>; // L T⁻¹

unit!(compound: MetrePerSecond, "m/s", [(Metre, P1), (Second, N1)]);
unit!(compound: KilometrePerHour, "km/h", [(Kilometre, P1), (Hour, N1)]);
unit!(compound: MilePerHour, "mph", [(Mile, P1), (Hour, N1)]);
unit!(compound: Knot, "kn", [(NauticalMile, P1), (Hour, N1)]);

// ============================================================================
// ACCELERATION
// ============================================================================
pub type Acceleration = Quantity<Z0, P1, N2, Z0, Z0, Z0, Z0>; // L T⁻²

unit!(compound: MetrePerSecondSquared, "m/s²", [(Metre, P1), (Second, N2)]);
unit!(compound: Gal, "Gal", [(Centimetre, P1), (Second, N2)], prefixable);

// ============================================================================
// WAVE NUMBER
// ============================================================================
pub type WaveNumber = Quantity<Z0, N1, Z0, Z0, Z0, Z0, Z0>; // L⁻¹

unit!(compound: Dioptre, "dpt", [(Metre, N1)], prefixable);

// ============================================================================
// DENSITY
// ============================================================================
pub type Density = Quantity<P1, N3, Z0, Z0, Z0, Z0, Z0>; // M L⁻³

unit!(compound: KilogramPerCubicMetre, "kg/m³", [(Kilogram, P1), (CubicMetre, N1)]);
unit!(compound: GramPerCubicCentimetre, "g/cm³", [(Gram, P1), (CubicCentimetre, N1)]);

// ============================================================================
// SPECIFIC VOLUME
// ============================================================================
pub type SpecificVolume = Quantity<N1, P3, Z0, Z0, Z0, Z0, Z0>; // L³ M⁻¹

unit!(compound: CubicMetrePerKilogram, "m³/kg", [(CubicMetre, P1), (Kilogram, N1)]);
unit!(compound: CubicCentimetrePerGram, "cm³/g", [(CubicCentimetre, P1), (Gram, N1)]);

// ============================================================================
// FORCE
// ============================================================================
pub type Force = Quantity<P1, P1, N2, Z0, Z0, Z0, Z0>; // M L T⁻²

unit!(compound: Newton, "N", [(Kilogram, P1), (Metre, P1), (Second, N2)], prefixable);
unit!(prefix: Kilonewton, Kilo, Newton);
unit!(prefix: Meganewton, Mega, Newton);

unit!(compound: Dyne, "dyn", [(Gram, P1), (Centimetre, P1), (Second, N2)]);
unit!(derived: Kilopond, "kp", (9.80665, Newton));
unit!(derived: PoundForce, "lbf", (4.448_221_615_260_5, Newton));

// ============================================================================
// PRESSURE
// ============================================================================
pub type Pressure = Quantity<P1, N1, N2, Z0, Z0, Z0, Z0>; // M L⁻¹ T⁻²

unit!(compound: Pascal, "Pa", [(Newton, P1), (Metre, N2)], prefixable);
unit!(prefix: Kilopascal, Kilo, Pascal);
unit!(prefix: Megapascal, Mega, Pascal);
unit!(prefix: Gigapascal, Giga, Pascal);

unit!(derived: Bar, "bar", (1e5, Pascal));
unit!(derived: Atmosphere, "atm", (101_325, Pascal));
unit!(derived: TechnicalAtmosphere, "at", (1, KilogramForcePerSquareCentimetre));

unit!(derived: Torr, "Torr", (1.0 / 760.0, Atmosphere));
unit!(derived: MillimetreOfMercury, "mmHg", (133.322_387_415, Pascal));
unit!(derived: MillimetreOfWater, "mmH2O", (9.80665, Pascal));
unit!(derived: InchOfMercury, "inHg", (3386.389, Pascal));
unit!(derived: InchOfWater, "inH2O", (249.0889, Pascal));

unit!(compound: PoundPerSquareInch, "psi", [(PoundForce, P1), (Inch, N2)]);
unit!(compound: PoundPerSquareFoot, "psf", [(PoundForce, P1), (Foot, N2)]);
unit!(compound: KilogramForcePerSquareCentimetre, "kgf/cm²", [(Kilopond, P1), (Centimetre, N2)]);

// ============================================================================
// ENERGY
// ============================================================================
pub type Energy = Quantity<P1, P2, N2, Z0, Z0, Z0, Z0>; // M L² T⁻²

unit!(compound: Joule, "J", [(Newton, P1), (Metre, P1)], prefixable);
unit!(prefix: Kilojoule, Kilo, Joule);
unit!(prefix: Megajoule, Mega, Joule);
unit!(prefix: Gigajoule, Giga, Joule);

unit!(compound: WattHour, "Wh", [(Watt, P1), (Hour, P1)], prefixable);

unit!(derived: Calorie, "cal", (4.184, Joule), prefixable);
unit!(prefix: Kilocalorie, Kilo, Calorie);

unit!(derived: ElectronVolt, "eV", (1.602_176_634e-19, Joule));
unit!(derived: BritishThermalUnit, "BTU", (1055.05585262, Joule), prefixable);
unit!(compound: Erg, "erg", [(Dyne, P1), (Centimetre, P1)]);
unit!(compound: FootPoundForce, "ft⋅lbf", [(Foot, P1), (PoundForce, P1)]);
unit!(derived: Therm, "thm", (1e5, BritishThermalUnit));
unit!(derived: Quad, "quad", (1e15, BritishThermalUnit));

// ============================================================================
// POWER
// ============================================================================
pub type Power = Quantity<P1, P2, N3, Z0, Z0, Z0, Z0>; // M L² T⁻³

unit!(compound: Watt, "W", [(Joule, P1), (Second, N1)], prefixable);
unit!(prefix: Kilowatt, Kilo, Watt);
unit!(prefix: Megawatt, Mega, Watt);
unit!(prefix: Gigawatt, Giga, Watt);
unit!(prefix: Milliwatt, Milli, Watt);

unit!(derived: Horsepower, "hp", (735.499, Watt)); // Try to use advanced compound unit?

// ============================================================================
// FREQUENCY
// ============================================================================
pub type Frequency = Quantity<Z0, Z0, N1, Z0, Z0, Z0, Z0>; // T⁻¹

unit!(compound: Hertz, "Hz", [(Second, N1)], prefixable);
unit!(prefix: Kilohertz, Kilo, Hertz);
unit!(prefix: Megahertz, Mega, Hertz);
unit!(prefix: Gigahertz, Giga, Hertz);
unit!(prefix: Terahertz, Tera, Hertz);

// ============================================================================
// ANGULAR VELOCITY
// ============================================================================
pub type AngularVelocity = Quantity<Z0, Z0, N1, Z0, Z0, Z0, Z0>; // T⁻¹

unit!(compound: RadianPerSecond, "rad/s", [(Radian, P1), (Second, N1)]);
unit!(compound: DegreePerSecond, "°/s", [(Degree, P1), (Second, N1)]);
unit!(compound: RevolutionPerMinute, "rpm", [(Radian, P1), (Minute, N1)]);

// ============================================================================
// ANGULAR ACCELERATION
// ============================================================================
pub type AngularAcceleration = Quantity<Z0, Z0, N2, Z0, Z0, Z0, Z0>; // T⁻²

unit!(compound: RadianPerSecondSquared, "rad/s²", [(Radian, P1), (Second, N2)]);

// ============================================================================
// TORQUE / MOMENT
// ============================================================================
pub type Torque = Quantity<P1, P2, N2, Z0, Z0, Z0, Z0>; // M L² T⁻²

unit!(compound: NewtonMetre, "N·m", [(Newton, P1), (Metre, P1)]);
unit!(compound: FootPound, "ft·lbf", [(Foot, P1), (PoundForce, P1)]);

// ============================================================================
// MOMENT OF INERTIA
// ============================================================================
pub type MomentOfInertia = Quantity<P1, P2, Z0, Z0, Z0, Z0, Z0>; // M L²

unit!(compound: KilogramSquareMetre, "kg·m²", [(Kilogram, P1), (SquareMetre, P1)]);

// ============================================================================
// DYNAMIC VISCOSITY
// ============================================================================
pub type DynamicViscosity = Quantity<P1, N1, N1, Z0, Z0, Z0, Z0>; // M L⁻¹ T⁻¹

unit!(compound: PascalSecond, "Pa·s", [(Pascal, P1), (Second, P1)]);
unit!(compound: Poise, "P", [(Dyne, P1), (Second, P1), (Centimetre, N2)], prefixable);
unit!(prefix: Centipoise, Centi, Poise);

// ============================================================================
// KINEMATIC VISCOSITY
// ============================================================================
pub type KinematicViscosity = Quantity<Z0, P2, N1, Z0, Z0, Z0, Z0>; // L² T⁻¹

unit!(compound: SquareMetrePerSecond, "m²/s", [(SquareMetre, P1), (Second, N1)]);
unit!(compound: Stokes, "St", [(SquareCentimetre, P1), (Second, N1)], prefixable);
unit!(prefix: Centistokes, Centi, Stokes);

// ============================================================================
// SURFACE TENSION
// ============================================================================
pub type SurfaceTension = Quantity<P1, Z0, N2, Z0, Z0, Z0, Z0>; // M T⁻²

unit!(compound: NewtonPerMetre, "N/m", [(Newton, P1), (Metre, N1)]);
unit!(compound: DynePerCentimetre, "dyn/cm", [(Dyne, P1), (Centimetre, N1)]);
