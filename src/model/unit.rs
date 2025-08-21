use std::{
    fmt::{Debug, Display},
    ops::{Add, Div, Mul, Sub},
};

use crate::model::{
    prefix::Prefix,
    quantity::{self, IntoUnit, Quantity, QuantityMarker},
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

/// Macro to define a derived unit in terms of a base unit with a conversion factor
#[macro_export]
macro_rules! unit {
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

        impl<U> std::ops::Add<U> for $unit_name
        where
            U: $crate::model::unit::Unit<Quantity = $quantity_type>,
        {
            type Output = $unit_name;

            fn add(self, rhs: U) -> Self::Output {
                Self::from_q(self.into_q() + rhs.into_q())
            }
        }

        impl<U> std::ops::Sub<U> for $unit_name
        where
            U: $crate::model::unit::Unit<Quantity = $quantity_type>,
        {
            type Output = $unit_name;

            fn sub(self, rhs: U) -> Self::Output {
                Self::from_q(self.into_q() - rhs.into_q())
            }
        }

        impl<U> std::ops::Mul<U> for $unit_name
        where
            U: $crate::model::unit::Unit,
            $quantity_type: std::ops::Mul<U::Quantity>,
        {
            type Output = <$quantity_type as std::ops::Mul<U::Quantity>>::Output;

            fn mul(self, rhs: U) -> Self::Output {
                self.into_q() * rhs.into_q()
            }
        }

        impl std::ops::Mul<f64> for $unit_name {
            type Output = $unit_name;

            fn mul(self, rhs: f64) -> Self::Output {
                $unit_name(self.0 * rhs)
            }
        }

        impl std::ops::Mul<$unit_name> for f64 {
            type Output = $unit_name;

            fn mul(self, rhs: $unit_name) -> Self::Output {
                $unit_name(self * rhs.0)
            }
        }

        impl<U> std::ops::Div<U> for $unit_name
        where
            U: $crate::model::unit::Unit,
            $quantity_type: std::ops::Div<U::Quantity>,
        {
            type Output = <$quantity_type as std::ops::Div<U::Quantity>>::Output;

            fn div(self, rhs: U) -> Self::Output {
                self.into_q() / rhs.into_q()
            }
        }

        impl std::ops::Div<f64> for $unit_name {
            type Output = $unit_name;

            fn div(self, rhs: f64) -> Self::Output {
                $unit_name(self.0 / rhs)
            }
        }

        impl std::ops::Div<$unit_name> for f64 {
            type Output = <$quantity_type as num_traits::Inv>::Output;

            fn div(self, rhs: $unit_name) -> Self::Output {
                let rhs_quantity: $quantity_type = rhs.into_q();
                self / rhs_quantity
            }
        }

        impl std::fmt::Display for $unit_name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                self.0.fmt(f)?;
                write!(f, " {}", <Self as $crate::model::unit::Unit>::ABBREV)?;
                Ok(())
            }
        }
    };

    ($unit_name:ident, $quantity_type:ty, $abbrev:literal) => {
        unit!($unit_name, $quantity_type, 1.0, $abbrev);
    };
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct PrefixedUnit<P: Prefix, U: Unit>(pub f64, pub(crate) std::marker::PhantomData<(P, U)>);

// impl<P, U> std::fmt::Display for PrefixedUnit<P, U>
// where
//     P: Prefix,
//     U: Unit,
// {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         std::fmt::Display::fmt(&self.0, f)?;
//         write!(f, " {}", <Self as Unit>::ABBREV)?;
//         Ok(())
//     }
// }

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
                $alias { 0: value, 1: std::marker::PhantomData }
            }

            fn raw_value(&self) -> f64 {
                self.0
            }

            fn convert<V: Unit<Quantity = Self::Quantity>>(self) -> V {
                let quantity: <$base_unit as $crate::model::unit::Unit>::Quantity = self.into();
                quantity.as_unit::<V>()
            }
        }

        impl From<<$base_unit as $crate::model::unit::Unit>::Quantity> for $alias {
            fn from(unit: <$base_unit as $crate::model::unit::Unit>::Quantity) -> Self {
                $alias::new(unit.value / <$alias as $crate::model::unit::Unit>::FACTOR)
            }
        }

        impl From<$alias> for <$base_unit as $crate::model::unit::Unit>::Quantity {
            fn from(unit: $alias) -> Self {
                <<$base_unit as $crate::model::unit::Unit>::Quantity as $crate::model::quantity::QuantityMarker>::new(unit.0 * <$alias as $crate::model::unit::Unit>::FACTOR)
            }
        }

        impl<U> std::ops::Add<U> for $alias
        where
            U: $crate::model::unit::Unit<Quantity = <$base_unit as $crate::model::unit::Unit>::Quantity>,
        {
            type Output = $alias;

            fn add(self, rhs: U) -> Self::Output {
                Self::from_q(self.into_q() + rhs.into_q())
            }
        }

        impl<U> std::ops::Sub<U> for $alias
        where
            U: $crate::model::unit::Unit<Quantity = <$base_unit as $crate::model::unit::Unit>::Quantity>,
        {
            type Output = $alias;

            fn sub(self, rhs: U) -> Self::Output {
                Self::from_q(self.into_q() - rhs.into_q())
            }
        }

        impl<U> std::ops::Mul<U> for $alias
        where
            U: $crate::model::unit::Unit,
            <$base_unit as $crate::model::unit::Unit>::Quantity: std::ops::Mul<U::Quantity>,
        {
            type Output = <<$base_unit as $crate::model::unit::Unit>::Quantity as std::ops::Mul<U::Quantity>>::Output;

            fn mul(self, rhs: U) -> Self::Output {
                self.into_q() * rhs.into_q()
            }
        }

        impl std::ops::Mul<f64> for $alias {
            type Output = $alias;

            fn mul(self, rhs: f64) -> Self::Output {
                $alias::new(self.0 * rhs)
            }
        }

        impl std::ops::Mul<$alias> for f64 {
            type Output = $alias;

            fn mul(self, rhs: $alias) -> Self::Output {
                $alias::new(self * rhs.0)
            }
        }

        impl<U> std::ops::Div<U> for $alias
        where
            U: $crate::model::unit::Unit,
            <$base_unit as $crate::model::unit::Unit>::Quantity: std::ops::Div<U::Quantity>,
        {
            type Output = <<$base_unit as $crate::model::unit::Unit>::Quantity as std::ops::Div<U::Quantity>>::Output;

            fn div(self, rhs: U) -> Self::Output {
                self.into_q() / rhs.into_q()
            }
        }

        impl std::ops::Div<f64> for $alias {
            type Output = $alias;

            fn div(self, rhs: f64) -> Self::Output {
                $alias::new(self.0 / rhs)
            }
        }

        impl std::ops::Div<$alias> for f64 {
            type Output = <<$base_unit as $crate::model::unit::Unit>::Quantity as num_traits::Inv>::Output;

            fn div(self, rhs: $alias) -> Self::Output {
                self / rhs.into_q()
            }
        }

        impl std::fmt::Display for $alias {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                self.0.fmt(f)?;
                write!(f, " {}", <Self as $crate::model::unit::Unit>::ABBREV)?;
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
