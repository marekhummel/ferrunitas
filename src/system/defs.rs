use crate::model::quantity::Quantity;
use typenum::*;

// ============================================================================
// Base SI Quantities as Type Aliases
// ============================================================================

pub type Mass = Quantity<P1, Z0, Z0, Z0, Z0, Z0, Z0>; // [1,0,0,0,0,0,0]
pub type Length = Quantity<Z0, P1, Z0, Z0, Z0, Z0, Z0>; // [0,1,0,0,0,0,0]
pub type Time = Quantity<Z0, Z0, P1, Z0, Z0, Z0, Z0>; // [0,0,1,0,0,0,0]

// Derived quantities
pub type Velocity = Quantity<Z0, P1, N1, Z0, Z0, Z0, Z0>; // [0,1,-1,0,0,0,0] = L/T
pub type Acceleration = Quantity<Z0, P1, N2, Z0, Z0, Z0, Z0>; // [0,1,-2,0,0,0,0] = L/T²
pub type Force = Quantity<P1, P1, N2, Z0, Z0, Z0, Z0>; // [1,1,-2,0,0,0,0] = M⋅L/T²
pub type Energy = Quantity<P1, P2, N2, Z0, Z0, Z0, Z0>; // [1,2,-2,0,0,0,0] = M⋅L²/T²
pub type Power = Quantity<P1, P2, N3, Z0, Z0, Z0, Z0>; // [1,2,-3,0,0,0,0] = M⋅L²/T³

// ============================================================================
// Unit Constructor Functions
// ============================================================================

// Base units (using gram as mass base)
pub fn gram(value: f64) -> Mass {
    Mass::new(value)
}

pub fn meter(value: f64) -> Length {
    Length::new(value)
}

pub fn second(value: f64) -> Time {
    Time::new(value)
}

// Prefixed base units
pub fn kilogram(value: f64) -> Mass {
    Mass::new(value * 1000.0) // Convert to grams
}

pub fn centimeter(value: f64) -> Length {
    Length::new(value * 0.01) // Convert to meters
}

pub fn millimeter(value: f64) -> Length {
    Length::new(value * 0.001) // Convert to meters
}

pub fn kilometer(value: f64) -> Length {
    Length::new(value * 1000.0) // Convert to meters
}

// Derived units
pub fn newton(value: f64) -> Force {
    Force::new(value * 1000.0) // 1 N = 1000 g⋅m/s²
}

pub fn joule(value: f64) -> Energy {
    Energy::new(value * 1000.0) // 1 J = 1000 g⋅m²/s²
}

pub fn watt(value: f64) -> Power {
    Power::new(value * 1000.0) // 1 W = 1000 g⋅m²/s³
}

// ============================================================================
// Unit Conversion Functions
// ============================================================================

pub fn to_kilograms(mass: Mass) -> f64 {
    mass.value() / 1000.0
}

pub fn to_grams(mass: Mass) -> f64 {
    mass.value()
}

pub fn to_meters(length: Length) -> f64 {
    length.value()
}

pub fn to_kilometers(length: Length) -> f64 {
    length.value() / 1000.0
}

pub fn to_centimeters(length: Length) -> f64 {
    length.value() / 0.01
}

pub fn to_seconds(time: Time) -> f64 {
    time.value()
}

pub fn to_newtons(force: Force) -> f64 {
    force.value() / 1000.0
}

pub fn to_joules(energy: Energy) -> f64 {
    energy.value() / 1000.0
}

pub fn to_watts(power: Power) -> f64 {
    power.value() / 1000.0
}
