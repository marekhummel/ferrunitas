use crate::model::quantity::Quantity;
use crate::model::unit::Unit;
use crate::system::defs::base::*;
use crate::system::defs::mechanics::*;
use crate::unit;
use typenum::*;

// ===========================
// ACOUSTIC IMPEDANCE
// ===========================
pub type AcousticImpedance = Quantity<P1, N2, N1, Z0, Z0, Z0, Z0>; // M L⁻² T⁻¹

unit!(compound: Rayl, "rayl", [(Pascal, P1), (Second, P1), (Metre, N1)]);

// ===========================
// SOUND INTENSITY
// ===========================
pub type SoundIntensity = Quantity<P1, Z0, N3, Z0, Z0, Z0, Z0>; // M T⁻³

// Already defined in photometrics for Irradiance
// unit!(compound: WattPerSquareMetre, "W/m²", [(Watt, P1), (Metre, N2)]);

// ===========================
// SOUND EXPOSURE
// ===========================
pub type SoundExposure = Quantity<P2, N2, N3, Z0, Z0, Z0, Z0>; // M² L⁻² T⁻³

unit!(compound: PascalSquaredSecond, "Pa²⋅s", [(Pascal, P2), (Second, P1)]);

// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::verify_unit;

    // ACOUSTIC IMPEDANCE
    verify_unit!(Rayl, AcousticImpedance, 1000.0);

    // SOUND EXPOSURE
    verify_unit!(PascalSquaredSecond, SoundExposure, 1000000.0);
}
