# Ferrunitas - Type-Safe Unit Conversion Library

A Rust library for compile-time dimensional analysis and unit conversions. Ferrunitas provides type safety for physical quantities and prevents dimensional errors at compile time through Rust's powerful type system.

Current build and test state: ![Build + Test](https://github.com/marekhummel/ferrunitas/actions/workflows/main.yml/badge.svg)

## Features

- **Compile-time dimensional analysis**: Prevents adding incompatible quantities (e.g. mass + length) at compile time
- **Type-safe unit conversions**: Zero runtime overhead conversions between compatible units
- **Flexible API**: Work with specific units (`Measure<Metre>`) or generic quantities (`Length`)
- **Extensive unit system**: Complete SI base units, derived units, and common non-SI units
- **Prefix support**: Metric prefixes (kilo, milli, etc.) with easy unit definition
- **Custom unit definition**: Macros for defining your own units, quantities, and prefixes
- **Quantity tagging** (optional): Distinguish between dimensionally equivalent but semantically different quantities (e.g., `Angle` vs `Information`)
- **Zero-cost abstractions**: All dimensional checking happens at compile time

## Usage

### Basics

```rust
use ferrunitas::system::*;
use ferrunitas::{Measure, Unit};

fn main() {
    // Create measures with specific units
    let distance = Metre::new(100.0);
    let time = Second::new(10.0);

    // Convert between compatible units
    let feet: Measure<Foot> = distance.convert();
    let minutes: Measure<Minute> = time.convert();
    println!("Distance: {:.2} or {:.2}", distance, feet);
    println!("Time: {:.2} or {:.2}", time, minutes);

    // Arithmetic operations with automatic dimensional checking
    let velocity: Velocity = distance / time;
    let speed_mps = velocity.as_measure::<MetrePerSecond>();
    let speed_mph = velocity.as_measure::<MilePerHour>();

    println!("Speed: {:.3} or {:.3}", speed_mps, speed_mph);
}
```

### Relevant imports
```rust
// For dimensional formatting
use ferrunitas::common::{format_dims, format_unit_dims};

// For access to all definitions to units, quantities and prefixes
use ferrunitas::system::*;

// Crate root access to most common struct Measure and trait Unit to make methods available
use ferrunitas::{Measure, Unit};

// Macros for own definitions (typenum consts for quantity macro)
use ferrunitas::typenum_consts::*;
use ferrunitas::{prefix, quantity, unit};
```

### Working with Quantities vs Measures

Ferrunitas offers two main ways to work with physical values (they are compatitble with each other regarding arithmetics):

**Measures** - Values with specific units:
```rust
let mass = Kilogram::new(5.0);        // Measure<Kilogram>
let length = Foot::new(10.0);         // Measure<Foot>
let sum = mass + Gram::new(500.0);    // Addition works across compatible units
```

**Quantities** - Dimensioned values without specific units:
```rust
let mass: Mass = Kilogram::new(5.0).into_q();     // Convert to quantity
let length: Length = Foot::new(10.0).into_q();    // Convert to quantity
let force: Force = mass * acceleration;           // Multiplication creates new quantity
```

### Unit Conversions

```rust
// Multiple ways to convert units
let liters = Litre::new(30.64);

// Via quantity (intermediate step)
let volume: Volume = liters.into_q();
let cubic_cm: Measure<CubicCentimetre> = volume.as_measure();

// Direct conversion
let cubic_cm_direct: Measure<CubicCentimetre> = liters.convert();

// One-line conversion
let cubic_cm_oneline = Volume::convert::<Litre, CubicCentimetre>(30.64);
```

### Physics Calculations

```rust
// All operations are dimensionally checked at compile time
let mass = Kilogram::new(2.0);
let acceleration = MetrePerSecondSquared::new(9.81);
let force: Force = mass * acceleration;  // F = ma

let distance = Metre::new(5.0);
let work: Energy = force * distance;     // W = F·d

let time = Second::new(2.0);
let power: Power = work / time;          // P = W/t

println!("Power: {:.2}", power.as_measure::<Watt>());
```

### Custom Units and Quantities

Define your own units using the provided macros:

```rust
use ferrunitas::{quantity, unit, prefix, typenum_consts::*};

// Define a new quantity (7 SI base dimensions: M, L, T, I, Th, N, J)
quantity!(MyLength: M Z0, L P1, T Z0, I Z0, Th Z0, N Z0, J Z0);

// Define a new quantity with tagging (requires `quantity_tags` feature)
quantity!(SpecialAngle: M Z0, L Z0, T Z0, I Z0, Th Z0, N Z0, J Z0; marked);

// Define a new prefix
prefix!(Magic, 42, "M");

// Define new units
unit!(base: Elbow, "elbow", MyLength; prefixable);
unit!(derived: Yard, "yd", (0.9144, Metre));
unit!(prefix: Magicmetre, Magic, Metre);

// Compound units with quantity tagging (requires `quantity_tags` feature)
unit!(compound: MyAngleUnit, "mau", [(Radian, P1)]; marked SpecialAngle);
```

### Quantity Tagging (Optional Feature)

When the `quantity_tags` feature is enabled, Ferrunitas can distinguish between quantities that are dimensionally identical but represent different physical concepts:

```rust
// Enable with: cargo build --features="quantity_tags"
use ferrunitas::system::*;
use ferrunitas::Unit;

// These are both dimensionless, but represent different concepts
let data = Bit::new(8.0);
let angle = Radian::new(1.0);

// Without quantity_tags: these can be added directly
// With quantity_tags: compilation error - different quantity types!
// let invalid = data + angle;  // Error with quantity_tags enabled

// Instead, you need to explicitly specify the target quantity:
let combined = data.into_q().specify::<Dimensionless>() +
               angle.into_q().specify::<Dimensionless>();

// This helps prevent mixing semantically different values
let area1 = SquareMetre::new(4.0);
let area2 = SquareMetre::new(1.0);
let solid_angle = Steradian::new(0.5);

// Without tags: this works (both are dimensionless ratios)
// With tags: need to specify the result type
let result = (area1 / area2).specify::<SolidAngle>() + solid_angle;
```

### Function Signatures

Ferrunitas provides flexible function signatures for different use cases:

```rust
// Accept specific units
fn kinetic_energy(mass: Measure<Gram>, velocity: Measure<MetrePerSecond>) -> Measure<Joule> {
    (0.5 * mass * velocity * velocity).as_measure()
}

// Accept any unit of a quantity type
fn calculate_force(
    mass: Measure<impl Unit<Quantity = Mass>>,
    acceleration: Measure<impl Unit<Quantity = Acceleration>>,
) -> Measure<Newton> {
    (mass.into_q() * acceleration.into_q()).as_measure()
}

// Work directly with quantities
fn calculate_acceleration(velocity_change: Velocity, time: Time) -> Acceleration {
    velocity_change / time
}
```

## Examples

The `examples/` directory contains comprehensive usage examples:

- **`basics.rs`** - Core concepts: measures, quantities, conversions, and arithmetic
- **`functions.rs`** - Different patterns for writing functions with dimensional types
- **`advanced.rs`** - Custom unit definitions using macros
- **`misc.rs`** - Dimensional introspection, debugging utilities, and quantity tagging demonstration

Run examples with:
```bash
cargo run --example basics
cargo run --example functions
cargo run --example advanced
cargo run --example misc

# To see quantity tagging in action:
cargo run --example misc --features="quantity_tags"
```



## Limitations / Notes
* Internal storage uses `f64`; typical floating point caveats apply, see examples/misc.rs.
* Rounding / formatting of display values is a caller concern, however usual format specifiers are respected.
* Offsets are supported for temperature scales, but are quite limited in usage besides simple conversion.

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue for any suggestions or improvements.

## License

This project is licensed under the Apache-2.0 License.
