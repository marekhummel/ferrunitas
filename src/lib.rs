// #![warn(missing_docs)]

//! Ferrunitas - A type-safe unit conversion library leveraging Rust's type system
//!
//! This library provides compile-time dimensional analysis and unit conversions
//! with zero runtime overhead through Rust's powerful type system.

// Make all definitions and the common functions public
pub mod common;
pub mod system;

// Keep model internal, but re-export some types for the exported macros
mod model;

#[doc(hidden)]
pub mod __model {
    pub use crate::model::{
        dimension::{DimensionVector, DimensionZero, TypePow},
        prefix::{Prefix, Prefixable},
        quantity::Quantity,
        unit::{__inner_unit_macros, Unit},
    };
}

// Export some modules / types on crate level for easy access
pub use crate::model::measure::Measure;
pub use crate::model::unit::Unit;
pub use typenum::consts as typenum_consts;

// For manual testing, this may be used (keeps compiler overhead low)
// pub mod system_test
// pub use crate::system_test as system;
