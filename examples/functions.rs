//! Function signature patterns for dimensional types.
//!
//! This example demonstrates different approaches for writing functions that work
//! with Ferrunitas types, showcasing various levels of flexibility and type safety.
//! Generally, it's recommended to use quantities within functions for maximum flexibility.
//!
//! Function patterns shown:
//! - **Specific Units**: Functions requiring exact unit types
//! - **Generic Units**: Functions accepting any unit of a quantity (using `impl Unit`)
//! - **Where Clauses**: More complex generic constraints
//! - **Pure Quantities**: Working directly with dimensionless quantities
//! - **Mixed Approaches**: Combining different parameter styles
//!
//! Each pattern has trade-offs between flexibility, type safety, and API simplicity.
//!
//! Run with: `cargo run --example functions`

use ferrunitas::{Measure, Unit, system::*};

// Requiring specific units in function signatures
fn calculate_kinetic_energy(mass: Measure<Gram>, velocity: Measure<MetrePerSecond>) -> Measure<Joule> {
    (0.5 * mass * velocity * velocity).as_measure()
}

// Working with quantities and where clause
fn calculate_work<F, L>(force: Measure<F>, distance: Measure<L>) -> Energy
where
    F: Unit<Quantity = Force>,
    L: Unit<Quantity = Length>,
{
    force * distance
}

// Working with quantities and impl
fn calculate_power_from_work_and_time(
    work: Measure<impl Unit<Quantity = Energy>>,
    time: Measure<impl Unit<Quantity = Time>>,
) -> Power {
    work / time
}

// Accepting any unit of that quantity but returning a specific unit
fn calculate_force(
    mass: Measure<impl Unit<Quantity = Mass>>,
    acceleration: Measure<impl Unit<Quantity = Acceleration>>,
) -> Measure<Newton> {
    (mass * acceleration).as_measure()
}

// Accepting a combination of measures and quantities
fn calculate_velocity<L>(distance: Measure<L>, time: Time) -> Measure<Knot>
where
    L: Unit<Quantity = Length>,
{
    (distance / time).as_measure()
}

// Working directly with quantities
fn calculate_acceleration(velocity_change: Velocity, time: Time) -> Acceleration {
    velocity_change / time
}

fn main() {
    println!("=== Ferrunitas Library Demo ===\n");
    // Test kinetic energy: KE = ½mv²
    let mass = Gram::new(4.0);
    let velocity = MetrePerSecond::new(6.0);
    let ke = calculate_kinetic_energy(mass, velocity);
    println!("Kinetic Energy of mass {} with velocity {}: {:.4}", mass, velocity, ke);

    // Test work: W = F⋅d
    let force = Newton::new(12.0);
    let distance = Metre::new(3.5);
    let work = calculate_work(force, distance);
    println!(
        "Work done by force {} over distance {}: {:.2}",
        force,
        distance,
        work.as_measure::<Joule>()
    );

    // Test power: P = W/t
    let work = Joule::new(150.0);
    let time = Second::new(10.0);
    let power = calculate_power_from_work_and_time(work, time);
    println!("Power of work {} over time {}: {:.2}", work, time, power.as_measure::<Watt>());

    // Test force: F = ma
    let mass = Gram::new(8.0);
    let acceleration = MetrePerSecondSquared::new(2.5);
    let force = calculate_force(mass, acceleration);
    println!("Force of mass {} with acceleration {}: {:.3}", mass, acceleration, force);

    // Test velocity: v = d/t
    let distance = Metre::new(200.0);
    let time = Second::new(25.0);
    let knots = calculate_velocity(distance, time.into_q());
    println!("Velocity of distance {} over time {}: {:.3}", distance, time, knots);

    // Test acceleration: a = v/t
    let velocity_start = MetrePerSecond::new(0.0);
    let velocity_end = MetrePerSecond::new(30.0);
    let time = Second::new(6.0);
    let acceleration = calculate_acceleration((velocity_end - velocity_start).into_q(), time.into_q());
    println!(
        "Acceleration from {} to {} over time {}: {:.2}",
        velocity_start,
        velocity_end,
        time,
        acceleration.as_measure::<MetrePerSecondSquared>()
    );
}
