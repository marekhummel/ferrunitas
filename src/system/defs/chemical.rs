use crate::{quantity, unit};
use crate::system::defs::base::*;
use crate::system::defs::mechanics::*;
use crate::system::prefixes::*;
use typenum::*;

// ===========================
// MOLAR MASS
// ===========================
quantity!(MolarMass: M P1, L Z0, T Z0, I Z0, Th Z0, N N1, J Z0); // M N⁻¹

unit!(compound: KilogramPerMole, "kg/mol", [(Kilogram, P1), (Mole, N1)]);
unit!(compound: GramPerMole, "g/mol", [(Gram, P1), (Mole, N1)]);

// ===========================
// MOLAR CONCENTRATION
// ===========================
quantity!(MolarConcentration: M Z0, L N3, T Z0, I Z0, Th Z0, N P1, J Z0); // L⁻³ N

unit!(compound: MolePerCubicMetre, "mol/m³", [(Mole, P1), (Metre, N3)]);
unit!(compound: MolePerLitre, "mol/L", [(Mole, P1), (Litre, N1)]);

unit!(compound: MolePerDecilitre, "mol/dL", [(Mole, P1), (Decilitre, N1)]);
unit!(compound: MolePerMillilitre, "mol/mL", [(Mole, P1), (Millilitre, N1)]);
unit!(compound: MillimolePerLitre, "mmol/L", [(Millimole, P1), (Litre, N1)]);
unit!(compound: MicromolePerLitre, "μmol/L", [(Micromole, P1), (Litre, N1)]);

// ===========================
// CATALYTIC ACTIVITY
// ===========================
quantity!(CatalyticActivity: M Z0, L Z0, T N1, I Z0, Th Z0, N P1, J Z0); // N T⁻¹

unit!(compound: Katal, "kat", [(Mole, P1), (Second, N1)]; prefixable);
unit!(prefix: Microkatal, Micro, Katal);
unit!(prefix: Nanokatal, Nano, Katal);

// ===========================
// MASS CONCENTRATION
// ===========================
quantity!(MassConcentration: M P1, L N3, T Z0, I Z0, Th Z0, N Z0, J Z0); // M L⁻³

unit!(compound: GramPerLitre, "g/L", [(Gram, P1), (Litre, N1)]);
unit!(compound: MilligramPerLitre, "mg/L", [(Milligram, P1), (Litre, N1)]);
unit!(compound: MicrogramPerLitre, "μg/L", [(Microgram, P1), (Litre, N1)]);

// ===========================
// CATALYTIC CONCENTRATION
// ===========================
quantity!(CatalyticConcentration: M Z0, L N3, T N1, I Z0, Th Z0, N P1, J Z0); // L⁻³ T⁻¹ N

unit!(compound: KatalPerCubicMetre, "kat/m³", [(Katal, P1), (CubicMetre, N1)]);
unit!(compound: KatalPerLitre, "kat/L", [(Katal, P1), (Litre, N1)]);

// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::verify_unit;

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
