use std::{
    fmt::{Debug, Display},
    ops::{Add, Div, Mul, Sub},
};

use crate::model::{
    prefix::Prefix,
    quantity::{IntoUnit, QuantityMarker},
};

pub trait Unit:
    Debug
    + Display
    + Clone
    + Copy
    + PartialEq
    + PartialOrd
    + Sized
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<f64, Output = Self>
    + Div<f64, Output = Self>
{
    type Quantity: QuantityMarker + IntoUnit;
    const FACTOR: f64;
    const ABBREV: &'static str;

    fn new(value: f64) -> Self;
    fn raw_value(&self) -> f64;

    fn from_q(q: Self::Quantity) -> Self {
        Self::new(q.raw_value() / Self::FACTOR)
    }
    fn into_q(self) -> Self::Quantity {
        Self::Quantity::new(self.raw_value() * Self::FACTOR)
    }
    fn to_q(&self) -> Self::Quantity {
        (*self).into_q()
    }

    fn convert<U: Unit<Quantity = Self::Quantity>>(self) -> U {
        self.to_q().to_unit::<U>()
    }
}

pub trait Prefixable {}

/// Macro to define a derived unit in terms of a base unit with a conversion factor
#[macro_export]
macro_rules! unit {
    ($unit_name:ident, $quantity_type:ty, $factor:expr, $abbrev:literal, prefixable) => {
        unit!($unit_name, $quantity_type, $factor, $abbrev);

        impl $crate::model::unit::Prefixable for $unit_name {}
    };

    ($unit_name:ident, $quantity_type:ty, $factor:expr, $abbrev:literal) => {
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
        pub struct $unit_name(pub f64);

        impl $crate::model::unit::Unit for $unit_name {
            type Quantity = $quantity_type;
            const FACTOR: f64 = $factor;
            const ABBREV: &'static str = $abbrev;

            fn new(value: f64) -> Self {
                $unit_name(value / $factor)
            }

            fn raw_value(&self) -> f64 {
                self.0
            }
        }

        $crate::__impl_arithmetics!($unit_name, $quantity_type);
        $crate::__impl_display!($unit_name);
    };

    ($unit_name:ident, $quantity_type:ty, $abbrev:literal, prefixable) => {
        unit!($unit_name, $quantity_type, 1.0, $abbrev, prefixable);
    };

    ($unit_name:ident, $quantity_type:ty, $abbrev:literal) => {
        unit!($unit_name, $quantity_type, 1.0, $abbrev);
    };
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct PrefixedUnit<P: Prefix, U: Unit + Prefixable>(
    pub f64,
    pub(crate) std::marker::PhantomData<(P, U)>,
);

// ============================================================================
// Macro for creating prefixed unit type aliases with conversions
// ============================================================================

// Macro to create a type alias for a prefixed unit (conversions implemented generically)
#[macro_export]
macro_rules! prefixed_unit {
    ($alias:ident, $prefix:ty, $base_unit:ty) => {
        pub type $alias = $crate::model::unit::PrefixedUnit<$prefix, $base_unit>;

        impl $crate::model::unit::Unit for $alias {
            type Quantity = <$base_unit as $crate::model::unit::Unit>::Quantity;
            const FACTOR: f64 = <$prefix as $crate::model::prefix::Prefix>::FACTOR
                * <$base_unit as $crate::model::unit::Unit>::FACTOR;
            const ABBREV: &'static str = const_format::concatcp!(
                <$prefix as $crate::model::prefix::Prefix>::SYMBOL,
                <$base_unit as $crate::model::unit::Unit>::ABBREV
            );

            fn new(value: f64) -> Self {
                $alias {
                    0: value,
                    1: std::marker::PhantomData,
                }
            }

            fn raw_value(&self) -> f64 {
                self.0
            }
        }

        $crate::__impl_arithmetics!($alias, <$base_unit as $crate::model::unit::Unit>::Quantity);
        $crate::__impl_display!($alias);
    };
}

#[macro_export]
macro_rules! __impl_arithmetics {
    ($unit:ty, $quantity:ty) => {
        macro_rules! __impl_ops_within_unit {
            ($trait:ident, $method:ident, $op:tt) => {
                impl<U> std::ops::$trait<U> for $unit
                where
                    U: $crate::model::unit::Unit<Quantity = $quantity>,
                {
                    type Output = $unit;

                    fn $method(self, rhs: U) -> Self::Output {
                        Self::from_q(self.into_q() $op rhs.into_q())
                    }
                }
            };
        }

        __impl_ops_within_unit!(Add, add, +);
        __impl_ops_within_unit!(Sub, sub, -);

        macro_rules! __impl_ops_to_quantity {
            ($trait:ident, $method:ident, $op:tt) => {
                impl<U> std::ops::$trait<U> for $unit
                where
                    U: $crate::model::unit::Unit,
                    $quantity: std::ops::$trait<U::Quantity>,
                {
                    type Output = <$quantity as std::ops::$trait<U::Quantity>>::Output;

                    fn $method(self, rhs: U) -> Self::Output {
                        self.into_q() $op rhs.into_q()
                    }
                }
            };
        }

        __impl_ops_to_quantity!(Mul, mul, *);
        __impl_ops_to_quantity!(Div, div, /);

        macro_rules! __impl_ops_scalar {
            () => {
                impl std::ops::Mul<f64> for $unit {
                    type Output = $unit;

                    fn mul(self, rhs: f64) -> Self::Output {
                        <$unit>::new(self.0 * rhs)
                    }
                }

                impl std::ops::Mul<$unit> for f64 {
                    type Output = $unit;

                    fn mul(self, rhs: $unit) -> Self::Output {
                        <$unit>::new(self * rhs.0)
                    }
                }

                impl std::ops::Div<f64> for $unit {
                    type Output = $unit;

                    fn div(self, rhs: f64) -> Self::Output {
                        <$unit>::new(self.0 / rhs)
                    }
                }

                impl std::ops::Div<$unit> for f64 {
                    type Output = <$quantity as num_traits::Inv>::Output;

                    fn div(self, rhs: $unit) -> Self::Output {
                        let rhs_quantity: $quantity = rhs.into_q();
                        self / rhs_quantity
                    }
                }
            };
        }

        __impl_ops_scalar!();
    };
}

#[macro_export]
macro_rules! __impl_display {
    ($unit:ty) => {
        impl std::fmt::Display for $unit {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                self.0.fmt(f)?;
                write!(f, " {}", <$unit>::ABBREV)?;
                Ok(())
            }
        }
    };
}

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
