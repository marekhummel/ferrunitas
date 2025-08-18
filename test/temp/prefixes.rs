#[derive(Debug, Clone, Copy)]
pub enum SIPrefix {
    Yotta, // 10^24, "Y"
    Zetta, // 10^21, "Z"
    Exa,   // 10^18, "E"
    Peta,  // 10^15, "P"
    Tera,  // 10^12, "T"
    Giga,  // 10^9,  "G"
    Mega,  // 10^6,  "M"
    Kilo,  // 10^3,  "k"
    Hecto, // 10^2,  "h"
    Deca,  // 10^1,  "da"
    None,  // 10^0,  ""
    Deci,  // 10^-1, "d"
    Centi, // 10^-2, "c"
    Milli, // 10^-3, "m"
    Micro, // 10^-6, "μ"
    Nano,  // 10^-9, "n"
    Pico,  // 10^-12,"p"
    Femto, // 10^-15,"f"
    Atto,  // 10^-18,"a"
    Zepto, // 10^-21,"z"
    Yocto, // 10^-24,"y"
}

impl SIPrefix {
    pub fn multiplier(&self) -> f64 {
        match self {
            SIPrefix::Yotta => 1e24,
            SIPrefix::Zetta => 1e21,
            SIPrefix::Exa => 1e18,
            SIPrefix::Peta => 1e15,
            SIPrefix::Tera => 1e12,
            SIPrefix::Giga => 1e9,
            SIPrefix::Mega => 1e6,
            SIPrefix::Kilo => 1e3,
            SIPrefix::Hecto => 1e2,
            SIPrefix::Deca => 1e1,
            SIPrefix::None => 1.0,
            SIPrefix::Deci => 1e-1,
            SIPrefix::Centi => 1e-2,
            SIPrefix::Milli => 1e-3,
            SIPrefix::Micro => 1e-6,
            SIPrefix::Nano => 1e-9,
            SIPrefix::Pico => 1e-12,
            SIPrefix::Femto => 1e-15,
            SIPrefix::Atto => 1e-18,
            SIPrefix::Zepto => 1e-21,
            SIPrefix::Yocto => 1e-24,
        }
    }

    pub fn abbreviation(&self) -> &'static str {
        match self {
            SIPrefix::Yotta => "Y",
            SIPrefix::Zetta => "Z",
            SIPrefix::Exa => "E",
            SIPrefix::Peta => "P",
            SIPrefix::Tera => "T",
            SIPrefix::Giga => "G",
            SIPrefix::Mega => "M",
            SIPrefix::Kilo => "k",
            SIPrefix::Hecto => "h",
            SIPrefix::Deca => "da",
            SIPrefix::None => "",
            SIPrefix::Deci => "d",
            SIPrefix::Centi => "c",
            SIPrefix::Milli => "m",
            SIPrefix::Micro => "μ",
            SIPrefix::Nano => "n",
            SIPrefix::Pico => "p",
            SIPrefix::Femto => "f",
            SIPrefix::Atto => "a",
            SIPrefix::Zepto => "z",
            SIPrefix::Yocto => "y",
        }
    }
}
