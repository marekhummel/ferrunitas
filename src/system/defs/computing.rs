//! Computing related units

use crate::system::defs::base::{Centimetre, Inch, Second};
use crate::system::prefixes::*;
use crate::{quantity, unit};
use typenum::*;

// ===========================
// DATA (DIMENSIONLESS)
// ===========================
quantity!(Information: M Z0, L Z0, T Z0, I Z0, Th Z0, N Z0, J Z0; marked); // Dimensionless

// Binary units
unit!(base: Bit, "bit", Information; prefixable);
unit!(derived: Byte, "B", (8, Bit); prefixable);
unit!(derived: Nibble, "nibble", (4, Bit));
unit!(derived: Word, "word", (16, Bit)); // 16-bit word (common in computing)
unit!(derived: DoubleWord, "dword", (32, Bit)); // 32-bit double word
unit!(derived: QuadWord, "qword", (64, Bit)); // 64-bit quad word

// Decimal prefixes for bits and bytes
unit!(prefix: Kilobit, Kilo, Bit);
unit!(prefix: Megabit, Mega, Bit);
unit!(prefix: Gigabit, Giga, Bit);
unit!(prefix: Terabit, Tera, Bit);
unit!(prefix: Petabit, Peta, Bit);
unit!(prefix: Exabit, Exa, Bit);

unit!(prefix: Kilobyte, Kilo, Byte);
unit!(prefix: Megabyte, Mega, Byte);
unit!(prefix: Gigabyte, Giga, Byte);
unit!(prefix: Terabyte, Tera, Byte);
unit!(prefix: Petabyte, Peta, Byte);
unit!(prefix: Exabyte, Exa, Byte);
unit!(prefix: Zettabyte, Zetta, Byte);
unit!(prefix: Yottabyte, Yotta, Byte);

// Binary prefixes for bits and bytes
unit!(prefix: Kibibit, Kibi, Bit);
unit!(prefix: Mebibit, Mebi, Bit);
unit!(prefix: Gibibit, Gibi, Bit);
unit!(prefix: Tebibit, Tebi, Bit);
unit!(prefix: Pebibit, Pebi, Bit);
unit!(prefix: Exbibit, Exbi, Bit);
unit!(prefix: Zebibit, Zebi, Bit);
unit!(prefix: Yobibit, Yobi, Bit);

unit!(prefix: Kibibyte, Kibi, Byte);
unit!(prefix: Mebibyte, Mebi, Byte);
unit!(prefix: Gibibyte, Gibi, Byte);
unit!(prefix: Tebibyte, Tebi, Byte);
unit!(prefix: Pebibyte, Pebi, Byte);
unit!(prefix: Exbibyte, Exbi, Byte);
unit!(prefix: Zebibyte, Zebi, Byte);
unit!(prefix: Yobibyte, Yobi, Byte);

// ===========================
// DATA THROUGHPUT / BANDWIDTH
// ===========================
quantity!(DataRate: M Z0, L Z0, T N1, I Z0, Th Z0, N Z0, J Z0; marked); // Information per time

// Base units for data rates
unit!(compound: BitPerSecond, "bit/s", [(Bit, P1), (Second, N1)]; prefixable, marked DataRate);
unit!(compound: BytePerSecond, "B/s", [(Byte, P1), (Second, N1)]; prefixable, marked DataRate);

// Common bandwidth units with decimal prefixes
unit!(prefix: KilobitPerSecond, Kilo, BitPerSecond);
unit!(prefix: MegabitPerSecond, Mega, BitPerSecond);
unit!(prefix: GigabitPerSecond, Giga, BitPerSecond);
unit!(prefix: TerabitPerSecond, Tera, BitPerSecond);

unit!(prefix: KilobytePerSecond, Kilo, BytePerSecond);
unit!(prefix: MegabytePerSecond, Mega, BytePerSecond);
unit!(prefix: GigabytePerSecond, Giga, BytePerSecond);
unit!(prefix: TerabytePerSecond, Tera, BytePerSecond);

// Binary prefix bandwidth units
unit!(prefix: KibibitPerSecond, Kibi, BitPerSecond);
unit!(prefix: MebibitPerSecond, Mebi, BitPerSecond);
unit!(prefix: GibibitPerSecond, Gibi, BitPerSecond);
unit!(prefix: TebibitPerSecond, Tebi, BitPerSecond);

unit!(prefix: KibibytePerSecond, Kibi, BytePerSecond);
unit!(prefix: MebibytePerSecond, Mebi, BytePerSecond);
unit!(prefix: GibibytePerSecond, Gibi, BytePerSecond);
unit!(prefix: TebibytePerSecond, Tebi, BytePerSecond);

// ===========================
// COMPUTATIONAL PERFORMANCE
// ===========================
quantity!(ComputationalRate: M Z0, L Z0, T N1, I Z0, Th Z0, N Z0, J Z0; marked); // Operations per time (dimensionless/time)

// Instructions per second
unit!(base: InstructionPerSecond, "IPS", ComputationalRate; prefixable);
unit!(prefix: KiloInstructionPerSecond, Kilo, InstructionPerSecond);
unit!(prefix: MegaInstructionPerSecond, Mega, InstructionPerSecond);
unit!(prefix: GigaInstructionPerSecond, Giga, InstructionPerSecond);

// Floating point operations per second
unit!(base: FloatingPointOperationPerSecond, "FLOPS", ComputationalRate; prefixable);
unit!(prefix: KiloFloatingPointOperationPerSecond, Kilo, FloatingPointOperationPerSecond);
unit!(prefix: MegaFloatingPointOperationPerSecond, Mega, FloatingPointOperationPerSecond);
unit!(prefix: GigaFloatingPointOperationPerSecond, Giga, FloatingPointOperationPerSecond);
unit!(prefix: TeraFloatingPointOperationPerSecond, Tera, FloatingPointOperationPerSecond);
unit!(prefix: PetaFloatingPointOperationPerSecond, Peta, FloatingPointOperationPerSecond);
unit!(prefix: ExaFloatingPointOperationPerSecond, Exa, FloatingPointOperationPerSecond);

// ===========================
// RESOLUTION / PIXEL DENSITY
// ===========================
quantity!(PixelDensity: M Z0, L N1, T Z0, I Z0, Th Z0, N Z0, J Z0; marked); // Pixels per area

unit!(compound: PixelPerInch, "PPI", [(Inch, N1)]; marked PixelDensity); // Pixels per inch
unit!(compound: DotPerInch, "DPI", [(Inch, N1)]; marked PixelDensity); // Dots per inch (printing)
unit!(compound: PixelPerCentimetre, "PPCM", [(Centimetre, N1)]; marked PixelDensity); // Pixels per cm

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::verify_unit;

    // DATA (DIMENSIONLESS)
    verify_unit!(Bit, Information, 1.0);
    verify_unit!(Byte, Information, 8.0);
    verify_unit!(Nibble, Information, 4.0);
    verify_unit!(Word, Information, 16.0);
    verify_unit!(DoubleWord, Information, 32.0);
    verify_unit!(QuadWord, Information, 64.0);

    // DECIMAL PREFIXED DATA UNITS
    verify_unit!(Kilobit, Information, 1000.0);
    verify_unit!(Megabit, Information, 1_000_000.0);
    verify_unit!(Gigabit, Information, 1_000_000_000.0);
    verify_unit!(Terabit, Information, 1_000_000_000_000.0);
    verify_unit!(Petabit, Information, 1_000_000_000_000_000.0);
    verify_unit!(Exabit, Information, 1_000_000_000_000_000_000.0);

    verify_unit!(Kilobyte, Information, 8_000.0);
    verify_unit!(Megabyte, Information, 8_000_000.0);
    verify_unit!(Gigabyte, Information, 8_000_000_000.0);
    verify_unit!(Terabyte, Information, 8_000_000_000_000.0);
    verify_unit!(Petabyte, Information, 8_000_000_000_000_000.0);
    verify_unit!(Exabyte, Information, 8_000_000_000_000_000_000.0);
    verify_unit!(Zettabyte, Information, 8e21);
    verify_unit!(Yottabyte, Information, 8e24);

    // BINARY PREFIXED DATA UNITS
    verify_unit!(Kibibit, Information, 1024.0);
    verify_unit!(Mebibit, Information, 1_048_576.0);
    verify_unit!(Gibibit, Information, 1_073_741_824.0);
    verify_unit!(Tebibit, Information, 1_099_511_627_776.0);
    verify_unit!(Pebibit, Information, 1_125_899_906_842_624.0);
    verify_unit!(Exbibit, Information, 1_152_921_504_606_846_976.0);
    verify_unit!(Zebibit, Information, 1_180_591_620_717_411_303_424.0);
    verify_unit!(Yobibit, Information, 1_208_925_819_614_629_174_706_176.0);

    verify_unit!(Kibibyte, Information, 8_192.0);
    verify_unit!(Mebibyte, Information, 8_388_608.0);
    verify_unit!(Gibibyte, Information, 8_589_934_592.0);
    verify_unit!(Tebibyte, Information, 8_796_093_022_208.0);
    verify_unit!(Pebibyte, Information, 9_007_199_254_740_992.0);
    verify_unit!(Exbibyte, Information, 9_223_372_036_854_775_808.0);
    verify_unit!(Zebibyte, Information, 9_444_732_965_739_290_427_392.0);
    verify_unit!(Yobibyte, Information, 9_671_406_556_917_033_397_649_408.0);

    // DATA RATE UNITS
    verify_unit!(BitPerSecond, DataRate, 1.0);
    verify_unit!(BytePerSecond, DataRate, 8.0);

    // DECIMAL PREFIXED DATA RATE UNITS
    verify_unit!(KilobitPerSecond, DataRate, 1_000.0);
    verify_unit!(MegabitPerSecond, DataRate, 1_000_000.0);
    verify_unit!(GigabitPerSecond, DataRate, 1_000_000_000.0);
    verify_unit!(TerabitPerSecond, DataRate, 1_000_000_000_000.0);

    verify_unit!(KilobytePerSecond, DataRate, 8_000.0);
    verify_unit!(MegabytePerSecond, DataRate, 8_000_000.0);
    verify_unit!(GigabytePerSecond, DataRate, 8_000_000_000.0);
    verify_unit!(TerabytePerSecond, DataRate, 8_000_000_000_000.0);

    // BINARY PREFIXED DATA RATE UNITS
    verify_unit!(KibibitPerSecond, DataRate, 1_024.0);
    verify_unit!(MebibitPerSecond, DataRate, 1_048_576.0);
    verify_unit!(GibibitPerSecond, DataRate, 1_073_741_824.0);
    verify_unit!(TebibitPerSecond, DataRate, 1_099_511_627_776.0);

    verify_unit!(KibibytePerSecond, DataRate, 8_192.0);
    verify_unit!(MebibytePerSecond, DataRate, 8_388_608.0);
    verify_unit!(GibibytePerSecond, DataRate, 8_589_934_592.0);
    verify_unit!(TebibytePerSecond, DataRate, 8_796_093_022_208.0);

    // COMPUTATIONAL PERFORMANCE UNITS
    verify_unit!(InstructionPerSecond, ComputationalRate, 1.0);
    verify_unit!(KiloInstructionPerSecond, ComputationalRate, 1_000.0);
    verify_unit!(MegaInstructionPerSecond, ComputationalRate, 1_000_000.0);
    verify_unit!(GigaInstructionPerSecond, ComputationalRate, 1_000_000_000.0);

    verify_unit!(FloatingPointOperationPerSecond, ComputationalRate, 1.0);
    verify_unit!(
        KiloFloatingPointOperationPerSecond,
        ComputationalRate,
        1_000.0
    );
    verify_unit!(
        MegaFloatingPointOperationPerSecond,
        ComputationalRate,
        1_000_000.0
    );
    verify_unit!(
        GigaFloatingPointOperationPerSecond,
        ComputationalRate,
        1_000_000_000.0
    );
    verify_unit!(
        TeraFloatingPointOperationPerSecond,
        ComputationalRate,
        1_000_000_000_000.0
    );
    verify_unit!(
        PetaFloatingPointOperationPerSecond,
        ComputationalRate,
        1_000_000_000_000_000.0
    );
    verify_unit!(
        ExaFloatingPointOperationPerSecond,
        ComputationalRate,
        1_000_000_000_000_000_000.0
    );

    // PIXEL DENSITY UNITS
    verify_unit!(PixelPerInch, PixelDensity, 39.3700787402); // 1/inch in 1/m
    verify_unit!(DotPerInch, PixelDensity, 39.3700787402); // Same as PPI
    verify_unit!(PixelPerCentimetre, PixelDensity, 100.0); // 1/cm in 1/m
}
