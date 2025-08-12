mod prefixes;
mod quantity;
mod unit;
mod units;
mod value;

use crate::units::{HORSEPOWER, KILOGRAM, METER, NEWTON, SECOND};
use crate::value::Value;

fn main() {
    // Create three values for the base units
    let length = Value {
        value: 2.0,
        unit: METER,
    };
    let mass = Value {
        value: 3.0,
        unit: KILOGRAM,
    };
    let time = Value {
        value: 4.0,
        unit: SECOND,
    };

    // Combine them: (mass * length) / (time^2)
    let force = mass * length / (time * time);
    println!("Force: {:?} = {} N", force, force.value);

    // Convert to horsepower
    if let Some(hp) = force.convert_to(HORSEPOWER) {
        println!("Force in horsepower: {:?} = {} hp", hp, hp.value);
    } else {
        println!("Cannot convert force to horsepower: incompatible dimensions");
    }
}
