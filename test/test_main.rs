// mod prefixes;
// mod quantity;
mod gemini;
// mod unit;
// mod units;
// mod value;

// use crate::quantity::QUANTITY_REGISTRY;
// use crate::units::{ImperialHorsepower, KILOGRAM, METER, NEWTON, SECOND};
// use crate::value::Value;
// use crate::quantity::{EncodedDimensionVector, Quantity};

use crate::gemini::{PrefixedUnit, Quantitiable, Unit, Value};

fn print_unit_details<U: Unit>() {
    println!("--- Details for {} ---", std::any::type_name::<U>());
    println!("Quantity: {}", std::any::type_name::<U::Quantity>());
    println!("Base Unit: {}", std::any::type_name::<U::Base>());
    println!("Conversion Factor to Base: {}", U::FACTOR_TO_QUANTITY_BASE);
    println!();
}

fn mass() {
    prefix!(Kilo, 1000.0, "k");
    prefix!(Mega, 1000000.0, "M");
    prefix!(Milli, 0.001, "m");
    // quantity!(Mass, (Kilogram, Kilo, Gram), "g");
    quantity!(Mass, [L 0, M 1, T 0, I 0, Th 0, N 0, J 0], Gram, "g");
    unit!(Pound, Mass, (453.59237, Gram), "lbs");
    unit!(Stone, Mass, (6350.29, Gram), "st");
    type Megapound = PrefixedUnit<Mega, Pound>;
    type Millistone = PrefixedUnit<Milli, Stone>;
    type Kilogram = PrefixedUnit<Kilo, Gram>;

    // Mass::BaseUnit = Kilogram
    // Kilogram::Base = Gram
    // Gram::Base = Gram
    // Pound::Base = Kilogram
    // Megapound::Base = Pound

    let mst: Value<f64, Millistone> = Value::new(212431230.0);
    let lb: Value<f64, Megapound> = mst.convert();
    println!("{} = {}", mst, lb);
    println!("{}", mst.value / 1000.0 * 6.35029 * 2.20462 / 1000000.0);

    let kg: Value<_, Kilogram> = mst.convert();
    println!("{} = {}", mst, kg);
    println!("{}", mst.value / 1000.0 * 6.35029);

    let g: Value<_, Gram> = mst.convert();
    println!("{} = {}", mst, g);
    println!("{}", mst.value / 1000.0 * 6.35029 * 1000.0);

    println!(
        "Quantity  Base: {}\n",
        std::any::type_name::<<Mass as Quantitiable>::BaseUnit>()
    );
    print_unit_details::<Millistone>();
    print_unit_details::<Stone>();
    print_unit_details::<Kilogram>();
    print_unit_details::<Gram>();
    print_unit_details::<Pound>();
    print_unit_details::<Megapound>();
}

// fn time() {
//     prefix!(Milli, 0.001, "m");
//     quantity!(Time, [L 0, M 0, T 1, I 0, Th 0, N 0, J 0], Second, "s");
//     unit!(Minute, Time, 60.0, "min");
//     unit!(Hour, Time, (60.0, Minute), "h");
//     type Millisecond = PrefixedUnit<Milli, Second>;

//     let h: Value<f64, Hour> = Value::new(2.0);
//     let ms: Value<f64, Millisecond> = h.convert();
//     println!("{} = {}", h, ms);

//     let min: Value<_, Minute> = h.convert();
//     println!("{} = {}", h, min);
// }

// fn temp() {
//     prefix!(Milli, 0.001, "m");
//     prefix!(Centi, 0.01, "c");
//     quantity!(Temperature, 4, Kelvin, "K");
//     unit!(Celsius, Temperature, (factor: 1.0, offset: 273.15, Kelvin), "C");
//     unit!(
//         Fahrenheit,
//         Temperature,
//         (factor: 5.0 / 9.0, offset: -32.0 * 5.0 / 9.0, Celsius),
//         "F"
//     );
//     type Millikelvin = PrefixedUnit<Milli, Kelvin>;
//     type Centifahrenheit = PrefixedUnit<Centi, Fahrenheit>;

//     let f: Value<f64, Fahrenheit> = Value::new(68.0);
//     let c: Value<f64, Celsius> = f.convert();
//     println!("{} = {}", f, c);
//     let k: Value<f64, Kelvin> = f.convert();
//     println!("{} = {}", f, k);
//     let mk: Value<f64, Millikelvin> = f.convert();
//     println!("{} = {}", f, mk);

//     let cf: Value<f64, Centifahrenheit> = Value::new(6800.0);
//     let mk: Value<f64, Millikelvin> = f.convert();
//     println!("{} = {}", cf, mk);
// }

fn area() {
    // quantity!(Length, [L 1, M 0, T 0, I 0, Th 0, N 0, J 0], Meter, "m");
    // quantity!(Area, [(Length, 2)]);
    // unit!(Foot, Length, 0.3048, "ft");
    // type SquareMeter = ExpUnit<Meter, 2>;
    // type SquareFoot = ExpUnit<Foot, 2>;
}
// mod sonnet;
mod sonnet_temp;
fn main() {
    // time();
    // mass();
    // temp();
    // area();

    // unit!(Meter, "m", [L 1, M 0, T 0, I 0, Th 0, N 0, J 0]);
    // unit!(NauticalMile, "nmi", (1852.0, Meter));
    // unit!(Second, "s", [L 0, M 0, T 1, I 0, Th 0, N 0, J 0]);
    // unit!(Hour, "h", (3600.0, Second));
    // unit!(Knots, "kn", [(NauticalMile, 1), (Hour, -1)]);

    sonnet_temp::main();
}
