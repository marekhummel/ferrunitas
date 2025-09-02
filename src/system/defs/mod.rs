//! Grouped domain-specific unit & quantity definitions.
//!
//! Each submodule declares quantities and concrete units for a physical
//! domain (mechanics, electromagnetism, chemistry, etc.). Files are largely
//! macro invocations generating unit structs & associated compile-time data.

pub mod acoustic;
pub mod base;
pub mod chemical;
pub mod dimensionless;
pub mod electromagnetism;
pub mod mechanics;
pub mod photometric;
pub mod radiation;
pub mod thermodynamics;
