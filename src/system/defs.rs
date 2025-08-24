use crate::model::quantity::Quantity;
use crate::model::unit::Unit;
use crate::{prefix, unit};
use typenum::*;

// ============================================================================
// Base SI Quantities as Type Aliases
// ============================================================================

pub type Mass = Quantity<P1, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Length = Quantity<Z0, P1, Z0, Z0, Z0, Z0, Z0>;
pub type Time = Quantity<Z0, Z0, P1, Z0, Z0, Z0, Z0>;
pub type ElectricCurrent = Quantity<Z0, Z0, Z0, P1, Z0, Z0, Z0>;
pub type Temperature = Quantity<Z0, Z0, Z0, Z0, P1, Z0, Z0>;
pub type AmountOfSubstance = Quantity<Z0, Z0, Z0, Z0, Z0, P1, Z0>;
pub type LuminousIntensity = Quantity<Z0, Z0, Z0, Z0, Z0, Z0, P1>;

// Derived quantities
pub type Velocity = Quantity<Z0, P1, N1, Z0, Z0, Z0, Z0>;
pub type Acceleration = Quantity<Z0, P1, N2, Z0, Z0, Z0, Z0>;
pub type Force = Quantity<P1, P1, N2, Z0, Z0, Z0, Z0>;
pub type Energy = Quantity<P1, P2, N2, Z0, Z0, Z0, Z0>;
pub type Power = Quantity<P1, P2, N3, Z0, Z0, Z0, Z0>;
pub type Area = Quantity<Z0, P2, Z0, Z0, Z0, Z0, Z0>;
pub type Volume = Quantity<Z0, P3, Z0, Z0, Z0, Z0, Z0>;

// ============================================================================
// SI Prefixes
// ============================================================================

prefix!(Kilo, 1000.0, "k", "kilo");
prefix!(Deci, 0.1, "d", "deci");
prefix!(Centi, 0.01, "c", "centi");
prefix!(Milli, 0.001, "m", "milli");
prefix!(Micro, 0.000001, "μ", "micro");
prefix!(Mega, 1_000_000.0, "M", "mega");
prefix!(Giga, 1_000_000_000.0, "G", "giga");

// ============================================================================
// Base Units
// ============================================================================

unit!(base: Gram, Mass, "g", prefixable);
unit!(base: Meter, Length, "m", prefixable);
unit!(base: Second, Time, "s", prefixable);
unit!(base: Ampere, ElectricCurrent, "A", prefixable);
unit!(base: Kelvin, Temperature, "K");
unit!(base: Mole, AmountOfSubstance, "mol");
unit!(base: Candela, LuminousIntensity, "cd", prefixable);

// ============================================================================
// Prefixed Units
// ============================================================================

// Mass units - prefixed
unit!(prefix: Kilogram, Kilo, Gram);

// Length units - prefixed
unit!(prefix: Kilometer, Kilo, Meter);
unit!(prefix: Decimeter, Deci, Meter);
unit!(prefix: Centimeter, Centi, Meter);
unit!(prefix: Millimeter, Milli, Meter);

// ============================================================================
// Derived Units (defined in terms of base units)
// ============================================================================

// Mass units - non-SI
unit!(derived: Pound, (453.592, Gram), "lb");
unit!(derived: Ounce, (28.3495, Gram), "oz");

// Length units - non-SI
unit!(derived: Inch, (2.54, Centimeter), "in");
unit!(derived: Foot, (12, Inch), "ft");
unit!(derived: Yard, (3, Foot), "yd");
unit!(derived: Mile, (1760, Yard), "mi");
unit!(derived: NauticalMile, (1852, Meter), "nmi");

// Time units - non-SI
unit!(derived: Minute, (60, Second), "min");
unit!(derived: Hour, (60, Minute), "hr");

// ============================================================================
// Compound Units (defined in terms of base units)
// ============================================================================

unit!(compound: MeterPerSecond, "m/s", [(Meter, P1), (Second, N1)]);
unit!(compound: MeterPerSecondSquared, "m/s²", [(Meter, P1), (Second, N2)]);
unit!(compound: Newton, "N", [(Kilogram, P1), (Meter, P1), (Second, N2)], prefixable);
unit!(compound: Joule, "J", [(Newton, P1), (Meter, P1)], prefixable);
unit!(compound: Watt, "W", [(Joule, P1), (Second, N1)], prefixable);
unit!(compound: MeterSquared, "m²", [(Meter, P2)]);
unit!(compound: MeterCubed, "m³", [(Meter, P3)]);

unit!(compound: Liter, "L", [(Decimeter, P3)], prefixable);
unit!(compound: Knots, "kn", [(NauticalMile, P1), (Hour, N1)]);
