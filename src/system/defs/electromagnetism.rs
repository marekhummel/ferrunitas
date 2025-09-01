use crate::model::macros::{quantity, unit};
use crate::system::defs::base::*;
use crate::system::defs::mechanics::*;
use crate::system::prefixes::*;
use typenum::*;
// ===========================
// ELECTRIC CHARGE
// ===========================
quantity!(Charge: M Z0, L Z0, T P1, I P1, Th Z0, N Z0, J Z0); // T I

unit!(compound: Coulomb, "C", [(Second, P1), (Ampere, P1)]; prefixable);
unit!(prefix: Millicoulomb, Milli, Coulomb);
unit!(prefix: Microcoulomb, Micro, Coulomb);
unit!(prefix: Nanocoulomb, Nano, Coulomb);

// ===========================
// ELECTRIC POTENTIAL
// ===========================
quantity!(Potential: M P1, L P2, T N3, I N1, Th Z0, N Z0, J Z0); // M L² T⁻³ I⁻¹

unit!(compound: Volt, "V", [(Watt, P1), (Ampere, N1)]; prefixable);
unit!(prefix: Millivolt, Milli, Volt);
unit!(prefix: Microvolt, Micro, Volt);
unit!(prefix: Kilovolt, Kilo, Volt);
unit!(prefix: Megavolt, Mega, Volt);

// ===========================
// CAPACITANCE
// ===========================
quantity!(Capacitance: M N1, L N2, T P4, I P2, Th Z0, N Z0, J Z0); // M⁻¹ L⁻² T⁴ I²

unit!(compound: Farad, "F", [(Coulomb, P1), (Volt, N1)]; prefixable);
unit!(prefix: Millifarad, Milli, Farad);
unit!(prefix: Microfarad, Micro, Farad);
unit!(prefix: Nanofarad, Nano, Farad);
unit!(prefix: Picofarad, Pico, Farad);

// ===========================
// RESISTANCE
// ===========================
quantity!(Resistance: M P1, L P2, T N3, I N2, Th Z0, N Z0, J Z0); // M L² T⁻³ I⁻²

unit!(compound: Ohm, "Ω", [(Volt, P1), (Ampere, N1)]; prefixable);
unit!(prefix: Milliohm, Milli, Ohm);
unit!(prefix: Kiloohm, Kilo, Ohm);
unit!(prefix: Megaohm, Mega, Ohm);

// ===========================
// CONDUCTANCE
// ===========================
quantity!(Conductance: M N1, L N2, T P3, I P2, Th Z0, N Z0, J Z0); // M⁻¹ L⁻² T³ I²

unit!(compound: Siemens, "S", [(Ohm, N1)]; prefixable);
unit!(prefix: Millisiemens, Milli, Siemens);
unit!(prefix: Microsiemens, Micro, Siemens);

// ===========================
// MAGNETIC FLUX
// ===========================
quantity!(MagneticFlux: M P1, L P2, T N2, I N1, Th Z0, N Z0, J Z0); // M L² T⁻² I⁻¹

unit!(compound: Weber, "Wb", [(Volt, P1), (Second, P1)]; prefixable);

// ===========================
// MAGNETIC FLUX DENSITY
// ===========================
quantity!(MagneticFluxDensity: M P1, L Z0, T N2, I N1, Th Z0, N Z0, J Z0); // M T⁻² I⁻¹

unit!(compound: Tesla, "T", [(Weber, P1), (Metre, N2)]; prefixable);

unit!(derived: Gauss, "G", (1e-4, Tesla));

// ===========================
// INDUCTANCE
// ===========================
quantity!(Inductance: M P1, L P2, T N2, I N2, Th Z0, N Z0, J Z0); // M L² T⁻² I⁻²

unit!(compound: Henry, "H", [(Weber, P1), (Ampere, N1)]; prefixable);
unit!(prefix: Millihenry, Milli, Henry);
unit!(prefix: Microhenry, Micro, Henry);

// ===========================
// ELECTRIC FIELD STRENGTH
// ===========================
quantity!(ElectricField: M P1, L P1, T N3, I N1, Th Z0, N Z0, J Z0); // M L T⁻³ I⁻¹

unit!(compound: VoltPerMetre, "V/m", [(Volt, P1), (Metre, N1)]);

// ===========================
// MAGNETIC FIELD STRENGTH
// ===========================
quantity!(MagneticFieldStrength: M Z0, L N1, T Z0, I P1, Th Z0, N Z0, J Z0); // L⁻¹ I

unit!(compound: AmperePerMetre, "A/m", [(Ampere, P1), (Metre, N1)]);

// ===========================
// ELECTRIC CONDUCTIVITY
// ===========================
quantity!(ElectricConductivity: M N1, L N3, T P3, I P2, Th Z0, N Z0, J Z0); // M⁻¹ L⁻³ T³ I²

unit!(compound: SiemensPerMetre, "S/m", [(Siemens, P1), (Metre, N1)]);

// ===========================
// ELECTRIC DISPLACEMENT FIELD
// ===========================
quantity!(ElectricDisplacement: M Z0, L N2, T P1, I P1, Th Z0, N Z0, J Z0); // L⁻² T I

unit!(compound: CoulombPerSquareMetre, "C/m²", [(Coulomb, P1), (SquareMetre, N1)]);

// ===========================
// PERMITTIVITY
// ===========================
quantity!(Permittivity: M N1, L N3, T P4, I P2, Th Z0, N Z0, J Z0); // M⁻¹ L⁻³ T⁴ I²

unit!(compound: FaradPerMetre, "F/m", [(Farad, P1), (Metre, N1)]);

// ===========================
// PERMEABILITY
// ===========================
quantity!(Permeability: M P1, L P1, T N2, I N2, Th Z0, N Z0, J Z0); // M L T⁻² I⁻²

unit!(compound: HenryPerMetre, "H/m", [(Henry, P1), (Metre, N1)]);

// ===========================
// CURRENT DENSITY
// ===========================
quantity!(CurrentDensity: M Z0, L N2, T Z0, I P1, Th Z0, N Z0, J Z0); // L⁻² I

unit!(compound: AmperePerSquareMetre, "A/m²", [(Ampere, P1), (SquareMetre, N1)]);

// ===========================
// CHARGE DENSITY
// ===========================
quantity!(ChargeDensity: M Z0, L N3, T P1, I P1, Th Z0, N Z0, J Z0); // L⁻³ T I

unit!(compound: CoulombPerCubicMetre, "C/m³", [(Coulomb, P1), (CubicMetre, N1)]);

// ===========================
// MAGNETIC MOMENT
// ===========================
quantity!(MagneticMoment: M Z0, L P2, T Z0, I P1, Th Z0, N Z0, J Z0); // L² I

unit!(compound: AmpereSquareMetre, "A⋅m²", [(Ampere, P1), (SquareMetre, P1)]);
unit!(compound: JoulePerTesla, "J/T", [(Joule, P1), (Tesla, N1)]);

// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::verify_unit;

    // ELECTRIC CHARGE
    verify_unit!(Coulomb, Charge, 1.0);
    verify_unit!(Millicoulomb, Charge, 0.001);
    verify_unit!(Microcoulomb, Charge, 1e-6);
    verify_unit!(Nanocoulomb, Charge, 1e-9);

    // ELECTRIC POTENTIAL
    verify_unit!(Volt, Potential, 1.0);
    verify_unit!(Millivolt, Potential, 1e-3);
    verify_unit!(Microvolt, Potential, 1e-6);
    verify_unit!(Kilovolt, Potential, 1e3);
    verify_unit!(Megavolt, Potential, 1e6);

    // CAPACITANCE
    verify_unit!(Farad, Capacitance, 1.0);
    verify_unit!(Millifarad, Capacitance, 1e-3);
    verify_unit!(Microfarad, Capacitance, 1e-6);
    verify_unit!(Nanofarad, Capacitance, 1e-9);
    verify_unit!(Picofarad, Capacitance, 1e-12);

    // RESISTANCE
    verify_unit!(Ohm, Resistance, 1.0);
    verify_unit!(Milliohm, Resistance, 1e-3);
    verify_unit!(Kiloohm, Resistance, 1e3);
    verify_unit!(Megaohm, Resistance, 1e6);

    // CONDUCTANCE
    verify_unit!(Siemens, Conductance, 1.0);
    verify_unit!(Millisiemens, Conductance, 1e-3);
    verify_unit!(Microsiemens, Conductance, 1e-6);

    // MAGNETIC FLUX
    verify_unit!(Weber, MagneticFlux, 1.0);

    // MAGNETIC FLUX DENSITY
    verify_unit!(Tesla, MagneticFluxDensity, 1.0);
    verify_unit!(Gauss, MagneticFluxDensity, 1e-4);

    // INDUCTANCE
    verify_unit!(Henry, Inductance, 1.0);
    verify_unit!(Millihenry, Inductance, 1e-3);
    verify_unit!(Microhenry, Inductance, 1e-6);

    // ELECTRIC FIELD STRENGTH
    verify_unit!(VoltPerMetre, ElectricField, 1.0);

    // MAGNETIC FIELD STRENGTH
    verify_unit!(AmperePerMetre, MagneticFieldStrength, 1.0);

    // ELECTRIC CONDUCTIVITY
    verify_unit!(SiemensPerMetre, ElectricConductivity, 1.0);

    // ELECTRIC DISPLACEMENT FIELD
    verify_unit!(CoulombPerSquareMetre, ElectricDisplacement, 1.0);

    // PERMITTIVITY
    verify_unit!(FaradPerMetre, Permittivity, 1.0);

    // PERMEABILITY
    verify_unit!(HenryPerMetre, Permeability, 1.0);

    // CURRENT DENSITY
    verify_unit!(AmperePerSquareMetre, CurrentDensity, 1.0);

    // CHARGE DENSITY
    verify_unit!(CoulombPerCubicMetre, ChargeDensity, 1.0);

    // MAGNETIC MOMENT
    verify_unit!(AmpereSquareMetre, MagneticMoment, 1.0);
    verify_unit!(JoulePerTesla, MagneticMoment, 1.0);
}
