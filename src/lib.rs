// Ferrunitas - A type-safe unit conversion library leveraging Rust's type system
//
// This library provides compile-time dimensional analysis and unit conversions
// with zero runtime overhead through Rust's powerful type system.

pub mod model;
pub mod system;

// Re-export the most commonly used items
pub use model::{prefix::Prefix, quantity::Quantity};

// TODO: Rework when defs are split into sub modules
pub use system::defs::{
    Acceleration,
    Centi,
    Centimeter,
    Energy,
    Foot,
    Force,
    Giga,
    // Base units
    Gram,
    Hour,

    Inch,
    // Prefixes
    Kilo,
    // Derived mass units
    Kilogram,
    // Derived length units
    Kilometer,
    Length,
    // Base quantities
    Mass,
    Mega,
    Meter,
    Micro,
    Mile,

    Milli,
    Millimeter,
    // Derived time units
    Minute,
    Ounce,

    Pound,
    Power,

    Second,

    Time,
    Velocity,
};
