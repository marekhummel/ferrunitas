//! Serde serialization and deserialization example
//!
//! This example demonstrates how to serialize and deserialize ferrunitas
//! quantities and measures using serde_json. It shows:
//! - Basic serialization of quantities and measures
//!
//! Run with: cargo run --example serialization --features="serde"
#![cfg(feature = "serde")]

use serde::{Deserialize, Serialize};

use ferrunitas::Measure;
use ferrunitas::system::*;

/// A race car struct
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct RaceStats {
    vehicle_mass: Mass,
    engine_power: Power,
    lap_times: Vec<Measure<Second>>,
}

fn main() {
    println!("=== Ferrunitas Serde Example ===\n");

    let stats = RaceStats {
        vehicle_mass: (950 * Kilogram).into_q(),
        engine_power: (450 * Kilowatt).into_q(),
        lap_times: vec![92.5 * Second, 91.8 * Second, 90.3 * Second],
    };

    // Basic serialization
    let json_str = serde_json::to_string_pretty(&stats).unwrap();
    println!("Serialized RaceStats:\n{}", json_str);

    // Basic deserialization
    let restored_stats: RaceStats = serde_json::from_str(&json_str).unwrap();
    assert_eq!(stats, restored_stats);
    println!("\nDeserialized RaceStats matches original:");
    println!(
        "Mass: {:.4}\nPower: {:.2}\nTimes: [{}]",
        restored_stats.vehicle_mass.as_measure::<Tonne>(),
        restored_stats.engine_power.as_measure::<Horsepower>(),
        restored_stats
            .lap_times
            .iter()
            .map(|t| format!("{:.2}", t))
            .collect::<Vec<_>>()
            .join(", ")
    );
}
