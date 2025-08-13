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

use crate::gemini::{PrefixedUnit, Quantity, Unit, Value};

fn print_unit_details<U: Unit>() {
    println!("--- Details for {} ---", std::any::type_name::<U>());
    println!("Quantity: {}", std::any::type_name::<U::Quantity>());
    println!("Base Unit: {}", std::any::type_name::<U::Base>());
    println!("Conversion Factor to Base: {}", U::FACTOR_TO_UNIT_BASE);
    println!();
}

fn mass() {
    prefix!(Kilo, 1000.0, "k");
    prefix!(Mega, 1000000.0, "M");
    prefix!(Milli, 0.001, "m");
    quantity!(Mass, (Kilogram, Kilo, Gram), "g");
    unit!(Pound, Mass, 0.45359237, "lbs");
    unit!(Stone, Mass, 6.35029, "st");
    type Megapound = PrefixedUnit<Mega, Pound>;
    type Millistone = PrefixedUnit<Milli, Stone>;

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

    println!(
        "Quantity  Base: {}\n",
        std::any::type_name::<<Mass as Quantity>::BaseUnit>()
    );
    // print_unit_details::<Millistone>();
    // print_unit_details::<Stone>();
    // print_unit_details::<Kilogram>();
    // print_unit_details::<Gram>();
    // print_unit_details::<Pound>();
    // print_unit_details::<Megapound>();
}

fn time() {
    prefix!(Milli, 0.001, "m");
    quantity!(Time, Second, "s");
    unit!(Minute, Time, 60.0, "min");
    unit!(Hour, Time, 3600.0, "h");
    type Millisecond = PrefixedUnit<Milli, Second>;

    let h: Value<f64, Hour> = Value::new(2.0);
    let ms: Value<f64, Millisecond> = h.convert();
    println!("{} = {}", h, ms);

    let min: Value<_, Minute> = h.convert();
    println!("{} = {}", h, min);
}

fn main() {
    time();
}
