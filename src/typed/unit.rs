// ================= core type =================

use crate::quantity::{EncodedDimensionVector, EncodedDimensions, Quantity};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Unit<const EDV: EncodedDimensionVector> {
    pub scale: f64, // scale to base unit
    pub symbol: &'static str,
}

// Trait for units
pub trait EncodedUnit {
    const EDV: EncodedDimensionVector;
    fn scale(&self) -> f64;
    fn symbol(&self) -> &'static str;
}

impl<const EDV: EncodedDimensionVector> EncodedUnit for Unit<EDV> {
    const EDV: EncodedDimensionVector = EDV;
    fn scale(&self) -> f64 {
        self.scale
    }
    fn symbol(&self) -> &'static str {
        self.symbol
    }
}

// Macro to define a new unit type
#[macro_export]
macro_rules! unit {
    (
		$name:ident,
		quantity = $quantity:ty,
		scale = $scale:expr,
		symbol = $symbol:expr
	) => {
        pub type $name =
            $crate::unit::Unit<{ <$quantity as $crate::quantity::EncodedDimensions>::EDV }>;
        impl $name {
            pub const fn new() -> Self {
                Self {
                    scale: $scale,
                    symbol: $symbol,
                }
            }
        }
    };
}
