//! Public unit system namespace.
//!
//! Aggregates all predefined physical domain unit definitions (base, derived,
//! compound) plus SI prefixes & selected constants. To access any unit, import
//! from here: `use ferrunitas::system::*;`.

// Modules can stay private, as all members are publicly re-exported
mod constants;
mod defs;
mod prefixes;

// Public export of all prefixes, constants and units under ferrunitas::system::*
pub use constants::*;
pub use prefixes::*;

pub use defs::acoustic::*;
pub use defs::base::*;
pub use defs::chemical::*;
pub use defs::computing::*;
pub use defs::dimensionless::*;
pub use defs::electromagnetism::*;
pub use defs::mechanics::*;
pub use defs::photometric::*;
pub use defs::radiation::*;
pub use defs::thermodynamics::*;
