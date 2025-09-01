use crate::model::macros::{quantity, unit};
use crate::model::quantity::Quantity;
use crate::system::defs::mechanics::PoundForce;
use crate::system::prefixes::*;
use typenum::*;

// ===========================
// MASS
// ===========================
quantity!(Mass: M P1, L Z0, T Z0, I Z0, Th Z0, N Z0, J Z0);

unit!(base: Gram, "g", Mass; prefixable, factor = 0.001,); // To counteract mass anomaly, that kilogram is base for mass

unit!(prefix: Kilogram, Kilo, Gram);
unit!(prefix: Milligram, Milli, Gram);
unit!(prefix: Microgram, Micro, Gram);
unit!(prefix: Nanogram, Nano, Gram);
unit!(prefix: Picogram, Pico, Gram);

unit!(derived: Tonne, "t", (1000, Kilogram); prefixable);
unit!(prefix: Kilotonne, Kilo, Tonne);
unit!(prefix: Megatonne, Mega, Tonne);
unit!(prefix: Gigatonne, Giga, Tonne);

unit!(derived: Carat, "ct", (0.2, Gram));
unit!(derived: Dalton, "Da", (1.660_539_066_60e-24, Gram));

unit!(derived: Ounce, "oz", (28.349_523_125, Gram));
unit!(derived: Pound, "lb", (16, Ounce));
unit!(derived: Stone, "st", (14, Pound));
unit!(derived: HundredweightUK, "cwt", (8, Stone));
unit!(derived: HundredweightUS, "cwt", (100, Pound));
unit!(derived: TonUK, "ton", (20, HundredweightUK)); // Long ton
unit!(derived: TonUS, "ton", (20, HundredweightUS)); // Short ton

// Historical and specialized mass units
unit!(derived: Grain, "gr", (64.79891, Milligram));
unit!(derived: Pennyweight, "dwt", (24, Grain));
unit!(derived: TroyOunce, "oz t", (20, Pennyweight));
unit!(derived: TroyPound, "lb t", (12, TroyOunce));
unit!(compound: Slug, "slug", [(PoundForce, P1), (Second, P2), (Foot, N1)]);

// ===========================
// LENGTH
// ===========================
quantity!(Length: M Z0, L P1, T Z0, I Z0, Th Z0, N Z0, J Z0);

unit!(base: Metre, "m", Length; prefixable,);

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

unit!(derived: NauticalMile, "NM", (1852, Metre); prefixable);
unit!(prefix: KilonauticalMile, Kilo, NauticalMile);

// Astronomical and very small length units
unit!(derived: Angstrom, "Å", (1e-10, Metre));
unit!(derived: AstronomicalUnit, "AU", (149_597_870_700i64, Metre));
unit!(derived: LightYear, "ly", (9_460_730_472_580_800i64, Metre));
unit!(derived: Parsec, "pc", (30_856_775_814_914_400i64, Metre); prefixable);
unit!(prefix: Kiloparsec, Kilo, Parsec);
unit!(prefix: Megaparsec, Mega, Parsec);

// ===========================
// TIME
// ===========================
quantity!(Time: M Z0, L Z0, T P1, I Z0, Th Z0, N Z0, J Z0);

unit!(base: Second, "s", Time; prefixable,);

unit!(prefix: Millisecond, Milli, Second);
unit!(prefix: Microsecond, Micro, Second);
unit!(prefix: Nanosecond, Nano, Second);
unit!(prefix: Picosecond, Pico, Second);

unit!(derived: Minute, "min", (60, Second));
unit!(derived: Hour, "h", (60, Minute));
unit!(derived: Day, "d", (24, Hour)); // Solar Day
unit!(derived: Week, "week", (7, Day));
unit!(derived: Year, "year", (365.2425, Day)); // Average year with leap years
unit!(derived: Month, "month", (1.0 / 12.0, Year)); // Average month
unit!(derived: Century, "century", (100, Year));
unit!(derived: Millennium, "millennium", (1000, Year));

// Specialized time units
unit!(derived: Fortnight, "fortnight", (14, Day));
unit!(derived: Shake, "shake", (1, Nanosecond)); // Nuclear physics unit
unit!(derived: Svedberg, "S", (1e-13, Second)); // Sedimentation coefficient time unit

// Astronomical time units
unit!(derived: SiderealDay, "sidereal day", (86164.0905, Second)); // 366 earth rotations in 365 days
unit!(derived: SiderealMonth, "sidereal month", (27.321661, Day)); // Moon's orbital period
unit!(derived: SiderealYear, "sidereal year", (365.25636, Day)); // Earth's orbital period relative to stars
unit!(derived: TropicalYear, "tropical year", (365.24219, Day)); // Time for seasons to repeat
unit!(derived: AnomalousYear, "anomalous year", (365.25964, Day)); // Earth's perihelion to perihelion
unit!(derived: DraconicYear, "draconic year", (346.6201, Day)); // Eclipse cycle period
unit!(derived: LunarMonth, "lunar month", (29.530589, Day)); // Synodic month - new moon to new moon
unit!(derived: JulianYear, "Julian year", (365.25, Day)); // Exactly 365.25 days
unit!(derived: BesselianYear, "Besselian year", (365.242198781, Day)); // Astronomical year epoch

// ===========================
// ELECTRIC CURRENT
// ===========================
quantity!(ElectricCurrent: M Z0, L Z0, T Z0, I P1, Th Z0, N Z0, J Z0);

unit!(base: Ampere, "A", ElectricCurrent; prefixable,);

unit!(prefix: Milliampere, Milli, Ampere);
unit!(prefix: Microampere, Micro, Ampere);
unit!(prefix: Kiloampere, Kilo, Ampere);

// ===========================
// TEMPERATURE
// ===========================
quantity!(Temperature: M Z0, L Z0, T Z0, I Z0, Th P1, N Z0, J Z0);

unit!(base: Kelvin, "K", Temperature; prefixable,);

unit!(prefix: Millikelvin, Milli, Kelvin);
unit!(prefix: Microkelvin, Micro, Kelvin);

// TODO
// unit!(derived: Celsius, "°C", (1.0, Kelvin));
// unit!(derived: Fahrenheit, "°F", (5.0/9.0, Kelvin)); // offset handling left to conversion logic

// ===========================
// AMOUNT OF SUBSTANCE
// ===========================
quantity!(AmountOfSubstance: M Z0, L Z0, T Z0, I Z0, Th Z0, N P1, J Z0);

unit!(base: Mole, "mol", AmountOfSubstance; prefixable,);

unit!(prefix: Millimole, Milli, Mole);
unit!(prefix: Micromole, Micro, Mole);
unit!(prefix: Nanomole, Nano, Mole);

// ===========================
// LUMINOUS INTENSITY
// ===========================
quantity!(LuminousIntensity: M Z0, L Z0, T Z0, I Z0, Th Z0, N Z0, J P1);

unit!(base: Candela, "cd", LuminousIntensity; prefixable,);

unit!(prefix: Millicandela, Milli, Candela);
unit!(prefix: Microcandela, Micro, Candela);
unit!(prefix: Kilocandela, Kilo, Candela);

// ==============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::verify_unit;

    // MASS
    verify_unit!(Gram, Mass, 1e-3);
    verify_unit!(Kilogram, Mass, 1.0);
    verify_unit!(Milligram, Mass, 1e-6);
    verify_unit!(Microgram, Mass, 1e-9);
    verify_unit!(Nanogram, Mass, 1e-12);
    verify_unit!(Picogram, Mass, 1e-15);
    verify_unit!(Tonne, Mass, 1e3);
    verify_unit!(Kilotonne, Mass, 1e6);
    verify_unit!(Megatonne, Mass, 1e9);
    verify_unit!(Gigatonne, Mass, 1e12);
    verify_unit!(Carat, Mass, 0.0002);
    verify_unit!(Dalton, Mass, 1.660_539_066_60e-27);
    verify_unit!(Ounce, Mass, 0.028349523);
    verify_unit!(Pound, Mass, 0.45359237);
    verify_unit!(Stone, Mass, 6.3502931);
    verify_unit!(HundredweightUK, Mass, 50.80234544);
    verify_unit!(HundredweightUS, Mass, 45.359237);
    verify_unit!(TonUK, Mass, 1.0160469088e3);
    verify_unit!(TonUS, Mass, 907.18474);
    verify_unit!(Grain, Mass, 0.00006479891);
    verify_unit!(Pennyweight, Mass, 0.00155517384);
    verify_unit!(TroyOunce, Mass, 0.03110347680);
    verify_unit!(TroyPound, Mass, 0.37324172);
    verify_unit!(Slug, Mass, 14.593903);

    // LENGTH
    verify_unit!(Metre, Length, 1.0);
    verify_unit!(Kilometre, Length, 1000.0);
    verify_unit!(Decimetre, Length, 0.1);
    verify_unit!(Centimetre, Length, 0.01);
    verify_unit!(Millimetre, Length, 0.001);
    verify_unit!(Micrometre, Length, 1e-6);
    verify_unit!(Nanometre, Length, 1e-9);
    verify_unit!(Inch, Length, 0.0254);
    verify_unit!(Foot, Length, 0.3048);
    verify_unit!(Yard, Length, 0.9144);
    verify_unit!(Mile, Length, 1609.344);
    verify_unit!(Thou, Length, 0.0000254);
    verify_unit!(Rod, Length, 5.0292);
    verify_unit!(Chain, Length, 20.1168);
    verify_unit!(Furlong, Length, 201.168);
    verify_unit!(League, Length, 4828.032);
    verify_unit!(NauticalMile, Length, 1852.0);
    verify_unit!(KilonauticalMile, Length, 1852000.0);
    verify_unit!(Angstrom, Length, 1e-10);
    verify_unit!(AstronomicalUnit, Length, 149597870700.0);
    verify_unit!(LightYear, Length, 9.4607304725808e15);
    verify_unit!(Parsec, Length, 3.08567758149144e16);
    verify_unit!(Kiloparsec, Length, 3.08567758149144e19);
    verify_unit!(Megaparsec, Length, 3.08567758149144e22);

    // TIME
    verify_unit!(Second, Time, 1.0);
    verify_unit!(Millisecond, Time, 0.001);
    verify_unit!(Microsecond, Time, 1e-6);
    verify_unit!(Nanosecond, Time, 1e-9);
    verify_unit!(Picosecond, Time, 1e-12);
    verify_unit!(Minute, Time, 60.0);
    verify_unit!(Hour, Time, 3600.0);
    verify_unit!(Day, Time, 86400.0);
    verify_unit!(Week, Time, 604800.0);
    verify_unit!(Month, Time, 2629746.0);
    verify_unit!(Year, Time, 31556952.0);
    verify_unit!(Century, Time, 3155695200.0);
    verify_unit!(Millennium, Time, 31556952000.0);
    verify_unit!(Fortnight, Time, 1209600.0);
    verify_unit!(Shake, Time, 1e-9);
    verify_unit!(Svedberg, Time, 1e-13);
    verify_unit!(SiderealDay, Time, 86164.0905);
    verify_unit!(SiderealMonth, Time, 2360591.504);
    verify_unit!(SiderealYear, Time, 31558149.504);
    verify_unit!(TropicalYear, Time, 31556925.216);
    verify_unit!(AnomalousYear, Time, 31558432.896);
    verify_unit!(DraconicYear, Time, 29947976.64);
    verify_unit!(LunarMonth, Time, 2551442.896);
    verify_unit!(JulianYear, Time, 31557600.0);
    verify_unit!(BesselianYear, Time, 31556925.974);

    // ELECTRIC CURRENT
    verify_unit!(Ampere, ElectricCurrent, 1.0);
    verify_unit!(Milliampere, ElectricCurrent, 0.001);
    verify_unit!(Microampere, ElectricCurrent, 1e-6);
    verify_unit!(Kiloampere, ElectricCurrent, 1000.0);

    // TEMPERATURE
    verify_unit!(Kelvin, Temperature, 1.0);
    verify_unit!(Millikelvin, Temperature, 0.001);
    verify_unit!(Microkelvin, Temperature, 1e-6);

    // AMOUNT OF SUBSTANCE
    verify_unit!(Mole, AmountOfSubstance, 1.0);
    verify_unit!(Millimole, AmountOfSubstance, 0.001);
    verify_unit!(Micromole, AmountOfSubstance, 1e-6);
    verify_unit!(Nanomole, AmountOfSubstance, 1e-9);

    // LUMINOUS INTENSITY
    verify_unit!(Candela, LuminousIntensity, 1.0);
    verify_unit!(Millicandela, LuminousIntensity, 0.001);
    verify_unit!(Microcandela, LuminousIntensity, 1e-6);
    verify_unit!(Kilocandela, LuminousIntensity, 1000.0);
}
