use crate::{
    prefixes::SIPrefix,
    quantity::{quantity, Quantity},
    unit::{unit, Unit},
};

// Base quantities
pub const LENGTH: Quantity = Quantity::new([1, 0, 0, 0, 0, 0, 0], Some("length"));
pub const MASS: Quantity = Quantity {
    dimension: [0, 1, 0, 0, 0, 0, 0],
    name: Some("mass"),
};
pub const TIME: Quantity = Quantity {
    dimension: [0, 0, 1, 0, 0, 0, 0],
    name: Some("time"),
};

// Base units
pub const METER: Unit = Unit {
    quantity: LENGTH,
    factor: 1.0,
    prefix: SIPrefix::None,
    name: "meter",
};

pub const KILOGRAM: Unit = Unit {
    quantity: MASS,
    factor: 1.0,
    prefix: SIPrefix::Kilo,
    name: "gram",
};

pub const SECOND: Unit = Unit {
    quantity: TIME,
    factor: 1.0,
    prefix: SIPrefix::None,
    name: "second",
};

// Force = mass * length / time^2
pub const FORCE: Quantity = quantity! {
    name: Some("force"),
    components: [(MASS, 1), (LENGTH, 1), (TIME, -2)]
};

// Newton: base unit for force
pub const NEWTON: Unit = unit! {
    name: "newton",
    components: [(KILOGRAM, 1), (METER, 1), (SECOND, -2)],
    factor: 1.0,
    prefix: SIPrefix::None
};

// Horsepower: 1 hp = 745.7 newton-meter/second
pub const HORSEPOWER: Unit = Unit {
    quantity: FORCE,
    factor: 745.7,
    prefix: SIPrefix::None,
    name: "horsepower",
};
