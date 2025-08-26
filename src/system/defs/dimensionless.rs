use crate::model::quantity::Quantity;
use crate::model::unit::Unit;
use crate::unit;
use typenum::*;

// ===========================
// DIMENSIONLESS QUANTITIES
// ===========================
pub type Dimensionless = Quantity<Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
unit!(base: One, "1", Dimensionless;);

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
unit!(derived: Byte, "B", (8, Bit); prefixable);
unit!(derived: Nibble, "nibble", (4, Bit));

// Additional logarithmic units
unit!(derived: Bel, "B", (std::f64::consts::LN_10, Neper));
unit!(derived: Octave, "oct", (std::f64::consts::LN_2, Neper));

// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::verify_unit;

    // BASE DIMENSIONLESS
    verify_unit!(One, Dimensionless, 1.0);

    // PLANE ANGLE
    verify_unit!(Radian, Dimensionless, 1.0);
    verify_unit!(Degree, Dimensionless, 0.017453292519943295);
    verify_unit!(Gradian, Dimensionless, 0.015707963267948967);

    // SOLID ANGLE
    verify_unit!(Steradian, Dimensionless, 1.0);

    // LOGARITHMIC RATIOS
    verify_unit!(Neper, Dimensionless, 1.0);
    verify_unit!(Decibel, Dimensionless, 0.23025850929940457);

    // PERCENTAGE UNITS
    verify_unit!(Percent, Dimensionless, 0.01);
    verify_unit!(Permille, Dimensionless, 0.001);
    verify_unit!(PartsPerMillion, Dimensionless, 1e-6);
    verify_unit!(PartsPerBillion, Dimensionless, 1e-9);

    // ADDITIONAL ANGULAR UNITS
    verify_unit!(Turn, Dimensionless, std::f64::consts::TAU);
    verify_unit!(Arcminute, Dimensionless, 0.0002908882086657216);
    verify_unit!(Arcsecond, Dimensionless, 4.84813681109536e-6);

    // INFORMATION UNITS
    verify_unit!(Bit, Dimensionless, 1.0);
    verify_unit!(Byte, Dimensionless, 8.0);
    verify_unit!(Nibble, Dimensionless, 4.0);

    // ADDITIONAL LOGARITHMIC UNITS
    verify_unit!(Bel, Dimensionless, std::f64::consts::LN_10);
    verify_unit!(Octave, Dimensionless, std::f64::consts::LN_2);
}
