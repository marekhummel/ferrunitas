use crate::model::quantity::Quantity;
use crate::system::defs::base::*;
use crate::system::defs::mechanics::*;
use crate::unit;
use typenum::*;

// ===========================
// HEAT CAPACITY / ENTROPY
// ===========================
pub type HeatCapacity = Quantity<P1, P2, N2, Z0, N1, Z0, Z0>; // M L² T⁻² Θ⁻¹
pub type Entropy = Quantity<P1, P2, N2, Z0, N1, Z0, Z0>; // M L² T⁻² Θ⁻¹

unit!(compound: JoulePerKelvin, "J/K", [(Joule, P1), (Kelvin, N1)]);

// ===========================
// SPECIFIC HEAT CAPACITY
// ===========================
pub type SpecificHeat = Quantity<Z0, P2, N2, Z0, N1, Z0, Z0>; // L² T⁻² Θ⁻¹

unit!(compound: JoulePerKilogramKelvin, "J/(kg·K)", [(Joule, P1), (Kilogram, N1), (Kelvin, N1)]);
unit!(compound: KilojoulePerKilogramKelvin, "kJ/(kg·K)", [(Kilojoule, P1), (Kilogram, N1), (Kelvin, N1)]);

// ===========================
// THERMAL CONDUCTIVITY
// ===========================
pub type ThermalConductivity = Quantity<P1, P1, N3, Z0, N1, Z0, Z0>; // M L T⁻³ Θ⁻¹

unit!(compound: WattPerMetreKelvin, "W/(m·K)", [(Watt, P1), (Metre, N1), (Kelvin, N1)]);
unit!(compound: MilliwattPerMetreKelvin, "mW/(m·K)", [(Milliwatt, P1), (Metre, N1), (Kelvin, N1)]);

// ===========================
// THERMAL RESISTANCE
// ===========================
pub type ThermalResistance = Quantity<N1, N2, P3, Z0, P1, Z0, Z0>; // M⁻¹ L⁻² T³ Θ

unit!(compound: KelvinPerWatt, "K/W", [(Kelvin, P1), (Watt, N1)]);

// ===========================
// THERMAL RESISTIVITY
// ===========================
pub type ThermalResistivity = Quantity<N1, N1, P3, Z0, P1, Z0, Z0>; // M⁻¹ L⁻¹ T³ Θ

unit!(compound: MetreKelvinPerWatt, "m·K/W", [(Metre, P1), (Kelvin, P1), (Watt, N1)]);

// ===========================
// THERMAL DIFFUSIVITY
// ===========================
pub type ThermalDiffusivity = Quantity<Z0, P2, N1, Z0, Z0, Z0, Z0>; // L² T⁻¹

// Already defined in mechanics for kinematic viscosity
// unit!(compound: SquareMetrePerSecond, "m²/s", [(Metre, P2), (Second, N1)]);

// ===========================
// HEAT TRANSFER COEFFICIENT
// ===========================
pub type HeatTransferCoefficient = Quantity<P1, Z0, N3, Z0, N1, Z0, Z0>; // M T⁻³ Θ⁻¹

unit!(compound: WattPerSquareMetreKelvin, "W/(m²·K)", [(Watt, P1), (Metre, N2), (Kelvin, N1)]);

// ===========================
// THERMAL EXPANSION COEFFICIENT
// ===========================
pub type ThermalExpansionCoefficient = Quantity<Z0, Z0, Z0, Z0, N1, Z0, Z0>; // Θ⁻¹

unit!(compound: PerKelvin, "K⁻¹", [(Kelvin, N1)]);

// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::verify_unit;

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
