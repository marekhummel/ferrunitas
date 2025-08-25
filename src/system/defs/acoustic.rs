use crate::model::quantity::Quantity;
use crate::model::unit::Unit;
use crate::system::defs::base::*;
use crate::system::defs::mechanics::*;
use crate::unit;
use typenum::*;

// ============================================================================
// ACOUSTIC IMPEDANCE
// ============================================================================
pub type AcousticImpedance = Quantity<P1, N2, N1, Z0, Z0, Z0, Z0>; // M L⁻² T⁻¹

unit!(compound: Rayl, "rayl", [(Pascal, P1), (Second, P1), (Metre, N1)]);

// ============================================================================
// SOUND INTENSITY
// ============================================================================
pub type SoundIntensity = Quantity<P1, Z0, N3, Z0, Z0, Z0, Z0>; // M T⁻³

// Already defined in photometrics for Irradiance
// unit!(compound: WattPerSquareMetre, "W/m²", [(Watt, P1), (Metre, N2)]);

// ============================================================================
// SOUND EXPOSURE
// ============================================================================
pub type SoundExposure = Quantity<P1, Z0, N1, Z0, Z0, Z0, Z0>; // M T⁻¹

unit!(compound: PascalSquaredSecond, "Pa²⋅s", [(Pascal, P2), (Second, P1)]);
