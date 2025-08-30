/// Macro to define a prefix with its factor, symbol, and name
macro_rules! prefix {
    ($prefix_name:ident, $factor:expr, $symbol:expr) => {
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
        pub struct $prefix_name;

        impl $crate::common::Sealed for $prefix_name {}

        impl $crate::model::prefix::Prefix for $prefix_name {
            const FACTOR: f64 = $factor;
            const SYMBOL: &'static str = $symbol;
        }
    };
}

// Note: Currently irrelevant
// /// Macro to get quantity type by combining others
// macro_rules! quantity {
//     // External case
//     ([$(($quantities:ty, $exps:ty)),+]) => {
//         $crate::quantity!(Quantity<Z0, Z0, Z0, Z0, Z0, Z0, Z0>; $(($quantities, $exps)),+)
//     };

//     // Recursive case
//     ($quantity_acc:ty; ($quantity:ty, $exp:ty) $(, ($quantities:ty, $exps:ty))*) => {
//         $crate::quantity!(
//             <$quantity_acc as std::ops::Mul<
//                 <$quantity as $crate::model::quantity::TypePow<$exp>>::Output
//             >>::Output;
//             $(($quantities, $exps)),*
//         )
//     };

//     // Base case
//     ($quantity_acc:ty;) => {
//         $quantity_acc
//     };
// }

/// Macro to define a new unit
#[macro_export]
macro_rules! unit {
    // Base units (prefixable or not)
    (base: $unit_name:ident, $abbrev:literal, $quantity_type:ty; $($optionals:tt)* ) => {
        unit!(base_internal: $unit_name, $abbrev, $quantity_type, 1.0; $($optionals)*);
    };


    (base_internal: $unit_name:ident, $abbrev:literal, $quantity_type:ty, $factor:expr; prefixable, $($optionals:tt)*) => {
        unit!(base_internal: $unit_name, $abbrev, $quantity_type, $factor; $($optionals)*);

        impl $crate::model::prefix::Prefixable for $unit_name {}
    };

    (base_internal: $unit_name:ident, $abbrev:literal, $quantity_type:ty, $factor:expr; factor = $new_factor:expr, $($optionals:tt)*) => {
        unit!(base_internal: $unit_name, $abbrev, $quantity_type, $new_factor; $($optionals)*);
    };

    (base_internal: $unit_name:ident, $abbrev:literal, $quantity_type:ty, $factor:expr;) => {
        $crate::__unit!($unit_name, $quantity_type, $factor, $abbrev);
    };

    // Derived units based on other unit
    (derived: $unit_name:ident,  $abbrev:literal, ($factor:expr, $base_unit:ty); prefixable) => {
        unit!(derived: $unit_name, $abbrev, ($factor, $base_unit));

        impl $crate::model::prefix::Prefixable for $unit_name {}
    };

    (derived: $unit_name:ident, $abbrev:literal, ($factor:expr, $base_unit:ty)) => {
        $crate::__unit!(
            $unit_name,
            <$base_unit as $crate::model::unit::Unit>::Quantity,
            ($factor as f64) * <$base_unit as $crate::model::unit::UnitBase>::FACTOR,
            $abbrev
        );
    };

    // Compound unit (not prefixable by default?)
    (compound: $unit_name:ident, $abbrev:literal, [$($components:tt),+]; prefixable) => {
        unit!(compound: $unit_name, $abbrev, [$($components),+]);

        impl $crate::model::prefix::Prefixable for $unit_name {}
    };

    (compound: $unit_name:ident, $abbrev:literal, [$($components:tt),+] ) => {
        $crate::__compound_unit!(
            $unit_name,
            $abbrev,
            [Quantity<Z0, Z0, Z0, Z0, Z0, Z0, Z0>, 1.0; $($components),+]
        );
    };

    // Prefixed unit
    (prefix: $alias:ident, $prefix:ty, $base_unit:ty) => {
        pub type $alias = $crate::model::unit::PrefixedUnit<$prefix, $base_unit>;

        impl $crate::model::unit::UnitBase for $alias {
            type InternalQuantity = <$base_unit as $crate::model::unit::Unit>::Quantity;
            const FACTOR: f64 = <$prefix as $crate::model::prefix::Prefix>::FACTOR
                * <$base_unit as $crate::model::unit::UnitBase>::FACTOR;
            const ABBREV: &'static str = const_format::concatcp!(
                <$prefix as $crate::model::prefix::Prefix>::SYMBOL,
                <$base_unit as $crate::model::unit::UnitBase>::ABBREV
            );

            fn new(value: impl Into<f64>) -> Self {
                Self::new(value)
            }

            fn raw_value(&self) -> f64 {
                self.raw_value()
            }
        }

        impl $crate::model::unit::Unit for $alias {
            type Quantity = <$base_unit as $crate::model::unit::Unit>::Quantity;
        }

        $crate::__impl_arithmetics!(
            $alias,
            <$base_unit as $crate::model::unit::Unit>::Quantity
        );
        $crate::__impl_display!($alias);
    };
}

pub(crate) use prefix;
// pub(crate) use quantity;
pub(crate) use unit;

/// Inner macros
pub(crate) mod __inner_unit_macros {
    /// Create a unit struct and impl Unit trait
    #[macro_export]
    #[doc(hidden)]
    macro_rules! __unit {
        ($unit_name:ident, $quantity_type:ty, $factor:expr, $abbrev:literal) => {
            #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
            pub struct $unit_name(pub(crate) f64);

            impl $crate::common::Sealed for $unit_name {}

            impl $crate::model::unit::UnitBase for $unit_name {
                type InternalQuantity = $quantity_type;
                const FACTOR: f64 = $factor;
                const ABBREV: &'static str = $abbrev;

                fn new(value: impl Into<f64>) -> Self {
                    $unit_name(value.into())
                }

                fn raw_value(&self) -> f64 {
                    self.0
                }
            }

            impl $crate::model::unit::Unit for $unit_name {
                type Quantity = $quantity_type;
            }

            // impl $crate::model::quantity::ToQuantity for $unit_name {
            //     type BaseQuantity = $quantity_type;

            //     fn to_base(&self) -> f64 {
            //         $crate::model::unit::Unit::into_q(*self).raw_value()
            //     }
            // }

            $crate::__impl_arithmetics!($unit_name, $quantity_type);
            $crate::__impl_display!($unit_name);
        };
    }

    /// Implement arithmetic traits for unit
    #[macro_export]
    #[doc(hidden)]
    macro_rules! __impl_arithmetics {
        ($unit:ty, $quantity:ty) => {
            $crate::__impl_ops_within_unit!($unit, $quantity, Add, add, +);
            $crate::__impl_ops_within_unit!($unit, $quantity, Sub, sub, -);

            $crate::__impl_ops_to_quantity!($unit, $quantity, Mul, mul, *);
            $crate::__impl_ops_to_quantity!($unit, $quantity, Div, div, /);

            $crate::__impl_ops_scalar!($unit, $quantity);
        };
    }

    /// Implement operations within the same unit (Add, AddAssign and Sub, SubAssign)
    #[macro_export]
    #[doc(hidden)]
    macro_rules! __impl_ops_within_unit {
        ($unit:ty, $quantity:ty, $trait:ident, $method:ident, $op:tt) => {
            impl<U> std::ops::$trait<U> for $unit
            where
                U: $crate::model::quantity::ToQuantity<BaseQuantity = $quantity>,
                Self: $crate::model::quantity::ToQuantity<BaseQuantity = $quantity>,
            {
                type Output = $unit;

                fn $method(self, rhs: U) -> Self::Output {
                    <$unit as $crate::model::quantity::ToQuantity>::internal_from_q(
                        $crate::model::quantity::ToQuantity::internal_into_q(self)
                        $op
                        $crate::model::quantity::ToQuantity::internal_into_q(rhs)
                    )
                }
            }

            paste::paste! {
                impl<U> std::ops::[<$trait Assign>]<U> for $unit
                where
                    U: $crate::model::quantity::ToQuantity<BaseQuantity = $quantity>,
                    Self: $crate::model::quantity::ToQuantity<BaseQuantity = $quantity>,
                {
                    fn [<$method _assign>](&mut self, rhs: U) {
                        // self.0 $assign_op rhs.convert::<Self>().0;
                        *self = *self $op
                            <Self as $crate::model::quantity::ToQuantity>::internal_from_q(
                                $crate::model::quantity::ToQuantity::internal_into_q(rhs)
                            );
                    }
                }
            }

            // impl<D> std::ops::$trait<D> for $unit
            // where
            //     <$unit as $crate::model::unit::Unit>::Quantity: std::ops::$trait<$quantity>,
            //     D: $crate::model::quantity::Dimensioned<M = <$quantity as $crate::model::quantity::Dimensioned>::M>,
            // {
            //     type Output = Self;

            //     fn $method(self, rhs: $quantity) -> Self::Output {
            //         $crate::model::unit::Unit::from_q(
            //             $crate::model::unit::Unit::into_q(self) $op rhs
            //         )
            //     }
            // }

            // paste::paste! {
            //     impl std::ops::[<$trait Assign>]<$quantity> for $unit
            //     where
            //         <$unit as $crate::model::unit::Unit>::Quantity: std::ops::$trait<$quantity>,
            //     {
            //         fn [<$method _assign>](&mut self, rhs: $quantity) {
            //             // self.0 $assign_op rhs.convert::<Self>().0;
            //             *self = *self $op rhs;
            //         }
            //     }
            // }
        };
    }

    /// Implement operations to convert between unit and quantity (Mul and Div)
    #[macro_export]
    #[doc(hidden)]
    macro_rules! __impl_ops_to_quantity {
        ($unit:ty, $quantity:ty, $trait:ident, $method:ident, $op:tt) => {
            impl<U> std::ops::$trait<U> for $unit
            where
                U: $crate::model::quantity::ToQuantity,
                $quantity: std::ops::$trait<U::BaseQuantity>,
            {
                type Output = <$quantity as std::ops::$trait<U::BaseQuantity>>::Output;

                fn $method(self, rhs: U) -> Self::Output {
                    $crate::model::quantity::ToQuantity::internal_into_q(self) $op $crate::model::quantity::ToQuantity::internal_into_q(rhs)
                }
            }

            // impl<M, L, T, I, Th, N, J> std::ops::$trait<$crate::model::quantity::Quantity<M, L, T, I, Th, N, J>> for $unit
            // where
            //     <$unit as $crate::model::unit::Unit>::Quantity: std::ops::$trait<$crate::model::quantity::Quantity<M, L, T, I, Th, N, J>>,
            // {
            //     type Output = <<$unit as $crate::model::unit::Unit>::Quantity as std::ops::$trait<$crate::model::quantity::Quantity<M, L, T, I, Th, N, J>>>::Output;

            //     fn $method(self, rhs: $crate::model::quantity::Quantity<M, L, T, I, Th, N, J>) -> Self::Output {
            //         $crate::model::unit::Unit::into_q(self) $op rhs
            //     }
            // }
        };
    }

    /// Implement scalar operations (Mul / Div with f64)
    #[macro_export]
    #[doc(hidden)]
    macro_rules! __impl_ops_scalar {
        ($unit:ty, $quantity:ty) => {
            impl std::ops::Mul<f64> for $unit {
                type Output = $unit;

                fn mul(self, rhs: f64) -> Self::Output {
                    <$unit as $crate::model::unit::UnitBase>::new(self.0 * rhs)
                }
            }

            impl std::ops::Mul<$unit> for f64 {
                type Output = $unit;

                fn mul(self, rhs: $unit) -> Self::Output {
                    <$unit as $crate::model::unit::UnitBase>::new(self * rhs.0)
                }
            }

            impl std::ops::MulAssign<f64> for $unit {
                fn mul_assign(&mut self, scalar: f64) {
                    self.0 *= scalar;
                }
            }

            impl std::ops::Div<f64> for $unit {
                type Output = $unit;

                fn div(self, rhs: f64) -> Self::Output {
                    <$unit as $crate::model::unit::UnitBase>::new(self.0 / rhs)
                }
            }

            impl std::ops::Div<$unit> for f64 {
                type Output =
                    <Quantity<Z0, Z0, Z0, Z0, Z0, Z0, Z0> as std::ops::Div<$quantity>>::Output;

                fn div(self, rhs: $unit) -> Self::Output {
                    let rhs_quantity: $quantity =
                        <$unit as $crate::model::quantity::ToQuantity>::internal_into_q(rhs);
                    self / rhs_quantity
                }
            }

            impl std::ops::DivAssign<f64> for $unit {
                fn div_assign(&mut self, scalar: f64) {
                    self.0 /= scalar;
                }
            }
        };
    }

    /// Implement display trait for unit
    #[macro_export]
    #[doc(hidden)]
    macro_rules! __impl_display {
        ($unit:ty) => {
            impl std::fmt::Display for $unit {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    self.0.fmt(f)?;
                    write!(f, " {}", <$unit as $crate::model::unit::UnitBase>::ABBREV)?;
                    Ok(())
                }
            }
        };
    }

    /// Create a compound unit
    #[macro_export]
    #[doc(hidden)]
    macro_rules! __compound_unit {
        // Base case
        ($unit_name:ident, $abbrev:literal, [$quantity_acc:ty, $factor_acc:expr;] ) => {
            $crate::__unit!(
                $unit_name,
                $quantity_acc,
                $factor_acc,
                $abbrev
            );
        };

        // Recursive cases
        ($unit_name:ident, $abbrev:literal, [$quantity_acc:ty, $factor_acc:expr; ($unit:ty, $exp:ty) $(, $components:tt)*] ) => {
            $crate::__compound_unit!(
                $unit_name,
                $abbrev,
                [$quantity_acc, $factor_acc; (1.0, $unit, $exp) $(, $components)*]
            );
        };


        ($unit_name:ident, $abbrev:literal, [$quantity_acc:ty, $factor_acc:expr; ($scalar:expr, $unit:ty, $exp:ty) $(, $components:tt)*] ) => {
            $crate::__compound_unit!(
                $unit_name,
                $abbrev,
                [
                    <$quantity_acc as std::ops::Mul<
                        <<$unit as $crate::model::unit::Unit>::Quantity as $crate::model::quantity::TypePow<$exp>>::Output
                    >>::Output,
                    $factor_acc * $crate::model::macros::__inner_unit_macros::__powi_const(
                        ($scalar as f64) * <$unit as $crate::model::unit::UnitBase>::FACTOR, <$exp as typenum::ToInt<i32>>::INT
                    );
                    $($components),*
                ]
            );
        };
    }

    /// Const fn for integer exponentiation
    #[doc(hidden)]
    pub const fn __powi_const(mut base: f64, mut exp: i32) -> f64 {
        if exp == 0 {
            return 1.0;
        }
        let neg = exp < 0;
        if neg {
            exp = -exp;
        }
        let mut e = exp as u32;
        let mut acc = 1.0;
        while e != 0 {
            if (e & 1) == 1 {
                acc *= base;
            }
            base *= base;
            e >>= 1;
        }
        if neg {
            1.0 / acc
        } else {
            acc
        }
    }

    // -----------------------------------

    // pub(crate) use __compound_unit;
    // pub(crate) use __unit;

    // pub(crate) use __impl_arithmetics;
    // pub(crate) use __impl_display;
    // pub(crate) use __impl_ops_scalar;
    // pub(crate) use __impl_ops_to_quantity;
    // pub(crate) use __impl_ops_within_unit;
}
