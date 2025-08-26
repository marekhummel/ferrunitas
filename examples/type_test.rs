use typenum::*;

trait T {
    const F: f64;
}

#[macro_export]
macro_rules! unit {
    // Compound unit
    (compound: $unit_name:ident, $abbrev:literal, [$($components:tt),+]; prefixable) => {
        unit!(compound: $unit_name, $abbrev, [$($components),+]);

        impl ferrunitas::model::prefix::Prefixable for $unit_name {}
    };

    (compound: $unit_name:ident, $abbrev:literal, [$($components:tt),+] ) => {
        __compound_unit!(
            $unit_name,
            $abbrev,
            [Quantity<Z0, Z0, Z0, Z0, Z0, Z0, Z0>, 1.0; $($components),+]
        );
    };
}

/// Create a compound unit
#[macro_export]
macro_rules! __compound_unit {
    // Base case
    ($unit_name:ident, $abbrev:literal, [$quantity_acc:ty, $factor_acc:expr;] ) => {
        #[derive(Debug)]
        struct $unit_name;

        impl T for $unit_name {
            const F: f64 = $factor_acc;
        }
        // __unit!(
        //     $unit_name,
        //     $quantity_acc,
        //     $factor_acc,
        //     $abbrev
        // );
    };

    ($unit_name:ident, $abbrev:literal, [$quantity_acc:ty, $factor_acc:expr; ($unit:ty, $exp:ty) $(, $components:tt)*] ) => {
        __compound_unit!($unit_name, $abbrev, [$quantity_acc, $factor_acc; (1.0, $unit, $exp) $(, $components)*] );
    };

    // Recursive case
    ($unit_name:ident, $abbrev:literal, [$quantity_acc:ty, $factor_acc:expr; ($factor:expr, $unit:ty, $exp:ty) $(, $components:tt)*] ) => {
        __compound_unit!(
            $unit_name,
            $abbrev,
            [
                $quantity_acc,
                $factor_acc * powi_const(
                    $factor * <$unit as T>::F, <$exp as typenum::ToInt<i32>>::INT
                );
                $($components),*
            ]
        );
    };
}

const fn powi_const(mut base: f64, mut exp: i32) -> f64 {
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

fn main() {
    #[derive(Debug)]
    struct MyUnit1;
    #[derive(Debug)]
    struct MyUnit2;

    impl T for MyUnit1 {
        const F: f64 = 2.0;
    }

    impl T for MyUnit2 {
        const F: f64 = 3.0;
    }

    unit!(compound: MyCompoundUnit, "MCU", [(MyUnit1, P1), (MyUnit2, P2)]);
    unit!(compound: MyCompoundUnit2, "MCU2", [(2.0, MyUnit1, P1), (MyUnit2, P2)]; prefixable);

    println!("a = {:?}", MyUnit1::F);
    println!("b = {:?}", MyUnit2::F);
    println!("x = {:?}", MyCompoundUnit::F);
    println!("y = {:?}", MyCompoundUnit2::F);
}
