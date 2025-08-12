use crate::prefixes::SIPrefix;
use crate::quantity::Quantity;

use std::ops::{Div, Mul};

#[derive(Debug, Clone, Copy)]
pub struct Unit {
    pub quantity: Quantity,
    pub factor: f64, // to SI base unit
    pub prefix: SIPrefix,
    pub name: &'static str,
}

impl Unit {
    pub fn to_si(&self, value: f64) -> f64 {
        value * self.prefix.multiplier() * self.factor
    }

    pub fn repr(&self) -> String {
        format!(
            "{} ({}{})",
            self.name,
            self.prefix.abbreviation(),
            self.quantity.canonical_name()
        )
    }
}

impl Mul for Unit {
    type Output = Unit;
    fn mul(self, rhs: Unit) -> Unit {
        Unit {
            quantity: self.quantity * rhs.quantity,
            factor: self.factor * rhs.factor,
            prefix: SIPrefix::None,
            name: "product_unit",
        }
    }
}

impl Div for Unit {
    type Output = Unit;
    fn div(self, rhs: Unit) -> Unit {
        Unit {
            quantity: self.quantity / rhs.quantity,
            factor: self.factor / rhs.factor,
            prefix: SIPrefix::None,
            name: "quotient_unit",
        }
    }
}

// Macro to create a const Quantity from base quantities and exponents
#[macro_export]
macro_rules! unit {
    (
        name: $name:expr,
        components: [ $( ($unt:expr, $exp:expr) ),* ],
        factor: $factor:expr,
        prefix: $prefix:expr
    ) => {
        Unit {
            quantity: quantity! {
                name: Some($name),
                components: [ $( ($unt.quantity, $exp) ),* ]
            },
            factor: $factor,
            prefix: $prefix,
            name: $name,
        }
    };
}
pub(crate) use unit;
