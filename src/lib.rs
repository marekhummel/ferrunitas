// #![warn(missing_docs)]

// Ferrunitas - A type-safe unit conversion library leveraging Rust's type system
//
// This library provides compile-time dimensional analysis and unit conversions
// with zero runtime overhead through Rust's powerful type system.

pub mod common;
pub mod model;
pub mod system;

mod sealed {
    /// Public trait in private module to prevent external implementations
    pub trait Sealed {}
}

// Export Unit on crate level for easy access
pub use crate::model::unit::Unit;
// pub use crate::system_test as system;
