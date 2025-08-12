// mod prefixes;
// mod quantity;
mod quantity_typed;
// mod unit;
// mod units;
// mod value;

// use crate::quantity::QUANTITY_REGISTRY;
// use crate::units::{HORSEPOWER, KILOGRAM, METER, NEWTON, SECOND};
// use crate::value::Value;
use crate::quantity_typed::test;

fn main() {
    // Create three values for the base units
    // let length = Value {
    //     value: 2.0,
    //     unit: METER,
    // };
    // let mass = Value {
    //     value: 3.0,
    //     unit: KILOGRAM,
    // };
    // let time = Value {
    //     value: 4.0,
    //     unit: SECOND,
    // };

    // // Combine them: (mass * length) / (time^2)
    // let mut force = mass * length / (time * time);
    // println!("Force: {:?} | {}", force.value, force.unit.repr());

    // // Convert to horsepower
    // if let Some(hp) = force.convert_to(HORSEPOWER) {
    //     println!("Force in horsepower: {:?} = {} hp", hp, hp.value);
    // } else {
    //     println!("Cannot convert force to horsepower: incompatible dimensions");
    // }

    // let registry = QUANTITY_REGISTRY.lock().unwrap();
    // print!("{:#?}", registry.map);

    test();
}
