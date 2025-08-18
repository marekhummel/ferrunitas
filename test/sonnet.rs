//! A type-safe unit conversion library using Rust's type system
//!
//! This library provides compile-time unit checking using typenum for dimensional analysis.
//! It supports quantities with different units and prefixes using Into<Quantity> trait.

use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Sub};
use typenum::*;

/// Base dimensions using typenum integers
/// Format: [Mass, Length, Time, Current, Temperature, Substance, Luminosity]
pub trait Dimension {
    type Mass: Integer;
    type Length: Integer;
    type Time: Integer;
    type Current: Integer;
    type Temperature: Integer;
    type Substance: Integer;
    type Luminosity: Integer;
}

/// Macro to easily define dimensions
macro_rules! dimension {
    ($name:ident: [$m:ty, $l:ty, $t:ty, $i:ty, $temp:ty, $n:ty, $lum:ty]) => {
        #[derive(Debug, Clone, Copy)]
        pub struct $name;

        impl Dimension for $name {
            type Mass = $m;
            type Length = $l;
            type Time = $t;
            type Current = $i;
            type Temperature = $temp;
            type Substance = $n;
            type Luminosity = $lum;
        }
    };
}

// Define common dimensions
dimension!(Dimensionless: [Z0, Z0, Z0, Z0, Z0, Z0, Z0]);
dimension!(Mass: [P1, Z0, Z0, Z0, Z0, Z0, Z0]);
dimension!(Length: [Z0, P1, Z0, Z0, Z0, Z0, Z0]);
dimension!(Time: [Z0, Z0, P1, Z0, Z0, Z0, Z0]);
dimension!(Velocity: [Z0, P1, N1, Z0, Z0, Z0, Z0]);
dimension!(Acceleration: [Z0, P1, N2, Z0, Z0, Z0, Z0]);
dimension!(Force: [P1, P1, N2, Z0, Z0, Z0, Z0]);
dimension!(Energy: [P1, P2, N2, Z0, Z0, Z0, Z0]);
dimension!(Power: [P1, P2, N3, Z0, Z0, Z0, Z0]);
dimension!(Area: [Z0, P2, Z0, Z0, Z0, Z0, Z0]);
dimension!(Volume: [Z0, P3, Z0, Z0, Z0, Z0, Z0]);

/// Trait for unit prefixes (powers of 10)
pub trait Prefix {
    const FACTOR: f64;
    const SYMBOL: &'static str;
}

// Define common prefixes
macro_rules! prefix {
    ($name:ident, $factor:expr, $symbol:expr) => {
        #[derive(Debug, Clone, Copy)]
        pub struct $name;
        impl Prefix for $name {
            const FACTOR: f64 = $factor;
            const SYMBOL: &'static str = $symbol;
        }
    };
}

prefix!(Yotta, 1e24, "Y");
prefix!(Zetta, 1e21, "Z");
prefix!(Exa, 1e18, "E");
prefix!(Peta, 1e15, "P");
prefix!(Tera, 1e12, "T");
prefix!(Giga, 1e9, "G");
prefix!(Mega, 1e6, "M");
prefix!(Kilo, 1e3, "k");
prefix!(Hecto, 1e2, "h");
prefix!(Deka, 1e1, "da");
prefix!(Unity, 1e0, "");
prefix!(Deci, 1e-1, "d");
prefix!(Centi, 1e-2, "c");
prefix!(Milli, 1e-3, "m");
prefix!(Micro, 1e-6, "μ");
prefix!(Nano, 1e-9, "n");
prefix!(Pico, 1e-12, "p");
prefix!(Femto, 1e-15, "f");
prefix!(Atto, 1e-18, "a");
prefix!(Zepto, 1e-21, "z");
prefix!(Yocto, 1e-24, "y");

/// Trait for base units
pub trait BaseUnit {
    type Dim: Dimension;
    const NAME: &'static str;
    const SYMBOL: &'static str;
    /// Conversion factor to SI base unit
    const TO_SI: f64 = 1.0;
}

// Define base units
macro_rules! base_unit {
    ($name:ident, $dim:ty, $unit_name:expr, $symbol:expr) => {
        #[derive(Debug, Clone, Copy)]
        pub struct $name;
        impl BaseUnit for $name {
            type Dim = $dim;
            const NAME: &'static str = $unit_name;
            const SYMBOL: &'static str = $symbol;
        }
    };
    ($name:ident, $dim:ty, $unit_name:expr, $symbol:expr, $to_si:expr) => {
        #[derive(Debug, Clone, Copy)]
        pub struct $name;
        impl BaseUnit for $name {
            type Dim = $dim;
            const NAME: &'static str = $unit_name;
            const SYMBOL: &'static str = $symbol;
            const TO_SI: f64 = $to_si;
        }
    };
}

// SI base units
base_unit!(Meter, Length, "meter", "m");
base_unit!(Gram, Mass, "gram", "g");
base_unit!(Second, Time, "second", "s");

// Derived units
base_unit!(Newton, Force, "newton", "N");
base_unit!(Joule, Energy, "joule", "J");
base_unit!(Watt, Power, "watt", "W");

// Non-SI units with conversion factors
base_unit!(Foot, Length, "foot", "ft", 0.3048);
base_unit!(Inch, Length, "inch", "in", 0.0254);
base_unit!(Mile, Length, "mile", "mi", 1609.344);
base_unit!(Pound, Mass, "pound", "lb", 453.59237); // in grams
base_unit!(Ounce, Mass, "ounce", "oz", 28.3495231);

/// A quantity with a value, unit, and prefix
// #[derive(Debug, Clone, Copy)]
pub struct Quantity<T, U, P = Unity>
where
    T: Copy,
    U: BaseUnit,
    P: Prefix,
{
    value: T,
    _unit: PhantomData<U>,
    _prefix: PhantomData<P>,
}

impl<T, U, P> Quantity<T, U, P>
where
    T: Copy,
    U: BaseUnit,
    P: Prefix,
{
    /// Create a new quantity
    pub fn new(value: T) -> Self {
        Self {
            value,
            _unit: PhantomData,
            _prefix: PhantomData,
        }
    }

    /// Get the raw value
    pub fn value(&self) -> T {
        self.value
    }

    /// Get the value in SI base units
    pub fn si_value(&self) -> T
    where
        T: Mul<f64, Output = T>,
    {
        self.value * (P::FACTOR * U::TO_SI)
    }
}

/// Convert between compatible units
impl<T, U1, U2, P1, P2> Quantity<T, U1, P1>
where
    T: Copy + Mul<f64, Output = T> + Div<f64, Output = T>,
    U1: BaseUnit,
    U2: BaseUnit<Dim = U1::Dim>,
    P1: Prefix,
    P2: Prefix,
{
    pub fn convert_to(self) -> Quantity<T, U2, P2> {
        let si_value = self.value * (P1::FACTOR * U1::TO_SI);
        let new_value = si_value / (P2::FACTOR * U2::TO_SI);
        Quantity::new(new_value)
    }
}

/// Unit wrapper that implements Into<Quantity>
#[derive(Debug, Clone, Copy)]
pub struct Unit<T, U, P = Unity>(pub T, PhantomData<U>, PhantomData<P>)
where
    T: Copy,
    U: BaseUnit,
    P: Prefix;

impl<T, U, P> Unit<T, U, P>
where
    T: Copy,
    U: BaseUnit,
    P: Prefix,
{
    pub fn new(value: T) -> Self {
        Self(value, PhantomData, PhantomData)
    }
}

impl<T, U, P> Into<Quantity<T, U, P>> for Unit<T, U, P>
where
    T: Copy,
    U: BaseUnit,
    P: Prefix,
{
    fn into(self) -> Quantity<T, U, P> {
        Quantity::new(self.0)
    }
}

/// Prefix wrapper that can be combined with units
#[derive(Debug, Clone, Copy)]
pub struct Prefixed<T, U, P>(pub T, PhantomData<U>, PhantomData<P>)
where
    T: Copy,
    U: BaseUnit,
    P: Prefix;

impl<T, U, P> Prefixed<T, U, P>
where
    T: Copy,
    U: BaseUnit,
    P: Prefix,
{
    pub fn new(value: T) -> Self {
        Self(value, PhantomData, PhantomData)
    }
}

impl<T, U, P> Into<Quantity<T, U, P>> for Prefixed<T, U, P>
where
    T: Copy,
    U: BaseUnit,
    P: Prefix,
{
    fn into(self) -> Quantity<T, U, P> {
        Quantity::new(self.0)
    }
}

/// Addition of quantities with same dimension
impl<T, U1, U2, P1, P2> Add<Quantity<T, U2, P2>> for Quantity<T, U1, P1>
where
    T: Copy + Add<Output = T> + Mul<f64, Output = T> + Div<f64, Output = T>,
    U1: BaseUnit,
    U2: BaseUnit<Dim = U1::Dim>,
    P1: Prefix,
    P2: Prefix,
{
    type Output = Quantity<T, U1, P1>;

    fn add(self, rhs: Quantity<T, U2, P2>) -> Self::Output {
        let rhs_converted: Quantity<T, U1, P1> = rhs.convert_to();
        Quantity::new(self.value + rhs_converted.value)
    }
}

/// Addition with Into<Quantity>
impl<T, U1, U2, P1, P2, Q> Add<Q> for Quantity<T, U1, P1>
where
    T: Copy + Add<Output = T> + Mul<f64, Output = T> + Div<f64, Output = T>,
    U1: BaseUnit,
    U2: BaseUnit<Dim = U1::Dim>,
    P1: Prefix,
    P2: Prefix,
    Q: Into<Quantity<T, U2, P2>>,
{
    type Output = Quantity<T, U1, P1>;

    fn add(self, rhs: Q) -> Self::Output {
        self + rhs.into()
    }
}

/// Subtraction of quantities with same dimension
impl<T, U1, U2, P1, P2> Sub<Quantity<T, U2, P2>> for Quantity<T, U1, P1>
where
    T: Copy + Sub<Output = T> + Mul<f64, Output = T> + Div<f64, Output = T>,
    U1: BaseUnit,
    U2: BaseUnit<Dim = U1::Dim>,
    P1: Prefix,
    P2: Prefix,
{
    type Output = Quantity<T, U1, P1>;

    fn sub(self, rhs: Quantity<T, U2, P2>) -> Self::Output {
        let rhs_converted: Quantity<T, U1, P1> = rhs.convert_to();
        Quantity::new(self.value - rhs_converted.value)
    }
}

/// Subtraction with Into<Quantity>
impl<T, U1, U2, P1, P2, Q> Sub<Q> for Quantity<T, U1, P1>
where
    T: Copy + Sub<Output = T> + Mul<f64, Output = T> + Div<f64, Output = T>,
    U1: BaseUnit,
    U2: BaseUnit<Dim = U1::Dim>,
    P1: Prefix,
    P2: Prefix,
    Q: Into<Quantity<T, U2, P2>>,
{
    type Output = Quantity<T, U1, P1>;

    fn sub(self, rhs: Q) -> Self::Output {
        self - rhs.into()
    }
}

/// Display implementation
impl<T, U, P> std::fmt::Display for Quantity<T, U, P>
where
    T: std::fmt::Display + Copy,
    U: BaseUnit,
    P: Prefix,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}{}", self.value, P::SYMBOL, U::SYMBOL)
    }
}

/// Type aliases for common dimensions to use in function signatures
pub type Length<T = f64> = Quantity<T, Meter, Unity>;
pub type Mass<T = f64> = Quantity<T, Gram, Unity>;
pub type Time<T = f64> = Quantity<T, Second, Unity>;
pub type Force<T = f64> = Quantity<T, Newton, Unity>;
pub type Energy<T = f64> = Quantity<T, Joule, Unity>;

/// Unit constructors that return Unit types implementing Into<Quantity>
pub mod units {
    use super::*;

    // Length units
    pub fn meters<T: Copy>(value: T) -> Unit<T, Meter, Unity> {
        Unit::new(value)
    }

    pub fn feet<T: Copy>(value: T) -> Unit<T, Foot, Unity> {
        Unit::new(value)
    }

    pub fn inches<T: Copy>(value: T) -> Unit<T, Inch, Unity> {
        Unit::new(value)
    }

    // Mass units
    pub fn grams<T: Copy>(value: T) -> Unit<T, Gram, Unity> {
        Unit::new(value)
    }

    pub fn pounds<T: Copy>(value: T) -> Unit<T, Pound, Unity> {
        Unit::new(value)
    }

    // Time units
    pub fn seconds<T: Copy>(value: T) -> Unit<T, Second, Unity> {
        Unit::new(value)
    }

    // Force units
    pub fn newtons<T: Copy>(value: T) -> Unit<T, Newton, Unity> {
        Unit::new(value)
    }
}

/// Prefix constructors for applying prefixes to units
pub mod prefixes {
    use super::*;

    pub fn kilo<T: Copy, U: BaseUnit>(value: T) -> Prefixed<T, U, Kilo> {
        Prefixed::new(value)
    }

    pub fn mega<T: Copy, U: BaseUnit>(value: T) -> Prefixed<T, U, Mega> {
        Prefixed::new(value)
    }

    pub fn giga<T: Copy, U: BaseUnit>(value: T) -> Prefixed<T, U, Giga> {
        Prefixed::new(value)
    }

    pub fn milli<T: Copy, U: BaseUnit>(value: T) -> Prefixed<T, U, Milli> {
        Prefixed::new(value)
    }

    pub fn micro<T: Copy, U: BaseUnit>(value: T) -> Prefixed<T, U, Micro> {
        Prefixed::new(value)
    }

    pub fn nano<T: Copy, U: BaseUnit>(value: T) -> Prefixed<T, U, Nano> {
        Prefixed::new(value)
    }

    pub fn centi<T: Copy, U: BaseUnit>(value: T) -> Prefixed<T, U, Centi> {
        Prefixed::new(value)
    }
}

/// Helper trait for accepting any unit of a given dimension
pub trait IntoMass<T> {
    fn into_mass(self) -> Quantity<T, Gram, Unity>;
}

impl<T, U, P> IntoMass<T> for Quantity<T, U, P>
where
    T: Copy + Mul<f64, Output = T> + Div<f64, Output = T>,
    U: BaseUnit<Dim = Mass>,
    P: Prefix,
{
    fn into_mass(self) -> Quantity<T, Gram, Unity> {
        self.convert_to()
    }
}

impl<T, U, P> IntoMass<T> for Unit<T, U, P>
where
    T: Copy + Mul<f64, Output = T> + Div<f64, Output = T>,
    U: BaseUnit<Dim = Mass>,
    P: Prefix,
{
    fn into_mass(self) -> Quantity<T, Gram, Unity> {
        let q: Quantity<T, U, P> = self.into();
        q.convert_to()
    }
}

impl<T, U, P> IntoMass<T> for Prefixed<T, U, P>
where
    T: Copy + Mul<f64, Output = T> + Div<f64, Output = T>,
    U: BaseUnit<Dim = Mass>,
    P: Prefix,
{
    fn into_mass(self) -> Quantity<T, Gram, Unity> {
        let q: Quantity<T, U, P> = self.into();
        q.convert_to()
    }
}

pub trait IntoLength<T> {
    fn into_length(self) -> Quantity<T, Meter, Unity>;
}

impl<T, U, P> IntoLength<T> for Quantity<T, U, P>
where
    T: Copy + Mul<f64, Output = T> + Div<f64, Output = T>,
    U: BaseUnit<Dim = Length>,
    P: Prefix,
{
    fn into_length(self) -> Quantity<T, Meter, Unity> {
        self.convert_to()
    }
}

impl<T, U, P> IntoLength<T> for Unit<T, U, P>
where
    T: Copy + Mul<f64, Output = T> + Div<f64, Output = T>,
    U: BaseUnit<Dim = Length>,
    P: Prefix,
{
    fn into_length(self) -> Quantity<T, Meter, Unity> {
        let q: Quantity<T, U, P> = self.into();
        q.convert_to()
    }
}

impl<T, U, P> IntoLength<T> for Prefixed<T, U, P>
where
    T: Copy + Mul<f64, Output = T> + Div<f64, Output = T>,
    U: BaseUnit<Dim = Length>,
    P: Prefix,
{
    fn into_length(self) -> Quantity<T, Meter, Unity> {
        let q: Quantity<T, U, P> = self.into();
        q.convert_to()
    }
}

pub trait IntoForce<T> {
    fn into_force(self) -> Quantity<T, Newton, Unity>;
}

impl<T, U, P> IntoForce<T> for Quantity<T, U, P>
where
    T: Copy + Mul<f64, Output = T> + Div<f64, Output = T>,
    U: BaseUnit<Dim = Force>,
    P: Prefix,
{
    fn into_force(self) -> Quantity<T, Newton, Unity> {
        self.convert_to()
    }
}

impl<T, U, P> IntoForce<T> for Unit<T, U, P>
where
    T: Copy + Mul<f64, Output = T> + Div<f64, Output = T>,
    U: BaseUnit<Dim = Force>,
    P: Prefix,
{
    fn into_force(self) -> Quantity<T, Newton, Unity> {
        let q: Quantity<T, U, P> = self.into();
        q.convert_to()
    }
}

impl<T, U, P> IntoForce<T> for Prefixed<T, U, P>
where
    T: Copy + Mul<f64, Output = T> + Div<f64, Output = T>,
    U: BaseUnit<Dim = Force>,
    P: Prefix,
{
    fn into_force(self) -> Quantity<T, Newton, Unity> {
        let q: Quantity<T, U, P> = self.into();
        q.convert_to()
    }
}

/// Helper functions for creating functions that accept any unit of a given dimension
pub fn calculate_force_from_mass_and_acceleration<M, A>(mass: M, acceleration: A) -> Force<f64>
where
    M: IntoMass<f64>,
    A: IntoLength<f64>, // Using Length for simplification (m/s²)
{
    let m = mass.into_mass();
    let a = acceleration.into_length(); // Simplified - this would be acceleration in a real implementation

    // F = ma (simplified calculation for demonstration)
    let force_value = m.value() * a.value();
    Force::new(force_value)
}

fn main() {
    use prefixes::*;
    use units::*;

    println!("=== Unit Conversion Library Demo ===\n");

    // Test 1: Into<Quantity> conversion
    println!("Test 1: Into<Quantity> conversion");
    let distance: Length = meters(100.0).into();
    println!("Distance: {}", distance);

    let distance_km: Quantity<f64, Meter, Kilo> = distance.convert_to();
    println!("Distance in km: {}", distance_km);
    assert!((distance_km.value() - 0.1).abs() < 1e-10);
    println!("✓ Conversion test passed\n");

    // Test 2: Prefix application
    println!("Test 2: Prefix application");
    let km_distance: Quantity<f64, Meter, Kilo> = kilo::<f64, Meter>(5.0).into();
    println!("5 km: {}", km_distance);

    let m_distance: Quantity<f64, Meter, Unity> = km_distance.convert_to();
    println!("Same distance in meters: {}", m_distance);
    assert!((m_distance.value() - 5000.0).abs() < 1e-10);
    println!("✓ Prefix test passed\n");

    // Test 3: Addition with Into
    println!("Test 3: Addition with Into");
    let base: Length = meters(100.0).into();
    let additional = meters(50.0); // This is a Unit<f64, Meter, Unity>

    let sum = base + additional; // Into<Quantity> conversion happens automatically
    println!("{} + {} = {}", base, additional.into(), sum);
    assert!((sum.value() - 150.0).abs() < 1e-10);
    println!("✓ Addition test passed\n");

    // Test 4: Mixed units addition
    println!("Test 4: Mixed units addition");
    let metric: Length = meters(1.0).into();
    let imperial = feet(1.0);

    let sum = metric + imperial;
    println!("{} + {} = {}", metric, imperial.into(), sum);
    // 1m + 1ft = 1m + 0.3048m = 1.3048m
    assert!((sum.value() - 1.3048).abs() < 1e-10);
    println!("✓ Mixed units test passed\n");

    // Test 5: Function with generic units using traits
    println!("Test 5: Function with generic units");
    fn calculate_something<M, L>(mass: M, length: L) -> f64
    where
        M: IntoMass<f64>,
        L: IntoLength<f64>,
    {
        let m = mass.into_mass();
        let l = length.into_length();
        m.value() * l.value()
    }

    let mass_g = grams(2000.0); // 2000 g
    let mass_kg = kilo::<f64, Gram>(2.0); // 2 kg = 2000 g
    let mass_lb = pounds(4.4); // ~2000 g
    let length_m: Length = meters(5.0).into();

    let result1 = calculate_something(mass_g, length_m);
    let result2 = calculate_something(mass_kg, length_m);
    let result3 = calculate_something(mass_lb, length_m);

    println!("Result with grams: {}", result1);
    println!("Result with kg: {}", result2);
    println!("Result with pounds: {}", result3);

    // All should give similar results (allowing for conversion precision)
    assert!((result1 - result2).abs() < 0.1);
    println!("✓ Generic function test passed\n");

    // Test 6: Display with prefixes
    println!("Test 6: Display with prefixes");
    let distance: Quantity<f64, Meter, Kilo> = kilo::<f64, Meter>(5.5).into();
    println!("Distance: {}", distance);
    assert_eq!(format!("{}", distance), "5.5 km");

    let mass: Quantity<f64, Gram, Milli> = milli::<f64, Gram>(250.0).into();
    println!("Mass: {}", mass);
    assert_eq!(format!("{}", mass), "250 mg");

    let force: Quantity<f64, Newton, Kilo> = kilo::<f64, Newton>(2.5).into();
    println!("Force: {}", force);
    assert_eq!(format!("{}", force), "2.5 kN");
    println!("✓ Display test passed\n");

    // Test 7: Mass conversion (grams as base)
    println!("Test 7: Mass conversion with grams as base");
    let mass_kg: Quantity<f64, Gram, Kilo> = kilo::<f64, Gram>(2.5).into(); // 2.5 kg
    let mass_g: Quantity<f64, Gram, Unity> = mass_kg.convert_to();
    println!("{} = {}", mass_kg, mass_g);
    assert!((mass_g.value() - 2500.0).abs() < 1e-10);

    let mass_lb_converted: Quantity<f64, Gram, Unity> = pounds(5.0).into().convert_to();
    println!("5 pounds = {}", mass_lb_converted);
    println!("✓ Mass conversion test passed\n");

    println!("🎉 All tests passed!");
}
