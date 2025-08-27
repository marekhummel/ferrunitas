use ferrunitas::format_quantity_dims;
use ferrunitas::model::quantity::QuantityMarker;
use ferrunitas::model::Unit;
use ferrunitas::system::*;

fn units_and_quantities() {
    let v = Kilovolt::new(400);
    let q = v.into_q();

    println!("--- Units and Quantities Example ---");
    println!("This is a unit: {}", v);
    println!("This is a quantity: {}", q);
    println!(
        "Any unit has a quantity: Kilovolt is {}",
        format_quantity_dims!(<Kilovolt as Unit>::Quantity)
    );
    println!(
        "Quantities hold a value, but have only internal meaning: {}",
        q.raw_value()
    );
    println!(
        "Quantities can be converted into any unit of the same quantity: {}",
        q.as_unit::<Megavolt>()
    );
    println!();
}

fn conversion() {
    let q = Volume::new(0.03064); // SHOULD NOT WORK
    let l = Litre::new(30.64);
    let l_to_q = l.into_q();
    let cm3_via_q1 = l_to_q.as_unit::<CubicCentimetre>();
    let cm3_via_q2: CubicCentimetre = l_to_q.as_unit();
    let cm3_direct1 = l.convert::<CubicCentimetre>();
    let cm3_direct2: CubicCentimetre = l.convert();

    println!("--- Conversion Example ---");
    println!("Quantity: {}", q);
    println!("Litres: {}", l);
    println!("Litres to Quantity: {}", l_to_q);
    println!(
        "Litres via Quantity to Cubic Centimetres (method 1): {:.2}",
        cm3_via_q1
    );
    println!(
        "Litres via Quantity to Cubic Centimetres (method 2): {:.2}",
        cm3_via_q2
    );
    println!("Litres to Cubic Centimetres (direct 1): {:.2}", cm3_direct1);
    println!("Litres to Cubic Centimetres (direct 2): {:.2}", cm3_direct2);
    println!("Final Result: {:.2} == {:.2}", l, cm3_direct2);
    println!();
}

fn interop_unit_quantity() {
    let t: Tonne = Tonne::new(2.0); // Mass as unit
    let t_q: Mass = t.into_q(); // Mass as quantity
    let a = MetrePerSecondSquared::new(9.81); // Acceleration as unit
    let a_q = a.into_q(); // Acceleration as quantity
    println!("--- Interop Unit and Quantity Example ---");

    // Add / subtract in either fashion, result type matches LHS !
    let t2 = Tonne::new(8.0);
    let t2_q = t2.into_q();
    let t_sum1: Tonne = t + t2;
    let t_sum2: Mass = t_q + t2;
    let t_sum3: Tonne = t + t2_q;
    let t_sum4: Mass = t_q + t2_q;
    println!("{} == {}   and  {} == {}", t_sum1, t_sum3, t_sum2, t_sum4);
    println!("Also: {} == {}", t_sum2, t_sum2.as_unit::<Tonne>());

    // Multiplication and division (unless scalar) always results in a quantity, but works in any combination of unit and quantity
    let f_q1: Force = t * a;
    let f_q2: Force = t_q * a;
    let f_q3: Force = t * a_q;
    let f_q4: Force = t_q * a_q;
    println!("{} == {} == {} == {}", f_q1, f_q2, f_q3, f_q4);
    println!();
}

fn comparison() {
    let f = Newton::new(10.0);
    let d = Metre::new(2.0);
    let p = Watt::new(20.0);
    let t = Second::new(1.0);

    let e1: FootPoundForce = (f * d).as_unit();
    let e2: Kilojoule = (p * t).as_unit();
    let are_equal = e1.is_equal_to(&e2);

    println!("--- Comparison Example ---");
    println!("Results: {} and {}, Equal?: {}", e1, e2, are_equal);
    println!();
}

fn computation_quantities() {
    // Define inputs
    let newton = Newton::new(10.0);
    let metre = Metre::new(5.0);
    let second = Second::new(2.0);

    // Work with quantities
    let mut force: Force = newton.into_q();
    let mut distance: Length = metre.into_q();
    let mut time: Time = second.into_q();

    // Modify variables a bit
    force += Kilonewton::new(2.0).into_q();
    distance -= Centimetre::new(3.0).into_q();
    distance /= 2.0;
    time += Millisecond::new(50.0).into_q();
    time *= 1.5;

    // Compute work and power.
    let work: Energy = force * distance; // W = F*d
    let power: Power = work / time; // P = W/t

    // Output
    println!("--- Computation Example (Quantities) ---");
    println!("Force: {:.3}", force);
    println!("Distance: {:.3}", distance);
    println!("Time: {:.3}", time);
    println!("Work: {:.3}", work);
    println!(
        "Power: {:.3} (as unit: {:.3})",
        power,
        power.as_unit::<Kilowatt>()
    );
    println!();
}

fn computation_units() {
    // Define base variable
    let mut newton = Newton::new(10.0);
    let mut metre = Metre::new(5.0);
    let mut second = Second::new(2.0);

    // Modify variables a bit (directly as units)
    newton += Kilonewton::new(2.0);
    metre -= Centimetre::new(3.0);
    metre /= 2.0;
    second += Millisecond::new(50.0);
    second *= 1.5;

    // Compute work and power. Note that any multiplication or division of units with other units or
    // quantities will result in a quantity and requires explicit conversion into a unit.
    let work: Energy = newton * metre; // W = F*d
    let power: Power = work / second; // P = W/t

    println!("--- Computation Example (Units) ---");
    println!("Newtons: {:.3}", newton);
    println!("Metres: {:.3}", metre);
    println!("Seconds: {:.3}", second);
    println!("Kilojoules: {:.3}", work.as_unit::<Kilojoule>());
    println!("Kilowatts: {:.6}", power.as_unit::<Kilowatt>());
    println!();
}

fn main() {
    units_and_quantities();

    conversion();

    interop_unit_quantity();

    comparison();

    computation_quantities();

    computation_units();
}
