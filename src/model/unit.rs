/// Trait for converting quantities back to unit values
pub trait FromQuantity<Q> {
    /// Convert a quantity back to this unit's value
    fn from_quantity(quantity: Q) -> Self;
}

/// Macro to define a base unit for a quantity type
#[macro_export]
macro_rules! base_unit {
    ($unit_name:ident, $quantity_type:ty) => {
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
        pub struct $unit_name(pub f64);

        impl From<$unit_name> for $quantity_type {
            fn from(unit: $unit_name) -> Self {
                <$quantity_type>::new(unit.0)
            }
        }

        impl $crate::model::unit::FromQuantity<$quantity_type> for $unit_name {
            fn from_quantity(quantity: $quantity_type) -> Self {
                Self(quantity.value()) // Base unit: no conversion needed
            }
        }
    };
}

/// Macro to define a derived unit in terms of a base unit with a conversion factor
#[macro_export]
macro_rules! derived_unit {
    ($unit_name:ident, $quantity_type:ty, $factor:expr) => {
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
        pub struct $unit_name(pub f64);

        impl From<$unit_name> for $quantity_type {
            fn from(unit: $unit_name) -> Self {
                <$quantity_type>::new(unit.0 * $factor)
            }
        }

        impl $crate::model::unit::FromQuantity<$quantity_type> for $unit_name {
            fn from_quantity(quantity: $quantity_type) -> Self {
                Self(quantity.value() / $factor) // Divide by factor to convert back
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::model::quantity::Quantity;
    use typenum::*;

    type TestMass = Quantity<P1, Z0, Z0, Z0, Z0, Z0, Z0>;

    base_unit!(TestGram, TestMass);
    derived_unit!(TestKilogram, TestMass, 1000.0);

    #[test]
    fn test_base_unit_conversion() {
        let mass: TestMass = TestGram(5.0).into();
        assert_eq!(mass.value(), 5.0);
    }

    #[test]
    fn test_derived_unit_conversion() {
        let mass: TestMass = TestKilogram(2.0).into();
        assert_eq!(mass.value(), 2000.0); // 2 kg = 2000 g
    }
}
