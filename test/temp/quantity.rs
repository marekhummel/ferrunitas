use std::collections::HashMap;
use std::ops::{Div, Mul};
use std::sync::{LazyLock, Mutex};

pub type DimensionVector = [i32; 7];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Quantity {
    pub dimension: DimensionVector, // L, M, T, I, Θ, N, J
    name: Option<&'static str>,
}

impl Quantity {
    pub fn new(dimension: DimensionVector, name: Option<&'static str>) -> Self {
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
        let local_name = name.or(reg_name);
        Quantity {
            dimension,
            name: local_name,
        }
    }

    pub fn combine(components: &[(Quantity, i32)], name: Option<&'static str>) -> Self {
        let mut new_dim = [0; 7];
        for (qty, exp) in components {
            new_dim.iter_mut().enumerate().for_each(|(i, val)| {
                *val += qty.dimension[i] * exp;
            });
        }
        Quantity::new(new_dim, name)
    }

    pub fn set_name(&mut self, name: &'static str) {
        let mut registry = QUANTITY_REGISTRY.lock().unwrap();
        let reg_name = registry.get_name(&self.dimension);
        if let Some(existing_name) = reg_name {
            panic!(
                "Quantity name already set: registry has '{}'",
                existing_name
            );
        }

        self.name = Some(name);
        registry.map.insert(self.dimension, name);
    }

    pub fn repr(&mut self) -> String {
        format!("{} [{:?}]", self.name().unwrap_or(""), self.dimension)
    }

    pub fn name(&mut self) -> Option<&'static str> {
        let registry = QUANTITY_REGISTRY.lock().unwrap();
        let reg_name = registry.get_name(&self.dimension);
        if self.name.is_none() && reg_name.is_some() {
            self.name = reg_name;
        }

        self.name
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

pub struct QuantityRegistry {
    pub map: HashMap<DimensionVector, &'static str>,
}

impl QuantityRegistry {
    pub fn new() -> Self {
        QuantityRegistry {
            map: HashMap::new(),
        }
    }

    pub fn get_name(&self, dim: &DimensionVector) -> Option<&'static str> {
        self.map.get(dim).copied()
    }
}

pub static QUANTITY_REGISTRY: LazyLock<Mutex<QuantityRegistry>> =
    LazyLock::new(|| Mutex::new(QuantityRegistry::new()));
