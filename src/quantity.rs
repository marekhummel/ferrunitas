use std::collections::HashMap;
use std::sync::{LazyLock, Mutex, OnceLock};

pub struct QuantityRegistry {
    map: HashMap<[i32; 7], &'static str>,
}

impl QuantityRegistry {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        QuantityRegistry { map }
    }

    pub fn get_name(&self, dim: &[i32; 7]) -> Option<&'static str> {
        self.map.get(dim).copied()
    }
}

static QUANTITY_REGISTRY: LazyLock<Mutex<QuantityRegistry>> =
    LazyLock::new(|| Mutex::new(QuantityRegistry::new()));

use std::ops::{Div, Mul};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Quantity {
    pub dimension: [i32; 7], // L, M, T, I, Θ, N, J
    pub name: Option<&'static str>,
}

impl Quantity {
    pub fn new(dimension: [i32; 7], name: Option<&'static str>) -> Self {
        let mut registry = QUANTITY_REGISTRY.lock().unwrap();
        let reg_name = registry.get_name(&dimension);
        if reg_name.is_some() && name.is_some() {
            panic!(
                "Conflicting names for quantity: registry has '{}' and parameter is '{}'",
                reg_name.unwrap(),
                name.unwrap()
            );
        }
        if let Some(n) = name {
            registry.map.insert(dimension, n);
        }
        let canonical = name.or(reg_name);
        Quantity {
            dimension,
            name: canonical,
        }
    }

    pub const fn powi(self, n: i32) -> Quantity {
        let new_dim = [
            self.dimension[0] * n,
            self.dimension[1] * n,
            self.dimension[2] * n,
            self.dimension[3] * n,
            self.dimension[4] * n,
            self.dimension[5] * n,
            self.dimension[6] * n,
        ];
        Quantity {
            dimension: new_dim,
            name: None,
        }
    }

    pub fn recip(self) -> Quantity {
        let mut new_dim = [0; 7];
        for i in 0..7 {
            new_dim[i] = -self.dimension[i];
        }
        Quantity {
            dimension: new_dim,
            name: None,
        }
    }

    pub fn set_name(&mut self, name: &'static str) {
        self.name = Some(name);
        let mut registry = QUANTITY_REGISTRY.lock().unwrap();
        registry.map.insert(self.dimension, name);
    }

    pub fn repr(&self) -> String {
        format!("{} [{:?}]", self.canonical_name(), self.dimension)
    }

    pub fn canonical_name(&self) -> &'static str {
        let registry = QUANTITY_REGISTRY.lock().unwrap();
        self.name
            .or_else(|| registry.get_name(&self.dimension))
            .unwrap_or("unknown")
    }
}

impl Mul for Quantity {
    type Output = Quantity;
    fn mul(self, rhs: Quantity) -> Quantity {
        let mut new_dim = [0; 7];
        new_dim.iter_mut().enumerate().for_each(|(i, val)| {
            *val = self.dimension[i] + rhs.dimension[i];
        });
        Quantity {
            dimension: new_dim,
            name: None,
        }
    }
}

impl Div for Quantity {
    type Output = Quantity;
    fn div(self, rhs: Quantity) -> Quantity {
        let mut new_dim = [0; 7];
        new_dim.iter_mut().enumerate().for_each(|(i, val)| {
            *val = self.dimension[i] - rhs.dimension[i];
        });
        Quantity {
            dimension: new_dim,
            name: None,
        }
    }
}

// Macro to create a const Quantity from base quantities and exponents
#[macro_export]
macro_rules! quantity {
    (
        name: $name:expr,
        components: [ $( ($qty:expr, $exp:expr) ),* ]
    ) => {{
        const fn build_dim() -> [i32; 7] {
            let mut dim = [0; 7];
            $(
                dim[0] += $qty.dimension[0] * $exp;
                dim[1] += $qty.dimension[1] * $exp;
                dim[2] += $qty.dimension[2] * $exp;
                dim[3] += $qty.dimension[3] * $exp;
                dim[4] += $qty.dimension[4] * $exp;
                dim[5] += $qty.dimension[5] * $exp;
                dim[6] += $qty.dimension[6] * $exp;
            )*
            dim
        }
        Quantity {
            dimension: build_dim(),
            name: $name,
        }
    }};
}
pub(crate) use quantity;
