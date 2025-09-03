//! SI & additional decimal prefixes.
//!
//! Provides prefix marker types (e.g. `Kilo`, `Milli`) with scale factors and
//! symbols used by the `unit!` macro to generate prefixed unit aliases.

use crate::prefix;

prefix!(Quecto, 1e-30, "q");
prefix!(Ronto, 1e-27, "r");
prefix!(Yocto, 1e-24, "y");
prefix!(Zepto, 1e-21, "z");
prefix!(Atto, 1e-18, "a");
prefix!(Femto, 1e-15, "f");
prefix!(Pico, 1e-12, "p");
prefix!(Nano, 1e-9, "n");
prefix!(Micro, 1e-6, "μ");
prefix!(Milli, 1e-3, "m");
prefix!(Centi, 1e-2, "c");
prefix!(Deci, 1e-1, "d");

prefix!(Deca, 1e1, "da");
prefix!(Hecto, 1e2, "h");
prefix!(Kilo, 1e3, "k");
prefix!(Mega, 1e6, "M");
prefix!(Giga, 1e9, "G");
prefix!(Tera, 1e12, "T");
prefix!(Peta, 1e15, "P");
prefix!(Exa, 1e18, "E");
prefix!(Zetta, 1e21, "Z");
prefix!(Yotta, 1e24, "Y");
prefix!(Ronna, 1e27, "R");
prefix!(Quetta, 1e30, "Q");

// ---

prefix!(Kibi, 1u128 << 10, "Ki");
prefix!(Mebi, 1u128 << 20, "Mi");
prefix!(Gibi, 1u128 << 30, "Gi");
prefix!(Tebi, 1u128 << 40, "Ti");
prefix!(Pebi, 1u128 << 50, "Pi");
prefix!(Exbi, 1u128 << 60, "Ei");
prefix!(Zebi, 1u128 << 70, "Zi");
prefix!(Yobi, 1u128 << 80, "Yi");

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::prefix::Prefix;

    #[test]
    fn accuracy_binary_prefixes() {
        // Works, because even if the number itself exceeds the mantissa size for f64 (53bits)
        // powers of two are just stored in the exponent.
        assert_eq!(Kibi::FACTOR as u128, 1u128 << 10);
        assert_eq!(Mebi::FACTOR as u128, 1u128 << 20);
        assert_eq!(Gibi::FACTOR as u128, 1u128 << 30);
        assert_eq!(Tebi::FACTOR as u128, 1u128 << 40);
        assert_eq!(Pebi::FACTOR as u128, 1u128 << 50);
        assert_eq!(Exbi::FACTOR as u128, 1u128 << 60);
        assert_eq!(Zebi::FACTOR as u128, 1u128 << 70);
        assert_eq!(Yobi::FACTOR as u128, 1u128 << 80);
    }
}
