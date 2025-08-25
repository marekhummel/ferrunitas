use crate::model::quantity::Quantity;
use crate::model::unit::Unit;
use crate::system::prefixes::*;
use crate::unit;
use typenum::*;

// ============================================================================
// MASS
// ============================================================================
pub type Mass = Quantity<P1, Z0, Z0, Z0, Z0, Z0, Z0>;

unit!(base: Gram, Mass, "g", prefixable);

unit!(prefix: Kilogram, Kilo, Gram);
unit!(prefix: Milligram, Milli, Gram);
unit!(prefix: Microgram, Micro, Gram);
unit!(prefix: Nanogram, Nano, Gram);
unit!(prefix: Picogram, Pico, Gram);

unit!(derived: Tonne, "t", (1000, Kilogram), prefixable);
unit!(prefix: Kilotonne, Kilo, Tonne);
unit!(prefix: Megatonne, Mega, Tonne);
unit!(prefix: Gigatonne, Giga, Tonne);

unit!(derived: Carat, "ct", (0.2, Gram));
unit!(derived: Dalton, "Da", (1.660_539_066_60e-24, Gram));
unit!(derived: UnifiedAtomicMassUnit, "u", (1.660_539_066_60e-24, Gram));

unit!(derived: Ounce, "oz", (28.3495, Gram));
unit!(derived: Pound, "lb", (16, Ounce));
unit!(derived: Stone, "st", (14, Pound));
unit!(derived: HundredweightUK, "cwt", (112, Pound));
unit!(derived: HundredweightUS, "cwt", (100, Pound));
unit!(derived: TonUK, "ton", (20, HundredweightUK));
unit!(derived: TonUS, "ton", (20, HundredweightUS));

// Historical and specialized mass units
unit!(derived: Grain, "gr", (64.79891, Milligram));
unit!(derived: Pennyweight, "dwt", (20, Grain));
unit!(derived: TroyOunce, "oz t", (20, Pennyweight));
unit!(derived: TroyPound, "lb t", (12, TroyOunce));
unit!(derived: Slug, "slug", (14.593903, Kilogram));

// ============================================================================
// LENGTH
// ============================================================================
pub type Length = Quantity<Z0, P1, Z0, Z0, Z0, Z0, Z0>;

unit!(base: Metre, Length, "m", prefixable);

unit!(prefix: Kilometre, Kilo, Metre);
unit!(prefix: Decimetre, Deci, Metre);
unit!(prefix: Centimetre, Centi, Metre);
unit!(prefix: Millimetre, Milli, Metre);
unit!(prefix: Micrometre, Micro, Metre);
unit!(prefix: Nanometre, Nano, Metre);

unit!(derived: Inch, "in", (2.54, Centimetre));
unit!(derived: Foot, "ft", (12, Inch));
unit!(derived: Yard, "yd", (3, Foot));
unit!(derived: Mile, "mi", (1760, Yard));
unit!(derived: Thou, "thou", (1e-3, Inch));
unit!(derived: Rod, "rod", (5.5, Yard));
unit!(derived: Chain, "ch", (4, Rod));
unit!(derived: Furlong, "fur", (10, Chain));
unit!(derived: League, "lea", (3, Mile));

unit!(derived: NauticalMile, "NM", (1852, Metre), prefixable);
unit!(prefix: KilonauticalMile, Kilo, NauticalMile);

// Astronomical and very small length units
unit!(derived: Angstrom, "Å", (1e-10, Metre));
unit!(derived: AstronomicalUnit, "AU", (149_597_870_700i64, Metre));
unit!(derived: LightYear, "ly", (9_460_730_472_580_800i64, Metre));
unit!(derived: Parsec, "pc", (30_856_775_814_914_400i64, Metre), prefixable);
unit!(prefix: Kiloparsec, Kilo, Parsec);
unit!(prefix: Megaparsec, Mega, Parsec);

// ============================================================================
// TIME
// ============================================================================
pub type Time = Quantity<Z0, Z0, P1, Z0, Z0, Z0, Z0>;

unit!(base: Second, Time, "s", prefixable);

unit!(prefix: Millisecond, Milli, Second);
unit!(prefix: Microsecond, Micro, Second);
unit!(prefix: Nanosecond, Nano, Second);
unit!(prefix: Picosecond, Pico, Second);

unit!(derived: Minute, "min", (60, Second));
unit!(derived: Hour, "h", (60, Minute));
unit!(derived: Day, "d", (24, Hour));
unit!(derived: Week, "week", (7, Day));
unit!(derived: Month, "month", (30.44, Day)); // Average month
unit!(derived: Year, "year", (365.2425, Day)); // Average year with leap years
unit!(derived: Century, "century", (100, Year));
unit!(derived: Millennium, "millennium", (1000, Year));

// Specialized time units
unit!(derived: Fortnight, "fortnight", (14, Day));
unit!(derived: Shake, "shake", (1, Nanosecond)); // Nuclear physics unit
unit!(derived: Svedberg, "S", (1e-13, Second)); // Sedimentation coefficient time unit

// ============================================================================
// ELECTRIC CURRENT
// ============================================================================
pub type ElectricCurrent = Quantity<Z0, Z0, Z0, P1, Z0, Z0, Z0>;

unit!(base: Ampere, ElectricCurrent, "A", prefixable);

unit!(prefix: Milliampere, Milli, Ampere);
unit!(prefix: Microampere, Micro, Ampere);
unit!(prefix: Kiloampere, Kilo, Ampere);

// ============================================================================
// TEMPERATURE
// ============================================================================
pub type Temperature = Quantity<Z0, Z0, Z0, Z0, P1, Z0, Z0>;

unit!(base: Kelvin, Temperature, "K", prefixable);

unit!(prefix: Millikelvin, Milli, Kelvin);
unit!(prefix: Microkelvin, Micro, Kelvin);

// TODO
// unit!(derived: Celsius, "°C", (1.0, Kelvin));
// unit!(derived: Fahrenheit, "°F", (5.0/9.0, Kelvin)); // offset handling left to conversion logic

// ============================================================================
// AMOUNT OF SUBSTANCE
// ============================================================================
pub type AmountOfSubstance = Quantity<Z0, Z0, Z0, Z0, Z0, P1, Z0>;

unit!(base: Mole, AmountOfSubstance, "mol", prefixable);

unit!(prefix: Millimole, Milli, Mole);
unit!(prefix: Micromole, Micro, Mole);
unit!(prefix: Nanomole, Nano, Mole);

// ============================================================================
// LUMINOUS INTENSITY
// ============================================================================
pub type LuminousIntensity = Quantity<Z0, Z0, Z0, Z0, Z0, Z0, P1>;

unit!(base: Candela, LuminousIntensity, "cd", prefixable);

unit!(prefix: Millicandela, Milli, Candela);
unit!(prefix: Microcandela, Micro, Candela);
unit!(prefix: Kilocandela, Kilo, Candela);
