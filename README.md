# Ferrunitas - Type-Safe Unit Conversion Library

A Rust library for compile-time dimensional analysis and unit conversions. Ferrunitas provides type safety for physical quantities and prevents dimensional errors at compile time.

## Features

- **Compile-time dimensional analysis**: Prevents adding masses to lengths or other dimensional errors
- **Type-safe unit conversions**: No runtime errors for invalid conversions
- **Physics calculations**: Built-in functions for common physics formulas (KE = ½mv², F = ma, etc.)
- **Extensive prefix system**: Support for metric prefixes (kilo, centi, milli, etc.)
- **Zero-cost abstractions**: All dimensional checking happens at compile time
- **Temperature conversions**: Celsius ↔ Fahrenheit conversions

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
ferrunitas = "0.1.0"
```

### Basic Usage

```rust
use ferrunitas::*;

// Create quantities
let mass = Mass::new(5.0);
let length = Length::new(10.0);
let time = Time::new(2.0);

// Convert between units
let kg = Kilogram::from(mass);
let meters = Metre::from(length);

// Physics calculations with compile-time checking
let velocity = calculate_velocity(length, time);  // v = d/t
let kinetic_energy = calculate_kinetic_energy(mass, velocity);  // KE = ½mv²

// Temperature conversions
let fahrenheit = celsius_to_fahrenheit(25.0);  // 77°F
let celsius = fahrenheit_to_celsius(77.0);     // 25°C
```

### Dimensional Arithmetic

```rust
use ferrunitas::*;

let distance = Length::new(100.0);  // 100 m
let time = Time::new(10.0);         // 10 s
let velocity = distance / time;      // 10 m/s (automatically Velocity type)

let mass = Mass::new(5.0);          // 5 kg
let force = mass * acceleration;    // F = ma (automatically Force type)

// This would fail at compile time:
// let invalid = mass + distance;   // ❌ Cannot add Mass and Length!
```

### Unit Conversions

```rust
use ferrunitas::*;

// Forward conversions (Unit → Quantity)
let kg_unit = Kilogram::new(5.0);
let mass_quantity = kg_unit.into_quantity();

// Backward conversions (Quantity → Unit)
let mass = Mass::new(5000.0);  // 5000 g in base units
let kg = Kilogram::from(mass);  // 5.0 kg
let lb = Pound::from(mass);     // 11.023 lb
```

## Library Structure

- `src/lib.rs` - Main library entry point with physics functions
- `src/model/` - Core types (Quantity, Unit, Prefix)
- `src/system/` - Unit definitions and system setup
- `tests/` - Comprehensive test suites

## Testing

Run the test suite:

```bash
cargo test
```

Run the demo:

```bash
cargo run --bin ferrunitas-demo
```

## Examples

See the demo binary (`src/main.rs`) for comprehensive examples of:
- Unit conversions
- Physics calculations
- Temperature handling
- Recipe conversions
- Round-trip precision tests
- Prefix system usage

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue for any suggestions or improvements.

## License

This project is licensed under the MIT License.