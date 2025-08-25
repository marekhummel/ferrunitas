use crate::model::quantity::Quantity;
use crate::model::unit::Unit;
use crate::system::defs::base::*;
use crate::system::defs::electromagnetism::*;
use crate::system::defs::mechanics::*;
use crate::system::prefixes::*;
use crate::unit;
use typenum::*;

// ============================================================================
// RADIOACTIVITY
// ============================================================================
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

// ============================================================================
// ABSORBED DOSE
// ============================================================================
pub type AbsorbedDose = Quantity<Z0, P2, N2, Z0, Z0, Z0, Z0>; // L² T⁻²

unit!(compound: Gray, "Gy", [(Joule, P1), (Kilogram, N1)], prefixable);
unit!(prefix: Milligray, Milli, Gray);
unit!(prefix: Microgray, Micro, Gray);

unit!(derived: Rad, "rad", (0.01, Gray));

// ============================================================================
// EQUIVALENT DOSE
// ============================================================================
pub type EquivalentDose = Quantity<Z0, P2, N2, Z0, Z0, Z0, Z0>; // L² T⁻²

unit!(compound: Sievert, "Sv", [(Joule, P1), (Kilogram, N1)], prefixable);
unit!(prefix: Millisievert, Milli, Sievert);
unit!(prefix: Microsievert, Micro, Sievert);

unit!(derived: Rem, "rem", (0.01, Sievert), prefixable);
unit!(prefix: Millirem, Milli, Rem);

// ============================================================================
// EXPOSURE (X-RAY/GAMMA RAY)
// ============================================================================
pub type Exposure = Quantity<N1, Z0, P1, P1, Z0, Z0, Z0>; // M⁻¹ T I

unit!(compound: CoulombPerKilogram, "C/kg", [(Coulomb, P1), (Kilogram, N1)]);
unit!(derived: Roentgen, "R", (2.58e-4, CoulombPerKilogram));

// ============================================================================
// DOSE RATE
// ============================================================================
pub type DoseRate = Quantity<Z0, P2, N3, Z0, Z0, Z0, Z0>; // L² T⁻³

unit!(compound: GrayPerSecond, "Gy/s", [(Gray, P1), (Second, N1)]);
unit!(compound: SievertPerSecond, "Sv/s", [(Sievert, P1), (Second, N1)]);
unit!(compound: RadPerSecond, "rad/s", [(Rad, P1), (Second, N1)]);

// ============================================================================
// NUCLEAR CROSS SECTION
// ============================================================================
pub type CrossSection = Quantity<Z0, P2, Z0, Z0, Z0, Z0, Z0>; // L²

unit!(derived: Barn, "b", (1e-28, SquareMetre), prefixable);
unit!(prefix: Millibarn, Milli, Barn);
unit!(prefix: Microbarn, Micro, Barn);
unit!(prefix: Nanobarn, Nano, Barn);

// ============================================================================
// FLUENCE
// ============================================================================
pub type Fluence = Quantity<Z0, N2, Z0, Z0, Z0, Z0, Z0>; // L⁻²

unit!(compound: ParticlePerSquareMetre, "m⁻²", [(Metre, N2)]);
unit!(compound: ParticlePerSquareCentimetre, "cm⁻²", [(Centimetre, N2)]);

// ============================================================================
// FLUX DENSITY
// ============================================================================
pub type FluxDensity = Quantity<Z0, N2, N1, Z0, Z0, Z0, Z0>; // L⁻² T⁻¹

unit!(compound: ParticlePerSquareMetreSecond, "m⁻²⋅s⁻¹", [(Metre, N2), (Second, N1)]);
unit!(compound: ParticlePerSquareCentimetreSecond, "cm⁻²⋅s⁻¹", [(Centimetre, N2), (Second, N1)]);
