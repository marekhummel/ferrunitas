//! Example usage of the Ferrunitas library regarding functions.
//! Generally its recommended to stick to quantities within functions, see below

use ferrunitas::{model::Unit, system::*};

// Requiring specific units in function signatures
fn calculate_kinetic_energy(mass: Gram, velocity: MetrePerSecond) -> Joule {
    (0.5 * mass * velocity * velocity).as_unit()
}

// Working with quantities and where clause
fn calculate_work<F, L>(force: F, distance: L) -> Energy
where
    F: Unit<Quantity = Force>,
    L: Unit<Quantity = Length>,
{
    force.into_q() * distance.into_q()
}

// Working with quantities and impl
fn calculate_power_from_work_and_time(
    work: impl Unit<Quantity = Energy>,
    time: impl Unit<Quantity = Time>,
) -> Power {
    work.into_q() / time.into_q()
}

// Accepting any unit of that quantity but returning a specific unit
fn calculate_force(
    mass: impl Unit<Quantity = Mass>,
    acceleration: impl Unit<Quantity = Acceleration>,
) -> Newton {
    (mass.into_q() * acceleration.into_q()).as_unit()
}

// Accepting any unit but computing with units requires more guards, as they can not be
// attached to the generic Unit trait yet.
fn calculate_velocity<L, T>(distance: L, time: T) -> Velocity
where
    L: Unit<Quantity = Length> + std::ops::Div<T, Output = Velocity>,
    T: Unit<Quantity = Time>,
{
    distance / time
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
    println!(
        "Kinetic Energy of mass {} with velocity {}: {:.4}",
        mass, velocity, ke
    );

    // Test work: W = F⋅d
    let force = Newton::new(12.0);
    let distance = Metre::new(3.5);
    let work = calculate_work(force, distance);
    println!(
        "Work done by force {} over distance {}: {:.2}",
        force,
        distance,
        work.as_unit::<Joule>()
    );

    // Test power: P = W/t
    let work = Joule::new(150.0);
    let time = Second::new(10.0);
    let power = calculate_power_from_work_and_time(work, time);
    println!(
        "Power of work {} over time {}: {:.2}",
        work,
        time,
        power.as_unit::<Watt>()
    );

    // Test force: F = ma
    let mass = Gram::new(8.0);
    let acceleration = MetrePerSecondSquared::new(2.5);
    let force = calculate_force(mass, acceleration);
    println!(
        "Force of mass {} with acceleration {}: {:.3}",
        mass, acceleration, force
    );

    // Test velocity: v = d/t
    let distance = Metre::new(200.0);
    let time = Second::new(25.0);
    let velocity = calculate_velocity(distance, time);
    println!(
        "Velocity of distance {} over time {}: {:.3}",
        distance,
        time,
        velocity.as_unit::<MetrePerSecond>()
    );

    // Test acceleration: a = v/t
    let velocity_start = MetrePerSecond::new(0.0);
    let velocity_end = MetrePerSecond::new(30.0);
    let time = Second::new(6.0);
    let acceleration = calculate_acceleration((velocity_end - velocity_start).to_q(), time.to_q());
    println!(
        "Acceleration from {} to {} over time {}: {:.2}",
        velocity_start,
        velocity_end,
        time,
        acceleration.as_unit::<MetrePerSecondSquared>()
    );
}
