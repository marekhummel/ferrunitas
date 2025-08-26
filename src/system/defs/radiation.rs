use crate::model::quantity::Quantity;
use crate::model::unit::Unit;
use crate::system::defs::base::*;
use crate::system::defs::electromagnetism::*;
use crate::system::defs::mechanics::*;
use crate::system::prefixes::*;
use crate::unit;
use typenum::*;

// ===========================
// RADIOACTIVITY
// ===========================
pub type Radioactivity = Quantity<Z0, Z0, N1, Z0, Z0, Z0, Z0>; // T⁻¹

unit!(compound: Becquerel, "Bq", [(Second, N1)], prefixable);
unit!(prefix: Kilobecquerel, Kilo, Becquerel);
unit!(prefix: Megabecquerel, Mega, Becquerel);
unit!(prefix: Gigabecquerel, Giga, Becquerel);

unit!(derived: Curie, "Ci", (3.7e10, Becquerel), prefixable);
unit!(prefix: Millicurie, Milli, Curie);
unit!(prefix: Microcurie, Micro, Curie);
unit!(prefix: Nanocurie, Nano, Curie);
unit!(prefix: Picocurie, Pico, Curie);

// ===========================
// ABSORBED DOSE
// ===========================
pub type AbsorbedDose = Quantity<Z0, P2, N2, Z0, Z0, Z0, Z0>; // L² T⁻²

unit!(compound: Gray, "Gy", [(Joule, P1), (Kilogram, N1)], prefixable);
unit!(prefix: Milligray, Milli, Gray);
unit!(prefix: Microgray, Micro, Gray);

unit!(derived: Rad, "rad", (0.01, Gray));

// ===========================
// EQUIVALENT DOSE
// ===========================
pub type EquivalentDose = Quantity<Z0, P2, N2, Z0, Z0, Z0, Z0>; // L² T⁻²

unit!(compound: Sievert, "Sv", [(Joule, P1), (Kilogram, N1)], prefixable);
unit!(prefix: Millisievert, Milli, Sievert);
unit!(prefix: Microsievert, Micro, Sievert);

unit!(derived: Rem, "rem", (0.01, Sievert), prefixable);
unit!(prefix: Millirem, Milli, Rem);

// ===========================
// EXPOSURE (X-RAY/GAMMA RAY)
// ===========================
pub type Exposure = Quantity<N1, Z0, P1, P1, Z0, Z0, Z0>; // M⁻¹ T I

unit!(compound: CoulombPerKilogram, "C/kg", [(Coulomb, P1), (Kilogram, N1)]);
unit!(derived: Roentgen, "R", (2.58e-4, CoulombPerKilogram));

// ===========================
// DOSE RATE
// ===========================
pub type DoseRate = Quantity<Z0, P2, N3, Z0, Z0, Z0, Z0>; // L² T⁻³

unit!(compound: GrayPerSecond, "Gy/s", [(Gray, P1), (Second, N1)]);
unit!(compound: SievertPerSecond, "Sv/s", [(Sievert, P1), (Second, N1)]);
unit!(compound: RadPerSecond, "rad/s", [(Rad, P1), (Second, N1)]);

// ===========================
// NUCLEAR CROSS SECTION
// ===========================
pub type CrossSection = Quantity<Z0, P2, Z0, Z0, Z0, Z0, Z0>; // L²

unit!(derived: Barn, "b", (1e-28, SquareMetre), prefixable);
unit!(prefix: Millibarn, Milli, Barn);
unit!(prefix: Microbarn, Micro, Barn);
unit!(prefix: Nanobarn, Nano, Barn);

// ===========================
// FLUENCE
// ===========================
pub type Fluence = Quantity<Z0, N2, Z0, Z0, Z0, Z0, Z0>; // L⁻²

unit!(compound: ParticlePerSquareMetre, "m⁻²", [(Metre, N2)]);
unit!(compound: ParticlePerSquareCentimetre, "cm⁻²", [(Centimetre, N2)]);

// ===========================
// FLUX DENSITY
// ===========================
pub type FluxDensity = Quantity<Z0, N2, N1, Z0, Z0, Z0, Z0>; // L⁻² T⁻¹

unit!(compound: ParticlePerSquareMetreSecond, "m⁻²⋅s⁻¹", [(Metre, N2), (Second, N1)]);
unit!(compound: ParticlePerSquareCentimetreSecond, "cm⁻²⋅s⁻¹", [(Centimetre, N2), (Second, N1)]);

// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::verify_unit;

    // RADIOACTIVITY
    verify_unit!(Becquerel, Radioactivity, 1.0);
    verify_unit!(Kilobecquerel, Radioactivity, 1000.0);
    verify_unit!(Megabecquerel, Radioactivity, 1e6);
    verify_unit!(Gigabecquerel, Radioactivity, 1e9);
    verify_unit!(Curie, Radioactivity, 3.7e10);
    verify_unit!(Millicurie, Radioactivity, 3.7e7);
    verify_unit!(Microcurie, Radioactivity, 3.7e4);
    verify_unit!(Nanocurie, Radioactivity, 37.0);
    verify_unit!(Picocurie, Radioactivity, 0.037);

    // ABSORBED DOSE
    verify_unit!(Gray, AbsorbedDose, 1.0);
    verify_unit!(Milligray, AbsorbedDose, 0.001);
    verify_unit!(Microgray, AbsorbedDose, 1e-6);
    verify_unit!(Rad, AbsorbedDose, 0.01);

    // EQUIVALENT DOSE
    verify_unit!(Sievert, EquivalentDose, 1.0);
    verify_unit!(Millisievert, EquivalentDose, 0.001);
    verify_unit!(Microsievert, EquivalentDose, 1e-6);
    verify_unit!(Rem, EquivalentDose, 0.01);
    verify_unit!(Millirem, EquivalentDose, 1e-5);

    // EXPOSURE
    verify_unit!(CoulombPerKilogram, Exposure, 0.001);
    verify_unit!(Roentgen, Exposure, 2.58e-7);

    // DOSE RATE
    verify_unit!(GrayPerSecond, DoseRate, 1.0);
    verify_unit!(SievertPerSecond, DoseRate, 1.0);
    verify_unit!(RadPerSecond, DoseRate, 0.01);

    // NUCLEAR CROSS SECTION
    verify_unit!(Barn, CrossSection, 1e-28);
    verify_unit!(Millibarn, CrossSection, 1e-31);
    verify_unit!(Microbarn, CrossSection, 1e-34);
    verify_unit!(Nanobarn, CrossSection, 1e-37);

    // FLUENCE
    verify_unit!(ParticlePerSquareMetre, Fluence, 1.0);
    verify_unit!(ParticlePerSquareCentimetre, Fluence, 10000.0);

    // FLUX DENSITY
    verify_unit!(ParticlePerSquareMetreSecond, FluxDensity, 1.0);
    verify_unit!(ParticlePerSquareCentimetreSecond, FluxDensity, 10000.0);
}
