use crate::system::defs::base::*;
use crate::system::defs::mechanics::*;
use crate::{quantity, unit};
use typenum::*;

// ===========================
// ACOUSTIC IMPEDANCE
// ===========================
quantity!(AcousticImpedance: M P1, L N2, T N1, I Z0, Th Z0, N Z0, J Z0); // M L⁻² T⁻¹

unit!(compound: Rayl, "rayl", [(Pascal, P1), (Second, P1), (Metre, N1)]);

// ===========================
// SOUND INTENSITY
// ===========================
quantity!(SoundIntensity: M P1, L Z0, T N3, I Z0, Th Z0, N Z0, J Z0); // M T⁻³

// Already defined in photometrics for Irradiance
// unit!(compound: WattPerSquareMetre, "W/m²", [(Watt, P1), (Metre, N2)]);

// ===========================
// SOUND EXPOSURE
// ===========================
quantity!(SoundExposure: M P2, L N2, T N3, I Z0, Th Z0, N Z0, J Z0); // M² L⁻² T⁻³

unit!(compound: PascalSquaredSecond, "Pa²⋅s", [(Pascal, P2), (Second, P1)]);

// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::verify_unit;

    // ACOUSTIC IMPEDANCE
    verify_unit!(Rayl, AcousticImpedance, 1.0);

    // SOUND EXPOSURE
    verify_unit!(PascalSquaredSecond, SoundExposure, 1.0);
}
