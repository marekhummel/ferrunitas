use crate::model::quantity::Quantity;
use crate::model::unit::Unit;
use crate::unit;
use typenum::*;

// ============================================================================
// DIMENSIONLESS QUANTITIES
// ============================================================================
pub type Dimensionless = Quantity<Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
unit!(base: One, Dimensionless, "1");

// Plane angle
unit!(derived: Radian, "rad", (1, One));
unit!(derived: Degree, "°", (std::f64::consts::PI / 180.0, Radian));
unit!(derived: Gradian, "gon", (std::f64::consts::PI / 200.0, Radian));

// Solid angle
unit!(derived: Steradian, "sr", (1, One));

// Logarithmic ratios
unit!(derived: Neper, "Np", (1, One));
unit!(derived: Decibel, "dB", (0.1 * std::f64::consts::LN_10, Neper));

// Others (useful named dimensionless units)
unit!(derived: Percent, "%", (0.01, One));
unit!(derived: Permille, "‰", (0.001, One));
unit!(derived: PartsPerMillion, "ppm", (1e-6, One));
unit!(derived: PartsPerBillion, "ppb", (1e-9, One));

// Additional angular units
unit!(derived: Turn, "tr", (2.0 * std::f64::consts::PI, Radian));
unit!(derived: Arcminute, "arcmin", (std::f64::consts::PI / 10800.0, Radian));
unit!(derived: Arcsecond, "arcsec", (std::f64::consts::PI / 648000.0, Radian));

// Information units (TODO)
unit!(derived: Bit, "bit", (1, One));
unit!(derived: Byte, "B", (8, Bit), prefixable);
unit!(derived: Nibble, "nibble", (4, Bit));

// Additional logarithmic units
unit!(derived: Bel, "B", (std::f64::consts::LN_10, Neper));
unit!(derived: Octave, "oct", (std::f64::consts::LN_2, Neper));
