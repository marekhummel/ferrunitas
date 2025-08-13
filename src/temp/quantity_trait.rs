use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, LazyLock, Mutex};

// Dimension encoding
pub const DIMENSIONS: usize = 7;
pub type Dimension = i16;
pub type DimensionVector = [Dimension; DIMENSIONS];

// Trait for quantities
pub trait QuantityTrait: fmt::Debug + Send + Sync {
    fn dimension_vector(&self) -> DimensionVector;
    fn name(&self) -> &'static str;
}

// Registry for quantities
struct Registry {
    map: Mutex<HashMap<DimensionVector, Arc<dyn QuantityTrait>>>,
}

impl Registry {
    fn new() -> Self {
        Registry {
            map: Mutex::new(HashMap::new()),
        }
    }
    fn get_or_insert<T: QuantityTrait + 'static>(
        &self,
        dv: DimensionVector,
        ctor: impl FnOnce() -> T,
    ) -> Arc<dyn QuantityTrait> {
        let mut map = self.map.lock().unwrap();
        if let Some(q) = map.get(&dv) {
            q.clone()
        } else {
            let q = Arc::new(ctor()) as Arc<dyn QuantityTrait>;
            map.insert(dv, q.clone());
            q
        }
    }
}

static QUANTITY_REGISTRY: LazyLock<Registry> = LazyLock::new(Registry::new);

// Macro to define a new quantity type and register it
#[macro_export]
macro_rules! define_quantity {
    (
        name: $name:expr,
        dim: [$($d:expr),* $(,)?]
    ) => {
        #[derive(Debug)]
        struct $name;
        impl $crate::quantity_trait::QuantityTrait for $name {
            fn dimension_vector(&self) -> $crate::quantity_trait::DimensionVector {
                [$($d),*]
            }
            fn name(&self) -> &'static str {
                $name
            }
        }
        $crate::quantity_trait::QUANTITY_REGISTRY.get_or_insert([$($d),*], || $name)
    };
}
