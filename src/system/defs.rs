use crate::model::prefix::*;
use crate::model::quantity::Quantity;
use crate::{base_unit, define_prefix, derived_unit, prefixed_unit};
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

define_prefix!(Kilo, 1000.0, "k", "kilo");
define_prefix!(Centi, 0.01, "c", "centi");
define_prefix!(Milli, 0.001, "m", "milli");
define_prefix!(Micro, 0.000001, "μ", "micro");
define_prefix!(Mega, 1_000_000.0, "M", "mega");
define_prefix!(Giga, 1_000_000_000.0, "G", "giga");

// ============================================================================
// Base Units
// ============================================================================

base_unit!(Gram, Mass);
base_unit!(Meter, Length);
base_unit!(Second, Time);

// ============================================================================
// Derived Units (defined in terms of base units)
// ============================================================================

// Mass units - prefixed
prefixed_unit!(Kilogram, Kilo, Gram);

// Mass units - non-SI
derived_unit!(Pound, Mass, 453.592); // 1 lb = 453.592 g
derived_unit!(Ounce, Mass, 28.3495); // 1 oz = 28.3495 g

// Length units - prefixed
prefixed_unit!(Kilometer, Kilo, Meter);
prefixed_unit!(Centimeter, Centi, Meter);
prefixed_unit!(Millimeter, Milli, Meter);

// Length units - non-SI
derived_unit!(Inch, Length, 0.0254); // 1 in = 0.0254 m
derived_unit!(Foot, Length, 0.3048); // 1 ft = 0.3048 m
derived_unit!(Mile, Length, 1609.34); // 1 mile = 1609.34 m

// Time units - non-SI (no SI prefixes commonly used for time)
derived_unit!(Minute, Time, 60.0); // 1 min = 60 s
derived_unit!(Hour, Time, 3600.0); // 1 hr = 3600 s
