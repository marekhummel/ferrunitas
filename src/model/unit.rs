use std::fmt::Debug;

use crate::model::{prefix::Prefix, quantity::QuantityMarker};

pub trait Unit:
    Debug + Clone + Copy + PartialEq + PartialOrd + Into<Self::Quantity> + From<Self::Quantity>
{
    type Quantity: QuantityMarker;
    const FACTOR: f64;
    const ABBREV: &'static str;
}

/// Macro to define a derived unit in terms of a base unit with a conversion factor
#[macro_export]
macro_rules! unit {
    ($unit_name:ident, $quantity_type:ty, $factor:expr, $abbrev:literal) => {
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
        pub struct $unit_name(pub f64);

        impl From<$unit_name> for $quantity_type {
            fn from(unit: $unit_name) -> Self {
                <$quantity_type as $crate::model::quantity::QuantityMarker>::new(unit.0 * $factor)
            }
        }

        impl From<$quantity_type> for $unit_name {
            fn from(quantity: $quantity_type) -> Self {
                Self(quantity.value / $factor) // Divide by factor to convert back
            }
        }

        impl $crate::model::unit::Unit for $unit_name {
            type Quantity = $quantity_type;
            const FACTOR: f64 = $factor;
            const ABBREV: &'static str = $abbrev;
        }

        impl std::fmt::Display for $unit_name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(
                    f,
                    "{} {}",
                    self.0,
                    <Self as $crate::model::unit::Unit>::ABBREV
                )
            }
        }
    };

    ($unit_name:ident, $quantity_type:ty, $abbrev:literal) => {
        unit!($unit_name, $quantity_type, 1.0, $abbrev);
    };
}

// #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
// pub struct PrefixedUnit<P: Prefix, U: Unit>(pub f64, std::marker::PhantomData<(P, U)>);

// impl<P: Prefix, U: Unit> PrefixedUnit<P, U> {
//     /// Create a new prefixed unit with the given value
//     pub fn new(value: f64) -> Self {
//         Self(value, std::marker::PhantomData)
//     }
// }

// impl<P: Prefix, U: Unit> From<PrefixedUnit<P, U>> for U::Quantity {
//     fn from(unit: PrefixedUnit<P, U>) -> Self {
//         <U::Quantity as QuantityMarker>::new(unit.0 * P::FACTOR * U::FACTOR)
//     }
// }

// ============================================================================
// Macro for creating prefixed unit type aliases with conversions
// ============================================================================

// Macro to create a type alias for a prefixed unit (conversions implemented generically)
// #[macro_export]
// macro_rules! prefixed_unit {
//     ($alias:ident, $prefix:ty, $base_unit:ty) => {
//         pub type $alias = $crate::model::unit::PrefixedUnit<$prefix, $base_unit>;

//         // Allow construction with function call syntax like Kilogram(5.0)
//         pub fn $alias(value: f64) -> $alias {
//             <$alias>::new(value)
//         }

//         impl std::fmt::Display for $alias {
//             fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//                 write!(
//                     f,
//                     "{} {}{}",
//                     self.0,
//                     <$prefix as $crate::model::prefix::Prefix>::SYMBOL,
//                     <$base_unit as $crate::model::unit::Unit>::ABBREV
//                 )
//             }
//         }
//     };
// }

// Implement conversions for specific quantity types
// #[macro_export]
// macro_rules! impl_prefix_conversions {
//     ($quantity_type:ty) => {
//         impl<P: $crate::model::prefix::Prefix, U> From<$crate::model::unit::PrefixedUnit<P, U>>
//             for $quantity_type
//         {
//             fn from(unit: $crate::model::unit::PrefixedUnit<P, U>) -> Self {
//                 <$quantity_type>::new(unit.0 * P::FACTOR)
//             }
//         }

//         impl<P: $crate::model::prefix::Prefix, U> From<$quantity_type>
//             for $crate::model::unit::PrefixedUnit<P, U>
//         {
//             fn from(quantity: $quantity_type) -> Self {
//                 Self::new(quantity.value / P::FACTOR)
//             }
//         }
//     };
// }

// #[cfg(test)]
// mod tests {
//     use crate::model::quantity::Quantity;
//     use typenum::*;

//     type TestMass = Quantity<P1, Z0, Z0, Z0, Z0, Z0, Z0>;

//     unit!(TestGram, TestMass, "tg");
//     unit!(TestKilogram, TestMass, 1000.0, "tkg");

//     #[test]
//     fn test_base_unit_conversion() {
//         let mass: TestMass = TestGram(5.0).into();
//         assert_eq!(mass.value, 5.0);
//     }

//     #[test]
//     fn test_derived_unit_conversion() {
//         let mass: TestMass = TestKilogram(2.0).into();
//         assert_eq!(mass.value, 2000.0); // 2 kg = 2000 g
//     }
// }
