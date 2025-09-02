//! Thermodynamic & heat-related derived units (heat capacity, entropy, flux, etc.).

use crate::system::defs::base::*;
use crate::system::defs::mechanics::*;
use crate::{quantity, unit};
use typenum::*;

// ===========================
// HEAT CAPACITY / ENTROPY
// ===========================
quantity!(HeatCapacity: M P1, L P2, T N2, I Z0, Th N1, N Z0, J Z0); // M L² T⁻² Θ⁻¹
quantity!(Entropy: M P1, L P2, T N2, I Z0, Th N1, N Z0, J Z0); // M L² T⁻² Θ⁻¹

unit!(compound: JoulePerKelvin, "J/K", [(Joule, P1), (Kelvin, N1)]);

// ===========================
// SPECIFIC HEAT CAPACITY
// ===========================
quantity!(SpecificHeat: M Z0, L P2, T N2, I Z0, Th N1, N Z0, J Z0); // L² T⁻² Θ⁻¹

unit!(compound: JoulePerKilogramKelvin, "J/(kg·K)", [(Joule, P1), (Kilogram, N1), (Kelvin, N1)]);
unit!(compound: KilojoulePerKilogramKelvin, "kJ/(kg·K)", [(Kilojoule, P1), (Kilogram, N1), (Kelvin, N1)]);

// ===========================
// THERMAL CONDUCTIVITY
// ===========================
quantity!(ThermalConductivity: M P1, L P1, T N3, I Z0, Th N1, N Z0, J Z0); // M L T⁻³ Θ⁻¹

unit!(compound: WattPerMetreKelvin, "W/(m·K)", [(Watt, P1), (Metre, N1), (Kelvin, N1)]);
unit!(compound: MilliwattPerMetreKelvin, "mW/(m·K)", [(Milliwatt, P1), (Metre, N1), (Kelvin, N1)]);

// ===========================
// THERMAL RESISTANCE
// ===========================
quantity!(ThermalResistance: M N1, L N2, T P3, I Z0, Th P1, N Z0, J Z0); // M⁻¹ L⁻² T³ Θ

unit!(compound: KelvinPerWatt, "K/W", [(Kelvin, P1), (Watt, N1)]);

// ===========================
// THERMAL RESISTIVITY
// ===========================
quantity!(ThermalResistivity: M N1, L N1, T P3, I Z0, Th P1, N Z0, J Z0); // M⁻¹ L⁻¹ T³ Θ

unit!(compound: MetreKelvinPerWatt, "m·K/W", [(Metre, P1), (Kelvin, P1), (Watt, N1)]);

// ===========================
// THERMAL DIFFUSIVITY
// ===========================
quantity!(ThermalDiffusivity: M Z0, L P2, T N1, I Z0, Th Z0, N Z0, J Z0); // L² T⁻¹

// Already defined in mechanics for kinematic viscosity
// unit!(compound: SquareMetrePerSecond, "m²/s", [(Metre, P2), (Second, N1)]);

// ===========================
// HEAT TRANSFER COEFFICIENT
// ===========================
quantity!(HeatTransferCoefficient: M P1, L Z0, T N3, I Z0, Th N1, N Z0, J Z0); // M T⁻³ Θ⁻¹

unit!(compound: WattPerSquareMetreKelvin, "W/(m²·K)", [(Watt, P1), (Metre, N2), (Kelvin, N1)]);

// ===========================
// THERMAL EXPANSION COEFFICIENT
// ===========================
quantity!(ThermalExpansionCoefficient: M Z0, L Z0, T Z0, I Z0, Th N1, N Z0, J Z0); // Θ⁻¹

unit!(compound: PerKelvin, "K⁻¹", [(Kelvin, N1)]);

// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::verify_unit;

    // HEAT CAPACITY / ENTROPY
    verify_unit!(JoulePerKelvin, HeatCapacity, 1.0);
    verify_unit!(JoulePerKelvin, Entropy, 1.0);

    // SPECIFIC HEAT CAPACITY
    verify_unit!(JoulePerKilogramKelvin, SpecificHeat, 1.0);
    verify_unit!(KilojoulePerKilogramKelvin, SpecificHeat, 1e3);

    // THERMAL CONDUCTIVITY
    verify_unit!(WattPerMetreKelvin, ThermalConductivity, 1.0);
    verify_unit!(MilliwattPerMetreKelvin, ThermalConductivity, 1e-3);

    // THERMAL RESISTANCE
    verify_unit!(KelvinPerWatt, ThermalResistance, 1.0);

    // THERMAL RESISTIVITY
    verify_unit!(MetreKelvinPerWatt, ThermalResistivity, 1.0);

    // HEAT TRANSFER COEFFICIENT
    verify_unit!(WattPerSquareMetreKelvin, HeatTransferCoefficient, 1.0);

    // THERMAL EXPANSION COEFFICIENT
    verify_unit!(PerKelvin, ThermalExpansionCoefficient, 1.0);
}
