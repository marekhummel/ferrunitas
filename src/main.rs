use crate::gemini::Quantity;

// mod prefixes;
// mod quantity;
mod gemini;
// mod unit;
// mod units;
// mod value;

// use crate::quantity::QUANTITY_REGISTRY;
// use crate::units::{HORSEPOWER, KILOGRAM, METER, NEWTON, SECOND};
// use crate::value::Value;
// use crate::quantity::{EncodedDimensionVector, Quantity};

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

    /*
    let dist: Length = 0.5 * Prefix::Kilo * Unit::Meter;

    let time: Time = 10 * Unit::Minute;

    let speed: Velocity = dist / time;

    print!("{}", speed); // 0.05 km/min

    print!("{}", speed.as(Unit::Meter / Unit::Second)); // 0.8333 m/s
     */

    // ================ examples / usage ===================

    // Define your SI basis order once: [L, M, T, I, Θ, N, J]
    //     type Length = quantity!(Length, [L 1, M 0, T 0, I 0, Th 0, N 0, J 0]);
    //     type Mass = quantity!(Mass, [L 0, M 1, T 0, I 0, Th 0, N 0, J 0]);
    //     type Time = quantity!(Time, [L 0, M 0, T 1, I 0, Th 0, N 0, J 0]);

    //     type Velocity = quantity!(Velocity, [(Length, 1), (Time, -1)]);
    //     type Accel = quantity!(Acceleration, [(Velocity, 1), (Time, -1)]);

    //     // compile-time equality guard in function signatures
    //     pub fn combine<const A: EncodedDimensionVector>(
    //         x: Quantity<A>,
    //         _y: Quantity<A>,
    //     ) -> Quantity<A> {
    //         // ...
    //         x
    //     }

    //     let x: Accel = Quantity {};
    //     type Accel2 = quantity!(Accel2, [(Length, 1), (Time, -2)]);
    //     let y: Accel2 = Quantity {};
    //     let _z = combine(x, y);

    use crate::gemini::{PrefixedUnit, Unit, Value};

    prefix!(Kilo, 1000.0, "k");
    prefix!(Mega, 10000000.0, "M");
    quantity!(Mass, (Kilogram, Kilo, Gram), "g");
    unit!(Pound, Mass, 0.45359237, "lbs");
    type Megapound = PrefixedUnit<Mega, Pound>;

    // Mass::BaseUnit = Kilogram
    // Kilogram::Base = Gram
    // Gram::Base = Gram
    // Pound::Base = Kilogram
    // Megapound::Base = Pound

    let kg: Value<f64, Kilogram> = Value::new(2.1240);
    let lb: Value<f64, Pound> = kg.convert();
    println!("{} = {}", kg, lb);

    fn print_unit_details<U: Unit>() {
        println!("--- Details for {} ---", std::any::type_name::<U>());
        println!("Quantity: {}", std::any::type_name::<U::Quantity>());
        println!("Base Unit: {}", std::any::type_name::<U::Base>());
        println!(
            "Conversion Factor to Base: {}",
            U::CONVERSION_FACTOR_TO_BASE
        );
        println!();
    }

    println!(
        "Quantity  Base: {}\n",
        std::any::type_name::<<Mass as Quantity>::BaseUnit>()
    );

    print_unit_details::<Kilogram>();
    print_unit_details::<Gram>();
    print_unit_details::<Pound>();
    print_unit_details::<Megapound>();
}
