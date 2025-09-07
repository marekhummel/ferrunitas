//! Dimensionless quantities and pure number-like units.
//!
//! Provides markers for ratios, angles, efficiency factors and any other
//! unitless constructs used in formulas.

use crate::system::prefixes::*;
use crate::{quantity, unit};
use typenum::*;

// ===========================
// BASE DIMENSIONLESS
// ===========================
quantity!(Dimensionless: M Z0, L Z0, T Z0, I Z0, Th Z0, N Z0, J Z0); // Dimensionless
unit!(base: One, "1", Dimensionless);
unit!(derived: Percent, "%", (0.01, One));
unit!(derived: Permille, "‰", (0.001, One));
unit!(derived: PartsPerMillion, "ppm", (1e-6, One));
unit!(derived: PartsPerBillion, "ppb", (1e-9, One));

// ===========================
// PLANE ANGLE
// ===========================
quantity!(Angle: M Z0, L Z0, T Z0, I Z0, Th Z0, N Z0, J Z0; marked); // Dimensionless
unit!(base: Radian, "rad", Angle);
unit!(derived: Degree, "°", (std::f64::consts::PI / 180.0, Radian));
unit!(derived: Gradian, "gon", (std::f64::consts::PI / 200.0, Radian));

unit!(derived: Turn, "tr", (2.0 * std::f64::consts::PI, Radian));
unit!(derived: Arcminute, "arcmin", (std::f64::consts::PI / 10800.0, Radian));
unit!(derived: Arcsecond, "arcsec", (1.0 / 60.0, Arcminute));

// ===========================
// SOLID ANGLE
// ===========================
quantity!(SolidAngle: M Z0, L Z0, T Z0, I Z0, Th Z0, N Z0, J Z0; marked); // Dimensionless
unit!(compound: Steradian, "sr", [(Radian, P2)]; marked SolidAngle);

// ===========================
// LOGARITHMIC RATIOS
// ===========================
quantity!(LogarithmicRatio: M Z0, L Z0, T Z0, I Z0, Th Z0, N Z0, J Z0; marked); // Dimensionless
unit!(derived: Neper, "Np", (1, One));
unit!(derived: Octave, "oct", (std::f64::consts::LN_2, Neper));
unit!(derived: Bel, "B", (std::f64::consts::LN_10, Neper));
unit!(prefix: Decibel, Deci, Bel);

// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::verify_unit;

    // BASE DIMENSIONLESS
    verify_unit!(One, Dimensionless, 1.0);
    verify_unit!(Percent, Dimensionless, 0.01);
    verify_unit!(Permille, Dimensionless, 0.001);
    verify_unit!(PartsPerMillion, Dimensionless, 1e-6);
    verify_unit!(PartsPerBillion, Dimensionless, 1e-9);

    // PLANE ANGLE
    verify_unit!(Radian, Angle, 1.0);
    verify_unit!(Degree, Angle, 0.017453292519943295);
    verify_unit!(Gradian, Angle, 0.015707963267948967);
    verify_unit!(Turn, Angle, std::f64::consts::TAU);
    verify_unit!(Arcminute, Angle, 0.0002908882086657216);
    verify_unit!(Arcsecond, Angle, 4.84813681109536e-6);

    // SOLID ANGLE
    verify_unit!(Steradian, SolidAngle, 1.0);

    // LOGARITHMIC RATIOS
    verify_unit!(Neper, LogarithmicRatio, 1.0);
    verify_unit!(Octave, LogarithmicRatio, std::f64::consts::LN_2);
    verify_unit!(Bel, LogarithmicRatio, std::f64::consts::LN_10);
    verify_unit!(Decibel, LogarithmicRatio, 0.23025850929940457);
}
