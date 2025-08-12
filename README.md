# Rust Unit Converter

This project implements a unit conversion system in Rust, leveraging the language's strong type system to define and convert between various physical quantities. The converter supports the seven SI base quantities and allows for derived quantities, unit conversions, and SI prefixes.

## Features

- **SI Base Quantities**: The project defines the seven SI base quantities:
  - Length
  - Mass
  - Time
  - Electric Current
  - Temperature
  - Amount of Substance
  - Luminous Intensity

- **Derived Quantities**: Users can define derived quantities such as:
  - Acceleration (length/time²)
  - Force (mass × acceleration)
  - Energy (force × distance)

- **Unit Definitions**: The project includes various units for each quantity, along with conversion factors to facilitate conversions between units.

- **SI Prefixes**: Support for SI prefixes such as kilo (10³), milli (10⁻³), and others, allowing for easy manipulation of unit scales.

## Usage

To use the unit converter, follow these steps:

1. Clone the repository:
   ```
   git clone <repository-url>
   cd rust-unit-converter
   ```

2. Build the project:
   ```
   cargo build
   ```

3. Run the example conversions:
   ```
   cargo run
   ```

## Example Conversions

The unit converter can perform various conversions, such as:

- Convert 10 meters to kilometers
- Convert 5 kilograms to grams
- Convert 2 hours to seconds

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue for any suggestions or improvements.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.