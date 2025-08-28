// Ferrunitas - A type-safe unit conversion library leveraging Rust's type system
//
// This library provides compile-time dimensional analysis and unit conversions
// with zero runtime overhead through Rust's powerful type system.

mod common;
pub mod model;
pub mod system;

// Export Unit on crate level for easy access
pub use crate::model::unit::Unit;
