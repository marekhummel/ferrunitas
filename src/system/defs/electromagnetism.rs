use crate::model::quantity::Quantity;
use crate::model::unit::Unit;
use crate::system::defs::base::*;
use crate::system::defs::mechanics::*;
use crate::system::prefixes::*;
use crate::unit;
use typenum::*;
// ============================================================================
// ELECTRIC CHARGE
// ============================================================================
pub type Charge = Quantity<Z0, Z0, P1, P1, Z0, Z0, Z0>; // T I

unit!(compound: Coulomb, "C", [(Second, P1), (Ampere, P1)], prefixable);
unit!(prefix: Millicoulomb, Milli, Coulomb);
unit!(prefix: Microcoulomb, Micro, Coulomb);
unit!(prefix: Nanocoulomb, Nano, Coulomb);

// ============================================================================
// ELECTRIC POTENTIAL
// ============================================================================
pub type Potential = Quantity<P1, P2, N3, N1, Z0, Z0, Z0>; // M L² T⁻³ I⁻¹

unit!(compound: Volt, "V", [(Watt, P1), (Ampere, N1)], prefixable);
unit!(prefix: Millivolt, Milli, Volt);
unit!(prefix: Microvolt, Micro, Volt);
unit!(prefix: Kilovolt, Kilo, Volt);
unit!(prefix: Megavolt, Mega, Volt);

// ============================================================================
// CAPACITANCE
// ============================================================================
pub type Capacitance = Quantity<N1, N2, P4, P2, Z0, Z0, Z0>; // M⁻¹ L⁻² T⁴ I²

unit!(compound: Farad, "F", [(Coulomb, P1), (Volt, N1)], prefixable);
unit!(prefix: Millifarad, Milli, Farad);
unit!(prefix: Microfarad, Micro, Farad);
unit!(prefix: Nanofarad, Nano, Farad);
unit!(prefix: Picofarad, Pico, Farad);

// ============================================================================
// RESISTANCE
// ============================================================================
pub type Resistance = Quantity<P1, P2, N3, N2, Z0, Z0, Z0>; // M L² T⁻³ I⁻²

unit!(compound: Ohm, "Ω", [(Volt, P1), (Ampere, N1)], prefixable);
unit!(prefix: Milliohm, Milli, Ohm);
unit!(prefix: Kiloohm, Kilo, Ohm);
unit!(prefix: Megaohm, Mega, Ohm);

// ============================================================================
// CONDUCTANCE
// ============================================================================
pub type Conductance = Quantity<N1, N2, P3, P2, Z0, Z0, Z0>; // M⁻¹ L⁻² T³ I²

unit!(compound: Siemens, "S", [(Ohm, N1)], prefixable);
unit!(prefix: Millisiemens, Milli, Siemens);
unit!(prefix: Microsiemens, Micro, Siemens);

// ============================================================================
// MAGNETIC FLUX
// ============================================================================
pub type MagneticFlux = Quantity<P1, P2, N2, N1, Z0, Z0, Z0>; // M L² T⁻² I⁻¹

unit!(compound: Weber, "Wb", [(Volt, P1), (Second, P1)], prefixable);

// ============================================================================
// MAGNETIC FLUX DENSITY
// ============================================================================
pub type MagneticFluxDensity = Quantity<P1, Z0, N2, N1, Z0, Z0, Z0>; // M T⁻² I⁻¹

unit!(compound: Tesla, "T", [(Weber, P1), (Metre, N2)], prefixable);

unit!(derived: Gauss, "G", (1e-4, Tesla));

// ============================================================================
// INDUCTANCE
// ============================================================================
pub type Inductance = Quantity<P1, P2, N2, N2, Z0, Z0, Z0>; // M L² T⁻² I⁻²

unit!(compound: Henry, "H", [(Weber, P1), (Ampere, N1)], prefixable);
unit!(prefix: Millihenry, Milli, Henry);
unit!(prefix: Microhenry, Micro, Henry);

// ============================================================================
// ELECTRIC FIELD STRENGTH
// ============================================================================
pub type ElectricField = Quantity<P1, P1, N3, N1, Z0, Z0, Z0>; // M L T⁻³ I⁻¹

unit!(compound: VoltPerMetre, "V/m", [(Volt, P1), (Metre, N1)]);

// ============================================================================
// MAGNETIC FIELD STRENGTH
// ============================================================================
pub type MagneticFieldStrength = Quantity<Z0, N1, Z0, P1, Z0, Z0, Z0>; // L⁻¹ I

unit!(compound: AmperePerMetre, "A/m", [(Ampere, P1), (Metre, N1)]);

// ============================================================================
// ELECTRIC CONDUCTIVITY
// ============================================================================
pub type ElectricConductivity = Quantity<N1, N3, P3, P2, Z0, Z0, Z0>; // M⁻¹ L⁻³ T³ I²

unit!(compound: SiemensPerMetre, "S/m", [(Siemens, P1), (Metre, N1)]);

// ============================================================================
// ELECTRIC DISPLACEMENT FIELD
// ============================================================================
pub type ElectricDisplacement = Quantity<Z0, N2, P1, P1, Z0, Z0, Z0>; // L⁻² T I

unit!(compound: CoulombPerSquareMetre, "C/m²", [(Coulomb, P1), (SquareMetre, N1)]);

// ============================================================================
// PERMITTIVITY
// ============================================================================
pub type Permittivity = Quantity<N1, N3, P4, P2, Z0, Z0, Z0>; // M⁻¹ L⁻³ T⁴ I²

unit!(compound: FaradPerMetre, "F/m", [(Farad, P1), (Metre, N1)]);

// ============================================================================
// PERMEABILITY
// ============================================================================
pub type Permeability = Quantity<P1, P1, N2, N2, Z0, Z0, Z0>; // M L T⁻² I⁻²

unit!(compound: HenryPerMetre, "H/m", [(Henry, P1), (Metre, N1)]);

// ============================================================================
// CURRENT DENSITY
// ============================================================================
pub type CurrentDensity = Quantity<Z0, N2, Z0, P1, Z0, Z0, Z0>; // L⁻² I

unit!(compound: AmperePerSquareMetre, "A/m²", [(Ampere, P1), (SquareMetre, N1)]);

// ============================================================================
// CHARGE DENSITY
// ============================================================================
pub type ChargeDensity = Quantity<Z0, N3, P1, P1, Z0, Z0, Z0>; // L⁻³ T I

unit!(compound: CoulombPerCubicMetre, "C/m³", [(Coulomb, P1), (CubicMetre, N1)]);
