use std::ops::{Div, Mul};
impl Mul for Value {
    type Output = Value;
    fn mul(self, rhs: Value) -> Value {
        Value {
            value: self.value * rhs.value,
            unit: self.unit * rhs.unit,
        }
    }
}

impl Div for Value {
    type Output = Value;
    fn div(self, rhs: Value) -> Value {
        Value {
            value: self.value / rhs.value,
            unit: self.unit / rhs.unit,
        }
    }
}
use crate::unit::Unit;

#[derive(Debug, Clone, Copy)]
pub struct Value {
    pub value: f64,
    pub unit: Unit,
}

impl Value {
    pub fn to_si(&self) -> f64 {
        self.unit.to_si(self.value)
    }

    pub fn convert_to(&self, target: Unit) -> Option<Value> {
        if self.unit.quantity.dimension != target.quantity.dimension {
            return None;
        }
        let si_value = self.to_si();
        let target_value = si_value / target.to_si(1.0);
        Some(Value {
            value: target_value,
            unit: target,
        })
    }
}
