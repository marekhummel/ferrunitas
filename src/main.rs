mod model;
mod system;

use std::ops::{Add, Div, Mul, Sub};
use system::defs::*;

// ============================================================================
// Physics Functions with Compile-Time Type Safety
// ============================================================================

pub fn calculate_kinetic_energy(mass: Mass, velocity: Velocity) -> Energy {
    mass * velocity * velocity * 0.5 // KE = ½mv² - type checked at compile time!
}

pub fn calculate_work(force: Force, distance: Length) -> Energy {
    force * distance // W = F⋅d - type checked at compile time!
}

pub fn calculate_power_from_work_and_time(work: Energy, time: Time) -> Power {
    work / time // P = W/t - type checked at compile time!
}

// Generic function that works with any compatible quantities
pub fn multiply_quantities<Q1, Q2>(a: Q1, b: Q2) -> Q1::Output
where
    Q1: Mul<Q2>,
{
    a * b // Works for ANY dimensional combination that's mathematically valid
}

// ============================================================================
// Main Function - Usage Examples
// ============================================================================

pub fn main() {
    println!("=== Type-Level Dimensional Analysis Test ===\n");

    // Test 1: Basic quantities with compile-time types
    println!("Test 1: Type-safe unit creation");
    let mass = kilogram(5.0); // Type: Mass
    let distance = meter(100.0); // Type: Length
    let time = second(5.0); // Type: Time

    println!("Mass: {} kg", to_kilograms(mass));
    println!("Distance: {} m", to_meters(distance));
    println!("Time: {} s", to_seconds(time));

    // Test 2: Compile-time dimensional arithmetic
    println!("\nTest 2: Compile-time physics calculations");
    let velocity = distance / time; // Type: Velocity (automatic!)
    let acceleration = velocity / time; // Type: Acceleration (automatic!)
    let force = mass * acceleration; // Type: Force (automatic!)
    let work = force * distance; // Type: Energy (automatic!)
    let power = work / time; // Type: Power (automatic!)

    println!("Velocity: {} m/s", velocity.value());
    println!("Acceleration: {} m/s²", acceleration.value());
    println!("Force: {} N", to_newtons(force));
    println!("Work: {} J", to_joules(work));
    println!("Power: {} W", to_watts(power));

    // Test 3: Physics functions with compile-time safety
    println!("\nTest 3: Type-safe physics functions");
    let ke = calculate_kinetic_energy(mass, velocity);
    let work2 = calculate_work(force, distance);
    let power2 = calculate_power_from_work_and_time(work, time);

    println!("Kinetic Energy: {} J", to_joules(ke));
    println!("Work (from function): {} J", to_joules(work2));
    println!("Power (from function): {} W", to_watts(power2));

    // Test 4: Generic multiplication
    println!("\nTest 4: Generic operations");
    let area = multiply_quantities(distance, distance); // Length × Length = Area
    let volume = multiply_quantities(area, distance); // Area × Length = Volume

    println!("Area: {} m²", area.value());
    println!("Volume: {} m³", volume.value());

    // Test 5: Compatible addition/subtraction
    println!("\nTest 5: Type-safe addition");
    let distance1 = meter(10.0);
    let distance2 = centimeter(500.0); // 5 m
    let total_distance = distance1 + distance2;

    println!("Total distance: {} m", to_meters(total_distance));

    // Test 6: Mixed units work seamlessly
    println!("\nTest 6: Mixed units");
    let small_mass = gram(250.0); // 0.25 kg
    let big_distance = kilometer(3.0); // 3000 m
    let short_time = second(0.1); // 0.1 s

    let extreme_velocity = big_distance / short_time; // 30000 m/s
    let extreme_acceleration = extreme_velocity / short_time; // 300000 m/s²
    let extreme_force = small_mass * extreme_acceleration; // Huge force

    println!("Extreme force: {} N", to_newtons(extreme_force));

    // Note: These would fail to compile:
    // let invalid = calculate_kinetic_energy(distance, mass); // Compile error!
    // let invalid2 = mass + distance; // Compile error!
    // let invalid3 = force + energy;  // Compile error!

    println!("\n=== All tests completed successfully! ===");
    println!("Note: All dimensional checking happened at compile time - zero runtime overhead!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_time_physics() {
        let m = kilogram(2.0);
        let d = meter(10.0);
        let t = second(2.0);

        let v = d / t; // Velocity
        let a = v / t; // Acceleration
        let f = m * a; // Force
        let w = f * d; // Energy
        let p = w / t; // Power

        assert_eq!(to_newtons(f), 5.0); // 2kg × 2.5m/s² = 5N
        assert_eq!(to_joules(w), 50.0); // 5N × 10m = 50J
        assert_eq!(to_watts(p), 25.0); // 50J / 2s = 25W
    }

    #[test]
    fn test_kinetic_energy() {
        let mass = kilogram(2.0);
        let velocity = meter(10.0) / second(2.0); // 5 m/s
        let ke = calculate_kinetic_energy(mass, velocity);

        assert_eq!(to_joules(ke), 25.0); // ½ × 2kg × (5m/s)² = 25J
    }

    #[test]
    fn test_unit_conversions() {
        let length_m = meter(1000.0);
        let length_km = kilometer(1.0);

        assert_eq!(to_meters(length_m), to_meters(length_km));
    }

    #[test]
    fn test_generic_multiplication() {
        let length1 = meter(5.0);
        let length2 = meter(3.0);
        let area = multiply_quantities(length1, length2);

        assert_eq!(area.value(), 15.0); // 5m × 3m = 15m²
    }
}
