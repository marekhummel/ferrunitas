//! Public unit system namespace.
//!
//! Aggregates all predefined physical domain unit definitions (base, derived,
//! compound) plus SI prefixes & selected constants. To access any unit, import
//! from here: `use ferrunitas::system::*;`.
//!
//! # Module Contents
//!
//! ## Base Units (`defs::base`)
//! Fundamental SI base units and common derived units:
//! - **Mass**: Gram, Kilogram, Pound, Ounce, Tonne, Stone, etc.
//! - **Length**: Metre, Foot, Inch, Mile, Kilometre, Centimetre, etc.
//! - **Time**: Second, Minute, Hour, Day, Year, Nanosecond, etc.
//! - **Electric Current**: Ampere, Milliampere, Kiloampere
//! - **Temperature**: Kelvin, Millikelvin, Microkelvin
//! - **Amount of Substance**: Mole, Millimole, Micromole
//! - **Luminous Intensity**: Candela, Millicandela, Kilocandela
//!
//! ## Mechanical Units (`defs::mechanics`)
//! Classical mechanics quantities and units:
//! - **Force**: Newton, Pound-force, Kilonewton, Dyne
//! - **Pressure**: Pascal, Bar, Atmosphere, PSI, Torr
//! - **Energy**: Joule, Calorie, BTU, Kilowatt-hour, Electron-volt
//! - **Power**: Watt, Horsepower, Kilowatt, Megawatt
//! - **Velocity**: Metre per second, Mile per hour, Knot
//! - **Acceleration**: Metre per second squared, g-force
//! - **Area**: Square metre, Square foot, Acre, Hectare
//! - **Volume**: Cubic metre, Litre, Gallon, Cubic foot
//! - **Density**: Kilogram per cubic metre, Gram per cubic centimetre
//! - **Wave Number**: Reciprocal metre
//! - **Specific Volume**: Cubic metre per kilogram
//! - **Frequency**: Hertz
//! - **Angular Velocity**: Radian per second
//! - **Angular Acceleration**: Radian per second squared
//! - **Torque**: Newton metre
//! - **Moment of Inertia**: Kilogram metre squared
//! - **Dynamic Viscosity**: Pascal second
//! - **Kinematic Viscosity**: Square metre per second
//! - **Surface Tension**: Newton per metre
//! - **Action**: Joule Second
//!
//! ## Electromagnetic Units (`defs::electromagnetism`)
//! Electrical and magnetic quantities:
//! - **Voltage**: Volt, Millivolt, Kilovolt
//! - **Resistance**: Ohm, Kilohm, Megohm
//! - **Capacitance**: Farad, Microfarad, Picofarad
//! - **Conductance**: Siemens, Millisiemens, Microsiemens
//! - **Inductance**: Henry, Millihenry, Microhenry
//! - **Magnetic Field**: Tesla, Gauss, Weber
//! - **Electric Charge**: Coulomb, Ampere-hour
//! - **Magnetic Flux**: Weber
//! - **Electric Field Strength**: Volt per metre
//! - **Magnetic Field Strength**: Ampere per metre
//! - **Electric Conductivity**: Siemens per metre
//! - **Electric Displacement Field**: Coulomb per square metre
//! - **Permittivity**: Farad per metre
//! - **Permeability**: Henry per metre
//! - **Current Density**: Ampere per square metre
//! - **Charge Density**: Coulomb per cubic metre
//! - **Magnetic Moment**: Ampere metre squared
//!
//! ## Thermodynamic Units (`defs::thermodynamics`)
//! Heat and thermodynamic quantities:
//! - **Heat Capacity**: Joule per Kelvin
//! - **Specific Heat Capacity**: Joule per kilogram-Kelvin
//! - **Thermal Conductivity**: Watt per metre-Kelvin
//! - **Thermal Resistance**: Metre-Kelvin per Watt
//! - **Thermal Diffusivity**: Square metre per second
//! - **Heat Transfer Coefficient**: Watt per square metre-Kelvin
//! - **Thermal Expansion**: Reciprocal Kelvin
//!
//! ## Chemical Units (`defs::chemical`)
//! Chemistry-related quantities:
//! - **Molar Mass**: Kilogram per mole, Gram per mole
//! - **Molar Concentration**: MolePerLitre
//! - **Catalytic Activity**: Katal
//! - **Mass Concentration**: Kilogram per cubic metre, Gram per cubic centimetre
//! - **Catalytic Concentration**: Mole per cubic metre
//!
//! ## Computing Units (`defs::computing`)
//! Digital and computing-related quantities:
//! - **Information**: Bit, Byte, Kilobyte, Megabyte, Gibibyte
//! - **Data Rate**: Bits per second, Megabits per second, Gigabytes per second
//! - **Computational Performance**: MIPS, FLOPS, Instructions per second
//! - **Pixel Density**: Pixels per inch, Dots per inch
//!
//! ## Photometric Units (`defs::photometric`)
//! Light and optical measurements:
//! - **Luminous Flux**: Lumen
//! - **Luminance**: Candela per square metre, Nit
//! - **Illuminance**: Lux, Foot-candle
//! - **Optical Power**: Dioptre
//! - **Luminous Energy**: LumenSecond, Talbot
//! - **Luminous Exposure**: LuxSecond
//! - **Luminous Efficacy**: Lumen per Watt
//! - **Radiance**: Watt per steradian
//! - **Irradiance**: Watt per square metre
//! - **Spectral Radiance**: Watt per steradian per square metre per metre
//!
//! ## Radiation Units (`defs::radiation`)
//! Radioactivity and radiation measurements:
//! - **Radioactivity**: Becquerel, Curie
//! - **Absorbed Dose**: Gray, Rad
//! - **Equivalent Dose**: Sievert, Rem
//! - **Exposure**: Coulomb per kilogram, Roentgen
//! - **Dose Rate**: Gray per second, Rad per second
//! - **Nuclear Cross Section**: Barn
//! - **Fluence**: Particle per square metre
//! - **Flux Density**: Particle per square metre second
//!
//! ## Acoustic Units (`defs::acoustic`)
//! Sound and acoustic measurements:
//! - **Acoustic Impedance**: Rayl
//! - **Sound Intensity**: Watt per square metre
//! - **Sound Exposure**: Pascal squared second
//!
//! ## Dimensionless Units (`defs::dimensionless`)
//! Ratios, percentages, and dimensionless quantities:
//! - **Percentages**: One, Percent, Permille, PartsPerMillion
//! - **Angle**: Radian, Degree, Gradian
//! - **Solid Angle**: Steradian
//! - **Logarithmic**: Neper, Bel
//!
//! ## SI Prefixes (`prefixes`)
//! Standard SI decimal and binary prefixes:
//! - **Decimal**: Kilo (k), Mega (M), Giga (G), Tera (T), Peta (P), etc.
//! - **Sub-unit**: Milli (m), Micro (μ), Nano (n), Pico (p), etc.
//! - **Binary**: Kibi (Ki), Mebi (Mi), Gibi (Gi), Tebi (Ti), etc.
//!
//! ## Physical Constants (`constants`)
//! Fundamental physical constants with proper units:
//! - Speed of light, Planck constant, Electron mass, etc.

// Modules can stay private, as all members are publicly re-exported
mod constants;
mod defs;
mod prefixes;

// Public export of all prefixes, constants and units under ferrunitas::system::*
pub use constants::*;
pub use prefixes::*;

pub use defs::acoustic::*;
pub use defs::base::*;
pub use defs::chemical::*;
pub use defs::computing::*;
pub use defs::dimensionless::*;
pub use defs::electromagnetism::*;
pub use defs::mechanics::*;
pub use defs::photometric::*;
pub use defs::radiation::*;
pub use defs::thermodynamics::*;
