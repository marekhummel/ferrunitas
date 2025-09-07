//! Ferrunitas – Type‑safe physical quantities & units
//!
//! Ferrunitas lets you express physical formulas with compile‑time dimensional
//! analysis. Using incompatible units results in a compile error
//! instead of a runtime surprise. Values are stored as raw `f64`, while *types*
//! encode the dimensional signature (mass, length, time, …) via the type system.
//!
//! # Goals
//! * Zero / near‑zero runtime overhead (all dimension checks at compile time)
//! * Ergonomic construction & conversion of units (e.g. `Kilogram::new(5.0)`)
//! * Seamless arithmetic on both concrete units (e.g. `Newton`) and generic
//!   quantities (e.g. `Force`)
//! * Prefix & compound unit support (kilo, milli, square, per, etc.)
//! * Optional quantity tagging to distinguish semantically different but dimensionally identical quantities
//!
//! # Core Concepts
//! * **Unit type**: A concrete label like `Kilogram`, `Metre`, `Newton` that
//!   carries its dimensional vector & scaling factor to an internal base.
//! * **Quantity**: A dimensioned numeric value without a fixed display unit.
//!   You obtain a quantity via `some_unit.into_q()` and it will be the result of
//!   multiplication and division. They work best when converted back to measures
//!   and as types for methods.
//! * **Quantity Tags** (optional): When the `quantity_tags` feature is enabled,
//!   quantities can carry additional type information to distinguish between
//!   dimensionally identical but semantically different values (e.g., `Angle` vs `Information`).
//! * **Prefixes / Compounds**: Generated via macros (e.g. `Millivolt`,
//!   `SquareMetre`, `MetrePerSecond`). These are normal unit types.
//! * **Conversion**: Convert a quantity back to a chosen unit type with
//!   `quantity.as_measure::<TargetUnit>()` or convert between unit types by going
//!   through a quantity: `let kg: Kilogram = grams.convert();`.
//!
//! # Quick Start
//! ```rust
//! use ferrunitas::{system::*, Measure, Unit};
//!
//! // Construct concrete units
//! let m = Kilogram::new(5.0);              // 5 kg
//! let a = MetrePerSecondSquared::new(2.0); // 2 m/s²
//!
//! // Multiply → force (unit × unit -> compound quantity)
//! let f_q = m * a + Millinewton::new(20.0);                // A quantity of type Force
//! let f: Measure<Newton> = f_q.as_measure();  // Represent it as Newtons
//! assert_eq!(f.value(), 10.02);
//!
//! // Unit ↔ Quantity ↔ Unit conversion
//! let g = Gram::new(5000.0);           // 5000 g
//! let kg_again = g.convert::<Kilogram>();
//! assert_eq!(kg_again.value(), 5.0);
//! ```
//!
//! # Quantity Tagging (Optional Feature)
//! Enable with `--features="quantity_tags"` to distinguish between dimensionally
//! equivalent but semantically different quantities:
//!
//! ```rust
//! use ferrunitas::{system::*, Unit};
//! // These are both dimensionless but represent different concepts
//! let data = Bit::new(8.0);
//! let angle = Radian::new(1.0);
//!
//! #[cfg(feature = "quantity_tags")]
//! {
//!     // With quantity_tags enabled, this won't compile:
//!     // let invalid = data + angle;  // Error!
//!
//!     // Instead, explicitly specify the target quantity:
//!     let result = data.into_q().specify::<Dimensionless>() +
//!                  angle.into_q().specify::<Dimensionless>();
//! }
//! ```
//!
//! # Relevant imports
//! ```rust
//! // For access to all definitions to units, quantities and prefixes
//! use ferrunitas::system::*;
//!
//! // Crate root access to most common struct Measure and trait Unit to make methods available
//! use ferrunitas::{Measure, Unit};
//!
//! // For dimensional formatting
//! use ferrunitas::common::{format_dims, format_unit_dims};
//!
//! // Macros for own definitions (typenum consts for quantity macro)
//! use ferrunitas::typenum_consts::*;
//! use ferrunitas::{prefix, quantity, unit};
//! ```
//!
//! # Equality & Ordering
//! * Concrete unit types (`Kilogram`, `Gram`) are **not** directly comparable to
//!   each other if their Rust types differ. Convert both to a quantity (or the
//!   same concrete unit) first. Use `.is_equal_to()` for comparisons.
//! * Quantities of the same dimension implement `PartialEq` / `PartialOrd`.
//!
//! # Arithmetic Overview
//! | Operation | Works On | Result |
//! |-----------|----------|--------|
//! | `+ -`     | Same dimension (unit or quantity) | Same dimension |
//! | `* /`     | Any compatible dimensions | New derived dimension |
//! | `* f64` / `f64 *` | Scale value | Same dimension |
//! | `/ f64`   | Scale value | Same dimension |
//!
//! # Limitations / Notes
//! * Internal storage uses `f64`; typical floating point caveats apply, see examples/misc.rs.
//! * Rounding / formatting of display values is a caller concern, usual format specifiers are respected.
//! * Offsets are supported for temperature scales, but are quite limited in usage besides simple conversion.
//!
//! # Extending
//! New units, prefixes, or compound relationships can be created using
//! the provided macros (`unit!`, etc.). The results work just like all the
//! predefined macros.
//!
//! # Minimum Example (copy/paste)
//! ```rust
//! use ferrunitas::{system::{Kilometre, Knot, Second, Yard}, Unit};
//! let a1 = Kilometre::new(1.2);
//! let a2 = Yard::new(233.0);
//! let b = Second::new(3400.0);
//! let speed = ((a1 + a2) / b).as_measure::<Knot>();
//! println!("{:.2}", speed);
//! ```

// Make all definitions and the common functions public
/// Common helper functions & utilities shared across units / quantities.
pub mod common;
/// Public system definitions: base, derived, prefixed, and compound units.
pub mod system;

// Keep model internal, but re-export some types for the exported macros
mod model;

#[doc(hidden)]
pub mod __model {
    pub use crate::model::{
        dimension::{DimensionVector, DimensionZero, TypePow},
        prefix::{Prefix, Prefixable},
        quantity::{Quantity, QuantityMarker, QuantityTag},
        unit::{__inner_unit_macros, Unit},
    };
}

// Export some modules / types on crate level for easy access
pub use crate::model::measure::Measure;
pub use crate::model::unit::Unit;
pub use typenum::consts as typenum_consts;

// For manual testing, this may be used (keeps compiler overhead low)
// pub mod system_test;
// pub use crate::system_test as system;
