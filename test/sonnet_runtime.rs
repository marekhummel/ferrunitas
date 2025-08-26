use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

// ===========================
// Runtime Dimensional System
// ===========================

// Dimensional vector stored at runtime: [M, L, T, I, Θ, N, J]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dimensions {
    pub mass: i32,        // M - Mass (kg)
    pub length: i32,      // L - Length (m)
    pub time: i32,        // T - Time (s)
    pub current: i32,     // I - Electric current (A)
    pub temperature: i32, // Θ - Temperature (K)
    pub amount: i32,      // N - Amount of substance (mol)
    pub luminosity: i32,  // J - Luminous intensity (cd)
}

impl Dimensions {
    pub const fn new(m: i32, l: i32, t: i32, i: i32, theta: i32, n: i32, j: i32) -> Self {
        Self {
            mass: m,
            length: l,
            time: t,
            current: i,
            temperature: theta,
            amount: n,
            luminosity: j,
        }
    }

    // Standard SI base dimensions
    pub const MASS: Dimensions = Dimensions::new(1, 0, 0, 0, 0, 0, 0);
    pub const LENGTH: Dimensions = Dimensions::new(0, 1, 0, 0, 0, 0, 0);
    pub const TIME: Dimensions = Dimensions::new(0, 0, 1, 0, 0, 0, 0);

    // Common derived dimensions
    pub const VELOCITY: Dimensions = Dimensions::new(0, 1, -1, 0, 0, 0, 0); // L/T
    pub const ACCELERATION: Dimensions = Dimensions::new(0, 1, -2, 0, 0, 0, 0); // L/T²
    pub const FORCE: Dimensions = Dimensions::new(1, 1, -2, 0, 0, 0, 0); // M⋅L/T²
    pub const ENERGY: Dimensions = Dimensions::new(1, 2, -2, 0, 0, 0, 0); // M⋅L²/T²
    pub const POWER: Dimensions = Dimensions::new(1, 2, -3, 0, 0, 0, 0); // M⋅L²/T³
}

impl Add for Dimensions {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            mass: self.mass + other.mass,
            length: self.length + other.length,
            time: self.time + other.time,
            current: self.current + other.current,
            temperature: self.temperature + other.temperature,
            amount: self.amount + other.amount,
            luminosity: self.luminosity + other.luminosity,
        }
    }
}

impl Sub for Dimensions {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            mass: self.mass - other.mass,
            length: self.length - other.length,
            time: self.time - other.time,
            current: self.current - other.current,
            temperature: self.temperature - other.temperature,
            amount: self.amount - other.amount,
            luminosity: self.luminosity - other.luminosity,
        }
    }
}

impl fmt::Display for Dimensions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut parts = Vec::new();

        if self.mass != 0 {
            parts.push(format!("M^{}", self.mass));
        }
        if self.length != 0 {
            parts.push(format!("L^{}", self.length));
        }
        if self.time != 0 {
            parts.push(format!("T^{}", self.time));
        }
        if self.current != 0 {
            parts.push(format!("I^{}", self.current));
        }
        if self.temperature != 0 {
            parts.push(format!("Θ^{}", self.temperature));
        }
        if self.amount != 0 {
            parts.push(format!("N^{}", self.amount));
        }
        if self.luminosity != 0 {
            parts.push(format!("J^{}", self.luminosity));
        }

        if parts.is_empty() {
            write!(f, "dimensionless")
        } else {
            write!(f, "[{}]", parts.join("⋅"))
        }
    }
}

// ===========================
// Quantity - Holds value and dimensions at runtime
// ===========================

#[derive(Debug, Clone, Copy)]
pub struct Quantity {
    value: f64,
    dimensions: Dimensions,
}

impl Quantity {
    pub fn new(value: f64, dimensions: Dimensions) -> Self {
        Self { value, dimensions }
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn dimensions(&self) -> Dimensions {
        self.dimensions
    }

    // Check if this quantity has the expected dimensions
    pub fn expect_dimensions(&self, expected: Dimensions) -> Result<(), String> {
        if self.dimensions == expected {
            Ok(())
        } else {
            Err(format!(
                "Expected dimensions {}, but got {}",
                expected, self.dimensions
            ))
        }
    }

    // Convert to a specific unit if dimensions match
    pub fn convert_to_unit(
        &self,
        target_dimensions: Dimensions,
        factor: f64,
    ) -> Result<f64, String> {
        if self.dimensions == target_dimensions {
            Ok(self.value / factor)
        } else {
            Err(format!(
                "Dimension mismatch: expected {}, got {}",
                target_dimensions, self.dimensions
            ))
        }
    }

    // Convenience constructors
    pub fn mass(value: f64) -> Self {
        Self::new(value, Dimensions::MASS)
    }

    pub fn length(value: f64) -> Self {
        Self::new(value, Dimensions::LENGTH)
    }

    pub fn time(value: f64) -> Self {
        Self::new(value, Dimensions::TIME)
    }

    pub fn force(value: f64) -> Self {
        Self::new(value, Dimensions::FORCE)
    }

    pub fn energy(value: f64) -> Self {
        Self::new(value, Dimensions::ENERGY)
    }

    pub fn power(value: f64) -> Self {
        Self::new(value, Dimensions::POWER)
    }
}

impl fmt::Display for Quantity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.value, self.dimensions)
    }
}

// ===========================
// Generic Arithmetic - Works for ANY combination!
// ===========================

impl Mul for Quantity {
    type Output = Quantity;

    fn mul(self, rhs: Quantity) -> Quantity {
        Quantity::new(
            self.value * rhs.value,
            self.dimensions + rhs.dimensions, // Add dimensions for multiplication
        )
    }
}

impl Div for Quantity {
    type Output = Quantity;

    fn div(self, rhs: Quantity) -> Quantity {
        Quantity::new(
            self.value / rhs.value,
            self.dimensions - rhs.dimensions, // Subtract dimensions for division
        )
    }
}

impl Add for Quantity {
    type Output = Result<Quantity, String>;

    fn add(self, rhs: Quantity) -> Result<Quantity, String> {
        if self.dimensions == rhs.dimensions {
            Ok(Quantity::new(self.value + rhs.value, self.dimensions))
        } else {
            Err(format!(
                "Cannot add incompatible dimensions: {} + {}",
                self.dimensions, rhs.dimensions
            ))
        }
    }
}

impl Sub for Quantity {
    type Output = Result<Quantity, String>;

    fn sub(self, rhs: Quantity) -> Result<Quantity, String> {
        if self.dimensions == rhs.dimensions {
            Ok(Quantity::new(self.value - rhs.value, self.dimensions))
        } else {
            Err(format!(
                "Cannot subtract incompatible dimensions: {} - {}",
                self.dimensions, rhs.dimensions
            ))
        }
    }
}

// ===========================
// Macro for SI Prefixes
// ===========================

macro_rules! define_prefix {
    ($name:ident, $factor:expr) => {
        #[derive(Debug, Clone, Copy)]
        pub struct $name;

        impl $name {
            pub const FACTOR: f64 = $factor;
        }
    };
}

// ===========================
// Unit Functions - Return Quantity directly
// ===========================

// Base units (using gram as mass base to avoid SI anomaly)
pub fn gram(value: f64) -> Quantity {
    Quantity::new(value, Dimensions::MASS)
}

pub fn meter(value: f64) -> Quantity {
    Quantity::new(value, Dimensions::LENGTH)
}

pub fn second(value: f64) -> Quantity {
    Quantity::new(value, Dimensions::TIME)
}

// Prefixed base units
pub fn kilogram(value: f64) -> Quantity {
    Quantity::new(value * 1000.0, Dimensions::MASS) // Convert to grams
}

pub fn centimeter(value: f64) -> Quantity {
    Quantity::new(value * 0.01, Dimensions::LENGTH) // Convert to meters
}

pub fn millimeter(value: f64) -> Quantity {
    Quantity::new(value * 0.001, Dimensions::LENGTH) // Convert to meters
}

pub fn kilometer(value: f64) -> Quantity {
    Quantity::new(value * 1000.0, Dimensions::LENGTH) // Convert to meters
}

// Derived units (factors account for base unit differences)
pub fn newton(value: f64) -> Quantity {
    Quantity::new(value * 1000.0, Dimensions::FORCE) // 1 N = 1000 g⋅m/s²
}

pub fn joule(value: f64) -> Quantity {
    Quantity::new(value * 1000.0, Dimensions::ENERGY) // 1 J = 1000 g⋅m²/s²
}

pub fn watt(value: f64) -> Quantity {
    Quantity::new(value * 1000.0, Dimensions::POWER) // 1 W = 1000 g⋅m²/s³
}

// Unit conversion functions
pub fn to_grams(q: Quantity) -> Result<f64, String> {
    q.expect_dimensions(Dimensions::MASS)?;
    Ok(q.value)
}

pub fn to_kilograms(q: Quantity) -> Result<f64, String> {
    q.expect_dimensions(Dimensions::MASS)?;
    Ok(q.value / 1000.0)
}

pub fn to_meters(q: Quantity) -> Result<f64, String> {
    q.expect_dimensions(Dimensions::LENGTH)?;
    Ok(q.value)
}

pub fn to_kilometers(q: Quantity) -> Result<f64, String> {
    q.expect_dimensions(Dimensions::LENGTH)?;
    Ok(q.value / 1000.0)
}

pub fn to_centimeters(q: Quantity) -> Result<f64, String> {
    q.expect_dimensions(Dimensions::LENGTH)?;
    Ok(q.value / 0.01)
}

pub fn to_seconds(q: Quantity) -> Result<f64, String> {
    q.expect_dimensions(Dimensions::TIME)?;
    Ok(q.value)
}

pub fn to_newtons(q: Quantity) -> Result<f64, String> {
    q.expect_dimensions(Dimensions::FORCE)?;
    Ok(q.value / 1000.0)
}

pub fn to_joules(q: Quantity) -> Result<f64, String> {
    q.expect_dimensions(Dimensions::ENERGY)?;
    Ok(q.value / 1000.0)
}

pub fn to_watts(q: Quantity) -> Result<f64, String> {
    q.expect_dimensions(Dimensions::POWER)?;
    Ok(q.value / 1000.0)
}

// ===========================
// Physics Functions with Dimensional Validation
// ===========================

// Function that expects specific dimensions
pub fn calculate_kinetic_energy(mass: Quantity, velocity: Quantity) -> Result<Quantity, String> {
    mass.expect_dimensions(Dimensions::MASS)?;
    velocity.expect_dimensions(Dimensions::VELOCITY)?;

    Ok(mass * velocity * velocity * 0.5) // KE = ½mv²
}

// Function that works with any compatible force and distance
pub fn calculate_work(force: Quantity, distance: Quantity) -> Result<Quantity, String> {
    force.expect_dimensions(Dimensions::FORCE)?;
    distance.expect_dimensions(Dimensions::LENGTH)?;

    Ok(force * distance) // W = F⋅d
}

// Generic function that accepts anything that can be converted to expected units
pub fn calculate_gravitational_force(
    mass1: Quantity,
    mass2: Quantity,
    distance: Quantity,
) -> Result<Quantity, String> {
    mass1.expect_dimensions(Dimensions::MASS)?;
    mass2.expect_dimensions(Dimensions::MASS)?;
    distance.expect_dimensions(Dimensions::LENGTH)?;

    const G: f64 = 6.67430e-11; // m³⋅kg⁻¹⋅s⁻² in SI units
                                // Convert to our base units: G = 6.67430e-11 * 1000 = 6.67430e-8 m³⋅g⁻¹⋅s⁻²
    const G_BASE: f64 = 6.67430e-8;

    Ok(Quantity::force(
        G_BASE * mass1.value * mass2.value / (distance.value * distance.value),
    ))
}

// Function that demonstrates mixed unit input
pub fn calculate_power_from_work_and_time(
    work: Quantity,
    time: Quantity,
) -> Result<Quantity, String> {
    work.expect_dimensions(Dimensions::ENERGY)?;
    time.expect_dimensions(Dimensions::TIME)?;

    Ok(work / time)
}

// ===========================
// Main Function - Usage Examples
// ===========================

fn main() -> Result<(), String> {
    println!("=== Runtime Dimensional Analysis Test ===\n");

    // Test 1: Function-based unit creation
    println!("Test 1: Function-based units");
    let mass = kilogram(5.0);
    let distance = meter(100.0);
    let time = second(5.0);

    println!("Mass: {} = {} kg", mass, to_kilograms(mass)?);
    println!("Distance: {} = {} m", distance, to_meters(distance)?);
    println!("Time: {} = {} s", time, to_seconds(time)?);

    // Test 2: Generic arithmetic
    println!("\nTest 2: Physics calculations");
    let velocity = distance / time;
    let acceleration = velocity / time;
    let force = mass * acceleration;
    let work = force * distance;
    let power = work / time;

    println!("Velocity: {} = {} m/s", velocity, velocity.value());
    println!(
        "Acceleration: {} = {} m/s²",
        acceleration,
        acceleration.value()
    );
    println!("Force: {} = {} N", force, to_newtons(force)?);
    println!("Work: {} = {} J", work, to_joules(work)?);
    println!("Power: {} = {} W", power, to_watts(power)?);

    // Test 3: Physics functions with validation
    println!("\nTest 3: Physics functions");

    let ke = calculate_kinetic_energy(mass, velocity)?;
    println!("Kinetic Energy: {} = {} J", ke, to_joules(ke)?);

    let work2 = calculate_work(force, distance)?;
    println!("Work (from function): {} = {} J", work2, to_joules(work2)?);

    let power2 = calculate_power_from_work_and_time(work, time)?;
    println!(
        "Power (from function): {} = {} W",
        power2,
        to_watts(power2)?
    );

    // Test 4: Mixed units
    println!("\nTest 4: Mixed units work seamlessly");
    let mass_small = gram(500.0); // 0.5 kg
    let distance_big = kilometer(2.0); // 2000 m
    let time_short = second(0.5); // 0.5 s

    let big_velocity = distance_big / time_short; // 4000 m/s
    let big_accel = big_velocity / time_short; // 8000 m/s²
    let small_force = mass_small * big_accel; // 4000000 g⋅m/s²

    println!("Small mass: {} = {} g", mass_small, to_grams(mass_small)?);
    println!(
        "Big distance: {} = {} km",
        distance_big,
        to_kilometers(distance_big)?
    );
    println!(
        "Result force: {} = {} N",
        small_force,
        to_newtons(small_force)?
    );

    // Test 5: Dimensional safety in functions
    println!("\nTest 5: Dimensional safety");

    // This would fail:
    match calculate_kinetic_energy(distance, mass) {
        Ok(_) => println!("This shouldn't happen"),
        Err(e) => println!("Expected error: {}", e),
    }

    // Test 6: Gravitational force calculation
    println!("\nTest 6: Gravitational force");
    let earth_mass = kilogram(5.972e24); // Earth's mass
    let object_mass = kilogram(100.0); // 100 kg object
    let earth_radius = meter(6.371e6); // Earth's radius

    let grav_force = calculate_gravitational_force(earth_mass, object_mass, earth_radius)?;
    println!(
        "Gravitational force: {} = {} N",
        grav_force,
        to_newtons(grav_force)?
    );

    // Test 7: Addition/subtraction safety
    println!("\nTest 7: Addition/subtraction with compatible units");
    let length1 = meter(10.0);
    let length2 = centimeter(500.0); // 5 m

    match length1 + length2 {
        Ok(total) => println!("Total length: {} = {} m", total, to_meters(total)?),
        Err(e) => println!("Error: {}", e),
    }

    // This would fail:
    match mass + distance {
        Ok(_) => println!("This shouldn't happen"),
        Err(e) => println!("Expected error: {}", e),
    }

    println!("\n=== All tests completed successfully! ===");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_signatures() -> Result<(), String> {
        let m = kilogram(2.0);
        let v = meter(10.0) / second(2.0); // 5 m/s

        let ke = calculate_kinetic_energy(m, v)?;
        assert_eq!(ke.dimensions(), Dimensions::ENERGY);
        assert_eq!(to_joules(ke)?, 25.0); // ½ × 2kg × (5m/s)² = 25J
        Ok(())
    }

    #[test]
    fn test_dimensional_validation() {
        let mass = kilogram(1.0);
        let length = meter(1.0);

        // This should fail - wrong dimensions
        let result = calculate_kinetic_energy(length, mass);
        assert!(result.is_err());
    }

    #[test]
    fn test_unit_conversions() -> Result<(), String> {
        let length_m = meter(1000.0);
        let length_km = kilometer(1.0);

        assert_eq!(to_meters(length_m)?, to_meters(length_km)?);
        Ok(())
    }

    #[test]
    fn test_arithmetic() -> Result<(), String> {
        let mass = kilogram(2.0);
        let distance = meter(10.0);
        let time = second(2.0);

        let velocity = distance / time;
        let acceleration = velocity / time;
        let force = mass * acceleration;

        assert_eq!(force.dimensions(), Dimensions::FORCE);
        assert_eq!(to_newtons(force)?, 5.0); // 2kg × 2.5m/s² = 5N
        Ok(())
    }
}
