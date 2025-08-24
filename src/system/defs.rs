// use crate::model::prefix::*;
use crate::model::quantity::Quantity;
use crate::model::unit::Unit;
use crate::{prefix, prefixed_unit, unit};
use typenum::*;

// ============================================================================
// Base SI Quantities as Type Aliases
// ============================================================================

pub type Mass = Quantity<P1, Z0, Z0, Z0, Z0, Z0, Z0>; // [1,0,0,0,0,0,0]
pub type Length = Quantity<Z0, P1, Z0, Z0, Z0, Z0, Z0>; // [0,1,0,0,0,0,0]
pub type Time = Quantity<Z0, Z0, P1, Z0, Z0, Z0, Z0>; // [0,0,1,0,0,0,0]
pub type ElectricCurrent = Quantity<Z0, Z0, Z0, P1, Z0, Z0, Z0>; // [0,0,0,1,0,0,0]
pub type Temperature = Quantity<Z0, Z0, Z0, Z0, P1, Z0, Z0>; // [0,0,0,0,1,0,0]
pub type AmountOfSubstance = Quantity<Z0, Z0, Z0, Z0, Z0, P1, Z0>; // [0,0,0,0,0,1,0]
pub type LuminousIntensity = Quantity<Z0, Z0, Z0, Z0, Z0, Z0, P1>; // [0,0,0,0,0,0,1]

// Derived quantities
pub type Velocity = Quantity<Z0, P1, N1, Z0, Z0, Z0, Z0>; // [0,1,-1,0,0,0,0] = L/T
pub type Acceleration = Quantity<Z0, P1, N2, Z0, Z0, Z0, Z0>; // [0,1,-2,0,0,0,0] = L/T²
pub type Force = Quantity<P1, P1, N2, Z0, Z0, Z0, Z0>; // [1,1,-2,0,0,0,0] = M⋅L/T²
pub type Energy = Quantity<P1, P2, N2, Z0, Z0, Z0, Z0>; // [1,2,-2,0,0,0,0] = M⋅L²/T²
pub type Power = Quantity<P1, P2, N3, Z0, Z0, Z0, Z0>; // [1,2,-3,0,0,0,0] = M⋅L²/T³

// ============================================================================
// SI Prefixes
// ============================================================================

prefix!(Kilo, 1000.0, "k", "kilo");
prefix!(Centi, 0.01, "c", "centi");
prefix!(Milli, 0.001, "m", "milli");
prefix!(Micro, 0.000001, "μ", "micro");
prefix!(Mega, 1_000_000.0, "M", "mega");
prefix!(Giga, 1_000_000_000.0, "G", "giga");

// ============================================================================
// Base Units
// ============================================================================

unit!(Gram, Mass, "g", prefixable);
unit!(Meter, Length, "m", prefixable);
unit!(Second, Time, "s", prefixable);

unit!(Ampere, ElectricCurrent, "A", prefixable);
unit!(Kelvin, Temperature, "K");
unit!(Mole, AmountOfSubstance, "mol");
unit!(Candela, LuminousIntensity, "cd", prefixable);

// Derived

// unit!(MeterPerSecond, Velocity, "m/s");
unit!(MeterPerSecondSquared, Acceleration, "m/s²");
unit!(Newton, Force, "N", prefixable);
unit!(Joule, Energy, "J", prefixable);
unit!(Watt, Power, "W", prefixable);

unit!(MeterSeconds, "ms", [(Kilometer, P1), (Second, N2)]); // m⋅s
unit!(NauticalMile, Length, (1852, Meter), "nmi"); // 1 nmi = 1852 m
unit!(Knots, "kn", [(NauticalMile, P1), (Hour, N1)]); // 1 kn = 1 nmi/h
unit!(MeterPerSecond, "m/s", [(Meter, P1), (Second, N1)]);

// ============================================================================
// Derived Units (defined in terms of base units)
// ============================================================================

// Mass units - prefixed
prefixed_unit!(Kilogram, Kilo, Gram);

// Mass units - non-SI
unit!(Pound, Mass, (453.592, Gram), "lb"); // 1 lb = 453.592 g
unit!(Ounce, Mass, (28.3495, Gram), "oz"); // 1 oz = 28.3495 g

// Length units - prefixed
prefixed_unit!(Kilometer, Kilo, Meter);
prefixed_unit!(Centimeter, Centi, Meter);
prefixed_unit!(Millimeter, Milli, Meter);

// Length units - non-SI
unit!(Inch, Length, (0.0254, Meter), "in");
unit!(Foot, Length, (12, Inch), "ft");
unit!(Yard, Length, (3, Foot), "yd");
unit!(Mile, Length, (1760, Yard), "mi");

// Time units - non-SI (no SI prefixes commonly used for time)
unit!(Minute, Time, (60, Second), "min"); // 1 min = 60 s
unit!(Hour, Time, (60, Minute), "hr"); // 1 hr = 3600 s
