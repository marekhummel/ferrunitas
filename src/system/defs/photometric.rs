use crate::system::defs::base::*;
use crate::system::defs::dimensionless::*;
use crate::system::defs::mechanics::*;
use crate::system::prefixes::*;
use crate::{quantity, unit};
use typenum::*;

// ===========================
// LUMINOUS FLUX (Steradiands are not dimensionless in SI, so this is equal to Luminious Intensity)
// ===========================
quantity!(LuminousFlux: M Z0, L Z0, T Z0, I Z0, Th Z0, N Z0, J P1); // J

unit!(compound: Lumen, "lm", [(Candela, P1), (Steradian, P1)]);

// ===========================
// LUMINANCE
// ===========================
quantity!(Luminance: M Z0, L N2, T Z0, I Z0, Th Z0, N Z0, J P1); // L⁻² J

unit!(compound: Nit, "nt", [(Candela, P1), (Metre, N2)]; prefixable);

// ===========================
// ILLUMINANCE
// ===========================
quantity!(Illuminance: M Z0, L N2, T Z0, I Z0, Th Z0, N Z0, J P1); // L⁻² J

unit!(compound: Lux, "lx", [(Lumen, P1), (SquareMetre, N1)]; prefixable);
unit!(prefix: Kilolux, Kilo, Lux);

// ===========================
// OPTICAL POWER (LENS)
// ===========================
quantity!(OpticalPower: M Z0, L N1, T Z0, I Z0, Th Z0, N Z0, J Z0); // L⁻¹

unit!(compound: Dioptre, "D", [(Metre, N1)]);

// ===========================
// LUMINOUS ENERGY
// ===========================
quantity!(LuminousEnergy: M Z0, L Z0, T P1, I Z0, Th Z0, N Z0, J P1); // T J

unit!(compound: LumenSecond, "lm⋅s", [(Lumen, P1), (Second, P1)]);
unit!(derived: Talbot, "T", (1, LumenSecond)); // Historical unit

// ===========================
// LUMINOUS EXPOSURE
// ===========================
quantity!(LuminousExposure: M Z0, L N2, T P1, I Z0, Th Z0, N Z0, J P1); // L⁻² T J

unit!(compound: LuxSecond, "lx⋅s", [(Lux, P1), (Second, P1)]);

// ===========================
// LUMINOUS EFFICACY
// ===========================
quantity!(LuminousEfficacy: M N1, L N2, T P3, I Z0, Th Z0, N Z0, J P1); // M⁻¹ L⁻² T³ J

unit!(compound: LumenPerWatt, "lm/W", [(Lumen, P1), (Watt, N1)]);

// ===========================
// RADIANCE
// ===========================
quantity!(Radiance: M P1, L Z0, T N3, I Z0, Th Z0, N Z0, J Z0); // M T⁻³

unit!(compound: WattPerSquareMetreSteradian, "W/(m²⋅sr)", [(Watt, P1), (Metre, N2), (Steradian, N1)]);

// ===========================
// RADIANT INTENSITY
// ===========================
quantity!(RadiantIntensity: M P1, L P2, T N3, I Z0, Th Z0, N Z0, J Z0); // M L² T⁻³

unit!(compound: WattPerSteradian, "W/sr", [(Watt, P1), (Steradian, N1)]);

// ===========================
// IRRADIANCE / RADIANT EXITANCE
// ===========================
quantity!(Irradiance: M P1, L Z0, T N3, I Z0, Th Z0, N Z0, J Z0); // M T⁻³

unit!(compound: WattPerSquareMetre, "W/m²", [(Watt, P1), (Metre, N2)]);

// ===========================
// SPECTRAL RADIANCE
// ===========================
quantity!(SpectralRadiance: M P1, L N1, T N3, I Z0, Th Z0, N Z0, J Z0); // M L⁻¹ T⁻³

unit!(compound: WattPerSquareMetreMetreSteradian, "W/(m³⋅sr)", [(Watt, P1), (Metre, N3), (Steradian, N1)]);
unit!(compound: WattPerSquareMetreNanometreSteradian, "W/(m²⋅nm⋅sr)", [(Watt, P1), (Metre, N2), (Nanometre, N1), (Steradian, N1)]);

// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::verify_unit;

    // LUMINOUS FLUX
    verify_unit!(Lumen, LuminousFlux, 1.0);

    // LUMINANCE
    verify_unit!(Nit, Luminance, 1.0);

    // ILLUMINANCE
    verify_unit!(Lux, Illuminance, 1.0);
    verify_unit!(Kilolux, Illuminance, 1000.0);

    // OPTICAL POWER
    verify_unit!(Dioptre, OpticalPower, 1.0);

    // LUMINOUS ENERGY
    verify_unit!(LumenSecond, LuminousEnergy, 1.0);
    verify_unit!(Talbot, LuminousEnergy, 1.0);

    // LUMINOUS EXPOSURE
    verify_unit!(LuxSecond, LuminousExposure, 1.0);

    // LUMINOUS EFFICACY
    verify_unit!(LumenPerWatt, LuminousEfficacy, 1.0);

    // RADIANCE
    verify_unit!(WattPerSquareMetreSteradian, Radiance, 1.0);

    // RADIANT INTENSITY
    verify_unit!(WattPerSteradian, RadiantIntensity, 1.0);

    // IRRADIANCE
    verify_unit!(WattPerSquareMetre, Irradiance, 1.0);

    // SPECTRAL RADIANCE
    verify_unit!(WattPerSquareMetreMetreSteradian, SpectralRadiance, 1.0);
    verify_unit!(WattPerSquareMetreNanometreSteradian, SpectralRadiance, 1e9);
}
