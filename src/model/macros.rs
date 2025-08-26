/// Macro to define a prefix with its factor, symbol, and name
#[macro_export]
macro_rules! prefix {
    ($prefix_name:ident, $factor:expr, $symbol:expr, $name:expr) => {
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
        pub struct $prefix_name;

        impl $crate::model::prefix::Prefix for $prefix_name {
            const FACTOR: f64 = $factor;
            const SYMBOL: &'static str = $symbol;
            const NAME: &'static str = $name;
        }
    };
}

#[macro_export]
macro_rules! quantity {
    // External case
    ([$(($quantities:ty, $exps:ty)),+]) => {
        $crate::quantity!(Quantity<Z0, Z0, Z0, Z0, Z0, Z0, Z0>; $(($quantities, $exps)),+)
    };

    // Recursive case
    ($quantity_acc:ty; ($quantity:ty, $exp:ty) $(, ($quantities:ty, $exps:ty))*) => {
        $crate::quantity!(
            <$quantity_acc as std::ops::Mul<
                <$quantity as $crate::model::quantity::TypePow<$exp>>::Output
            >>::Output;
            $(($quantities, $exps)),*
        )
    };

    // Base case
    ($quantity_acc:ty;) => {
        $quantity_acc
    };
}

#[macro_export]
macro_rules! format_quantity_dims {
    ($quantity:ty) => {{
        let items: [(&str, i8); 7] =
            [
                (
                    "M",
                    <<$quantity as $crate::model::quantity::QuantityMarker>::M as typenum::Integer>::to_i8(),
                ),
                (
                    "L",
                    <<$quantity as $crate::model::quantity::QuantityMarker>::L as typenum::Integer>::to_i8(),
                ),
                (
                    "T",
                    <<$quantity as $crate::model::quantity::QuantityMarker>::T as typenum::Integer>::to_i8(),
                ),
                (
                    "I",
                    <<$quantity as $crate::model::quantity::QuantityMarker>::I as typenum::Integer>::to_i8(),
                ),
                (
                    "Θ",
                    <<$quantity as $crate::model::quantity::QuantityMarker>::Theta as typenum::Integer>::to_i8(),
                ),
                (
                    "N",
                    <<$quantity as $crate::model::quantity::QuantityMarker>::N as typenum::Integer>::to_i8(),
                ),
                (
                    "J",
                    <<$quantity as $crate::model::quantity::QuantityMarker>::J as typenum::Integer>::to_i8(),
                ),
            ];
        let mut dim_string = String::new();
        for (dim, exp) in items {
            if exp == 0 {
                continue;
            }
            dim_string.push_str(&format!("{}^{}·", dim, exp));
        }
        if !dim_string.is_empty() {
            dim_string.pop();
        }

        dim_string
    }};
}

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
        $crate::model::macros::__inner_unit_macros::__unit!($unit_name, $quantity_type, $factor, $abbrev);
    };

    // Derived units based on other unit
    (derived: $unit_name:ident,  $abbrev:literal, ($factor:expr, $base_unit:ty); prefixable) => {
        unit!(derived: $unit_name, $abbrev, ($factor, $base_unit));

        impl $crate::model::prefix::Prefixable for $unit_name {}
    };

    (derived: $unit_name:ident, $abbrev:literal, ($factor:expr, $base_unit:ty)) => {
        $crate::model::macros::__inner_unit_macros::__unit!(
            $unit_name,
            <$base_unit as $crate::model::unit::Unit>::Quantity,
            ($factor as f64) * <$base_unit as $crate::model::unit::Unit>::FACTOR,
            $abbrev
        );
    };

    // Compound unit (not prefixable by default?)
    (compound: $unit_name:ident, $abbrev:literal, [$($components:tt),+]; prefixable) => {
        unit!(compound: $unit_name, $abbrev, [$($components),+]);

        impl $crate::model::prefix::Prefixable for $unit_name {}
    };

    (compound: $unit_name:ident, $abbrev:literal, [$($components:tt),+] ) => {
        $crate::model::macros::__inner_unit_macros::__compound_unit!(
            $unit_name,
            $abbrev,
            [Quantity<Z0, Z0, Z0, Z0, Z0, Z0, Z0>, 1.0; $($components),+]
        );
    };

    // Prefixed unit
    (prefix: $alias:ident, $prefix:ty, $base_unit:ty) => {
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

        $crate::model::macros::__inner_unit_macros::__impl_arithmetics!(
            $alias,
            <$base_unit as $crate::model::unit::Unit>::Quantity
        );
        $crate::model::macros::__inner_unit_macros::__impl_display!($alias);
    };
}

/// Inner macros
pub(crate) mod __inner_unit_macros {
    /// Create a unit struct and impl Unit trait
    macro_rules! __unit {
        ($unit_name:ident, $quantity_type:ty, $factor:expr, $abbrev:literal) => {
            #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
            pub struct $unit_name(pub(crate) f64);

            impl $crate::model::unit::Unit for $unit_name {
                type Quantity = $quantity_type;
                const FACTOR: f64 = $factor;
                const ABBREV: &'static str = $abbrev;

                fn new(value: f64) -> Self {
                    $unit_name(value)
                }

                fn raw_value(&self) -> f64 {
                    self.0
                }
            }

            $crate::model::macros::__inner_unit_macros::__impl_arithmetics!(
                $unit_name,
                $quantity_type
            );
            $crate::model::macros::__inner_unit_macros::__impl_display!($unit_name);
        };
    }

    // Implement arithmetic traits for unit
    macro_rules! __impl_arithmetics {
        ($unit:ty, $quantity:ty) => {
            $crate::model::macros::__inner_unit_macros::__impl_ops_within_unit!($unit, $quantity, Add, add, +);
            $crate::model::macros::__inner_unit_macros::__impl_ops_within_unit!($unit, $quantity, Sub, sub, -);

            $crate::model::macros::__inner_unit_macros::__impl_ops_to_quantity!($unit, $quantity, Mul, mul, *);
            $crate::model::macros::__inner_unit_macros::__impl_ops_to_quantity!($unit, $quantity, Div, div, /);

            $crate::model::macros::__inner_unit_macros::__impl_ops_scalar!($unit, $quantity);
        };
    }

    // Implement operations within the same unit (Add and Sub)
    macro_rules! __impl_ops_within_unit {
        ($unit:ty, $quantity:ty, $trait:ident, $method:ident, $op:tt) => {
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

    /// Implement operations to convert between unit and quantity (Mul and Div)
    macro_rules! __impl_ops_to_quantity {
        ($unit:ty, $quantity:ty, $trait:ident, $method:ident, $op:tt) => {
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

    /// Implement scalar operations (Mul / Div with f64)
    macro_rules! __impl_ops_scalar {
        ($unit:ty, $quantity:ty) => {
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

    /// Implement display trait for unit
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

    /// Create a compound unit
    macro_rules! __compound_unit {
        // Base case
        ($unit_name:ident, $abbrev:literal, [$quantity_acc:ty, $factor_acc:expr;] ) => {
            $crate::model::macros::__inner_unit_macros::__unit!(
                $unit_name,
                $quantity_acc,
                $factor_acc,
                $abbrev
            );
        };

        // Recursive cases
        ($unit_name:ident, $abbrev:literal, [$quantity_acc:ty, $factor_acc:expr; ($unit:ty, $exp:ty) $(, $components:tt)*] ) => {
            $crate::model::macros::__inner_unit_macros::__compound_unit!(
                $unit_name,
                $abbrev,
                [$quantity_acc, $factor_acc; (1.0, $unit, $exp) $(, $components)*]
            );
        };


        ($unit_name:ident, $abbrev:literal, [$quantity_acc:ty, $factor_acc:expr; ($scalar:expr, $unit:ty, $exp:ty) $(, $components:tt)*] ) => {
            $crate::model::macros::__inner_unit_macros::__compound_unit!(
                $unit_name,
                $abbrev,
                [
                    <$quantity_acc as std::ops::Mul<
                        <<$unit as $crate::model::unit::Unit>::Quantity as $crate::model::quantity::TypePow<$exp>>::Output
                    >>::Output,
                    $factor_acc * $crate::model::macros::__inner_unit_macros::powi_const(
                        ($scalar as f64) * <$unit as $crate::model::unit::Unit>::FACTOR, <$exp as typenum::ToInt<i32>>::INT
                    );
                    $($components),*
                ]
            );
        };
    }

    /// Const fn for integer exponentiation
    pub(crate) const fn powi_const(mut base: f64, mut exp: i32) -> f64 {
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

    pub(crate) use __impl_arithmetics;
    pub(crate) use __impl_display;
    pub(crate) use __impl_ops_scalar;
    pub(crate) use __impl_ops_to_quantity;
    pub(crate) use __impl_ops_within_unit;

    pub(crate) use __compound_unit;
    pub(crate) use __unit;
}
