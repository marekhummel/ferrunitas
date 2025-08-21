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

// Implement conversions for all quantity types
// impl_prefix_conversions!(crate::system::defs::Mass);
// impl_prefix_conversions!(crate::system::defs::Length);
// impl_prefix_conversions!(crate::system::defs::Time);

// ============================================================================
// Base Units
// ============================================================================

unit!(Gram, Mass, "g");
unit!(Meter, Length, "m");
unit!(Second, Time, "s");
unit!(MeterPerSecond, Velocity, "m/s");
unit!(MeterPerSecondSquared, Acceleration, "m/s²");
unit!(Newton, Force, "N");
unit!(Joule, Energy, "J");
unit!(Watt, Power, "W");

// ============================================================================
// Derived Units (defined in terms of base units)
// ============================================================================

// Mass units - prefixed
prefixed_unit!(Kilogram, Kilo, Gram);

// Mass units - non-SI
unit!(Pound, Mass, 453.592, "lb"); // 1 lb = 453.592 g
unit!(Ounce, Mass, 28.3495, "oz"); // 1 oz = 28.3495 g

// Length units - prefixed
prefixed_unit!(Kilometer, Kilo, Meter);
// prefixed_unit!(Centimeter, Centi, Meter);
// prefixed_unit!(Millimeter, Milli, Meter);

// Length units - non-SI
unit!(Inch, Length, 0.0254, "in"); // 1 in = 0.0254 m
unit!(Foot, Length, 0.3048, "ft"); // 1 ft = 0.3048 m
unit!(Mile, Length, 1609.34, "mi"); // 1 mile = 1609.34 m

// Time units - non-SI (no SI prefixes commonly used for time)
unit!(Minute, Time, 60.0, "min"); // 1 min = 60 s
unit!(Hour, Time, 3600.0, "hr"); // 1 hr = 3600 s
