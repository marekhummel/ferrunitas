/// Macro to define a prefix with its factor, symbol, and name
macro_rules! prefix {
    ($prefix_name:ident, $factor:expr, $symbol:expr) => {
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
        pub struct $prefix_name;

        impl $crate::sealed::Sealed for $prefix_name {}

        impl $crate::model::prefix::Prefix for $prefix_name {
            const FACTOR: f64 = $factor;
            const SYMBOL: &'static str = $symbol;
        }
    };
}

/// Macro to get quantity type by combining others
#[macro_export]
macro_rules! quantity {
    // Literal case
    ($quantity:ident: M $mass:ty, L $length:ty, T $time:ty, I $current:ty, Th $temperature:ty, N $amount:ty, J $luminosity:ty) => {
        quantity!(
            $quantity:
            $crate::model::dimension::DimensionVector<$mass, $length, $time, $current, $temperature, $amount, $luminosity>;
        );
    };

    // Compound case
    ($quantity:ident: [$(($dims:ty, $exps:ty)),+]) => {
        quantity!(
            $quantity:
            $crate::model::dimension::DimensionZero; $(($dims, $exps)),+
        );
    };

    // Recursive case
    ($quantity:ident: $dim_acc:ty; ($dim_vec:ty, $exp:ty) $(, ($dims:ty, $exps:ty))*) => {
        quantity!(
            $quantity:
            <$dim_acc as std::ops::Mul<
                <$dim_vec as $crate::model::dimension::TypePow<$exp>>::Output
            >>::Output;
            $(($dims, $exps)),*
        );
    };

    // Base case
    ($quantity:ident: $dim_acc:ty;) => {
        pub type $quantity =
            $crate::model::quantity::Quantity<$dim_acc>;
    };
}

/// Macro to define a new unit
#[macro_export]
macro_rules! unit {
    // Base units (prefixable or not)
    (base: $unit_name:ident, $abbrev:literal, $quantity:ty; $($optionals:tt)* ) => {
        unit!(base_internal: $unit_name, $abbrev, <$quantity as $crate::model::quantity::QuantityMarker>::DimensionVector, 1.0; $($optionals)*);
    };


    (base_internal: $unit_name:ident, $abbrev:literal, $dim_vec:ty, $factor:expr; prefixable, $($optionals:tt)*) => {
        unit!(base_internal: $unit_name, $abbrev, $dim_vec, $factor; $($optionals)*);

        impl $crate::model::prefix::Prefixable for $unit_name {}
    };

    (base_internal: $unit_name:ident, $abbrev:literal, $dim_vec:ty, $factor:expr; factor = $new_factor:expr, $($optionals:tt)*) => {
        unit!(base_internal: $unit_name, $abbrev, $dim_vec, $new_factor; $($optionals)*);
    };

    (base_internal: $unit_name:ident, $abbrev:literal, $dim_vec:ty, $factor:expr;) => {
        $crate::__unit!($unit_name, $dim_vec, $factor, $abbrev);
    };

    // Derived units based on other unit
    (derived: $unit_name:ident,  $abbrev:literal, ($factor:expr, $base_unit:ty); prefixable) => {
        unit!(derived: $unit_name, $abbrev, ($factor, $base_unit));

        impl $crate::model::prefix::Prefixable for $unit_name {}
    };

    (derived: $unit_name:ident, $abbrev:literal, ($factor:expr, $base_unit:ty)) => {
        $crate::__unit!(
            $unit_name,
            <$base_unit as $crate::model::unit::UnitBase>::DimensionVector,
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
            [
                $crate::model::dimension::DimensionZero,
                1.0;
                $($components),+
            ]
        );
    };

    // Prefixed unit
    (prefix: $alias:ident, $prefix:ty, $base_unit:ty) => {
        $crate::__unit!(
            $alias,
            <$base_unit as $crate::model::unit::UnitBase>::DimensionVector,
            <$prefix as $crate::model::prefix::Prefix>::FACTOR * <$base_unit as $crate::model::unit::UnitBase>::FACTOR,
            const_format::concatcp!(
                <$prefix as $crate::model::prefix::Prefix>::SYMBOL,
                <$base_unit as $crate::model::unit::UnitBase>::ABBREV
            )
        );
        // pub type $alias = $crate::model::unit::PrefixedUnit<$prefix, $base_unit>;

        // impl $crate::model::unit::UnitBase for $alias {
        //     type InternalQuantity = <$base_unit as $crate::model::unit::Unit>::Quantity;
        //     const FACTOR: f64 = <$prefix as $crate::model::prefix::Prefix>::FACTOR
        //         * <$base_unit as $crate::model::unit::UnitBase>::FACTOR;
        //     const ABBREV: &'static str = const_format::concatcp!(
        //         <$prefix as $crate::model::prefix::Prefix>::SYMBOL,
        //         <$base_unit as $crate::model::unit::UnitBase>::ABBREV
        //     );

        //     fn new(value: impl Into<f64>) -> Self {
        //         Self::new(value)
        //     }

        //     fn raw_value(&self) -> f64 {
        //         self.raw_value()
        //     }
        // }

        // impl $crate::model::unit::Unit for $alias {
        //     type Quantity = <$base_unit as $crate::model::unit::Unit>::Quantity;
        // }

        // $crate::__impl_arithmetics!(
        //     $alias,
        //     <$base_unit as $crate::model::unit::Unit>::Quantity
        // );
        // $crate::__impl_display!($alias);
    };
}

pub(crate) use prefix;
pub(crate) use quantity;
pub(crate) use unit;

/// Inner macros
pub(crate) mod __inner_unit_macros {
    /// Create a unit struct and impl Unit trait
    #[macro_export]
    #[doc(hidden)]
    macro_rules! __unit {
        ($unit_name:ident, $dim_vec:ty, $factor:expr, $abbrev:expr) => {
            #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
            pub struct $unit_name;

            impl $unit_name {
                pub const fn new_const(value: f64) -> $crate::model::quantity::Quantity<$dim_vec> {
                    $crate::model::quantity::Quantity::new_const(value * $factor)
                }
            }

            impl $crate::model::unit::UnitBase for $unit_name {
                type DimensionVector = $dim_vec;
                const FACTOR: f64 = $factor;
                const ABBREV: &'static str = $abbrev;
            }

            impl $crate::model::unit::Unit for $unit_name {
                type Quantity = $crate::model::quantity::Quantity<$dim_vec>;
            }

            $crate::__impl_display!($unit_name);
        };
    }
    /// Implement display trait for unit
    #[macro_export]
    #[doc(hidden)]
    macro_rules! __impl_display {
        ($unit:ty) => {
            impl std::fmt::Display for $unit {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(f, "{}", <$unit as $crate::model::unit::UnitBase>::ABBREV)
                }
            }
        };
    }
    /// Create a compound unit
    #[macro_export]
    #[doc(hidden)]
    macro_rules! __compound_unit {
        // Base case
        ($unit_name:ident, $abbrev:literal, [$dim_vec:ty, $factor_acc:expr;] ) => {
            $crate::__unit!(
                $unit_name,
                $dim_vec,
                $factor_acc,
                $abbrev
            );
        };

        // Recursive cases
        ($unit_name:ident, $abbrev:literal, [$dim_vec:ty, $factor_acc:expr; ($unit:ty, $exp:ty) $(, $components:tt)*] ) => {
            $crate::__compound_unit!(
                $unit_name,
                $abbrev,
                [$dim_vec, $factor_acc; (1.0, $unit, $exp) $(, $components)*]
            );
        };


        ($unit_name:ident, $abbrev:literal, [$dim_vec:ty, $factor_acc:expr; ($scalar:expr, $unit:ty, $exp:ty) $(, $components:tt)*] ) => {
            $crate::__compound_unit!(
                $unit_name,
                $abbrev,
                [
                    <$dim_vec as std::ops::Mul<
                        <<$unit as $crate::model::unit::UnitBase>::DimensionVector as $crate::model::dimension::TypePow<$exp>>::Output
                    >>::Output,
                    $factor_acc * $crate::model::macros::__inner_unit_macros::__powi_const(
                        ($scalar as f64) * <$unit as $crate::model::unit::UnitBase>::FACTOR, <$exp as typenum::ToInt<i32>>::INT
                    );
                    $($components),*
                ]
            );
        };

        ($unit_name:ident, $abbrev:literal, [$dim_vec:ty, $factor_acc:expr; (constant $constant:expr, $quantity:ty, $exp:ty) $(, $components:tt)*] ) => {
            $crate::__compound_unit!(
                $unit_name,
                $abbrev,
                [
                    <$dim_vec as std::ops::Mul<
                        <<$quantity as $crate::model::quantity::QuantityMarker>::DimensionVector as $crate::model::dimension::TypePow<$exp>>::Output
                    >>::Output,
                    $factor_acc * $crate::model::macros::__inner_unit_macros::__powi_const(
                        ($constant).value_const(), <$exp as typenum::ToInt<i32>>::INT
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
}
