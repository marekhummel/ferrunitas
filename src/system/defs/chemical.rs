use crate::model::quantity::Quantity;
use crate::system::defs::base::*;
use crate::system::defs::mechanics::*;
use crate::system::prefixes::*;
use crate::unit;
use typenum::*;

// ===========================
// MOLAR MASS
// ===========================
pub type MolarMass = Quantity<P1, Z0, Z0, Z0, Z0, N1, Z0>; // M N⁻¹

unit!(compound: KilogramPerMole, "kg/mol", [(Kilogram, P1), (Mole, N1)]);
unit!(compound: GramPerMole, "g/mol", [(Gram, P1), (Mole, N1)]);

// ===========================
// MOLAR CONCENTRATION
// ===========================
pub type MolarConcentration = Quantity<Z0, N3, Z0, Z0, Z0, P1, Z0>; // L⁻³ N

unit!(compound: MolePerCubicMetre, "mol/m³", [(Mole, P1), (Metre, N3)]);
unit!(compound: MolePerLitre, "mol/L", [(Mole, P1), (Litre, N1)]);

unit!(compound: MolePerDecilitre, "mol/dL", [(Mole, P1), (Decilitre, N1)]);
unit!(compound: MolePerMillilitre, "mol/mL", [(Mole, P1), (Millilitre, N1)]);
unit!(compound: MillimolePerLitre, "mmol/L", [(Millimole, P1), (Litre, N1)]);
unit!(compound: MicromolePerLitre, "μmol/L", [(Micromole, P1), (Litre, N1)]);

// ===========================
// CATALYTIC ACTIVITY
// ===========================
pub type CatalyticActivity = Quantity<Z0, Z0, N1, Z0, Z0, P1, Z0>; // N T⁻1

unit!(compound: Katal, "kat", [(Mole, P1), (Second, N1)]; prefixable);
unit!(prefix: Microkatal, Micro, Katal);
unit!(prefix: Nanokatal, Nano, Katal);

// ===========================
// MASS CONCENTRATION
// ===========================
pub type MassConcentration = Quantity<P1, N3, Z0, Z0, Z0, Z0, Z0>; // M L⁻³

unit!(compound: GramPerLitre, "g/L", [(Gram, P1), (Litre, N1)]);
unit!(compound: MilligramPerLitre, "mg/L", [(Milligram, P1), (Litre, N1)]);
unit!(compound: MicrogramPerLitre, "μg/L", [(Microgram, P1), (Litre, N1)]);

// ===========================
// CATALYTIC CONCENTRATION
// ===========================
pub type CatalyticConcentration = Quantity<Z0, N3, N1, Z0, Z0, P1, Z0>; // L⁻³ T⁻¹ N

unit!(compound: KatalPerCubicMetre, "kat/m³", [(Katal, P1), (CubicMetre, N1)]);
unit!(compound: KatalPerLitre, "kat/L", [(Katal, P1), (Litre, N1)]);

// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::verify_unit;

    // MOLAR MASS
    verify_unit!(KilogramPerMole, MolarMass, 1.0);
    verify_unit!(GramPerMole, MolarMass, 1e-3);

    // MOLAR CONCENTRATION
    verify_unit!(MolePerCubicMetre, MolarConcentration, 1.0);
    verify_unit!(MolePerLitre, MolarConcentration, 1e3);
    verify_unit!(MolePerDecilitre, MolarConcentration, 1e4);
    verify_unit!(MolePerMillilitre, MolarConcentration, 1e6);
    verify_unit!(MillimolePerLitre, MolarConcentration, 1.0);
    verify_unit!(MicromolePerLitre, MolarConcentration, 1e-3);

    // CATALYTIC ACTIVITY
    verify_unit!(Katal, CatalyticActivity, 1.0);
    verify_unit!(Microkatal, CatalyticActivity, 1e-6);
    verify_unit!(Nanokatal, CatalyticActivity, 1e-9);

    // MASS CONCENTRATION
    verify_unit!(GramPerLitre, MassConcentration, 1.0);
    verify_unit!(MilligramPerLitre, MassConcentration, 1e-3);
    verify_unit!(MicrogramPerLitre, MassConcentration, 1e-6);

    // CATALYTIC CONCENTRATION
    verify_unit!(KatalPerCubicMetre, CatalyticConcentration, 1.0);
    verify_unit!(KatalPerLitre, CatalyticConcentration, 1e3);
}
