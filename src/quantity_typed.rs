#[derive(Debug, PartialEq, Eq)]
pub struct Quantity<const L: i32, const T: i32>;

pub trait Dimensions {
    const L: i32;
    const T: i32;
}

impl<const L: i32, const T: i32> Dimensions for Quantity<L, T> {
    const L: i32 = L;
    const T: i32 = T;
}

// ---------- Macro ----------
#[macro_export]
macro_rules! dim {
    ( [ $( ($ty:ty, $pow:expr) ),* $(,)? ] ) => {
        Quantity<
            { 0 $( + (<$ty as Dimensions>::L * $pow) )* },
            { 0 $( + (<$ty as Dimensions>::T * $pow) )* }
        >
    };
    ( [ $( $ty:ty ),+ $(,)? ] ) => {
        dim!([ $( ($ty, 1) ),+ ])
    };
    ( [] ) => { $crate::Dim<0, 0> };
}

// ---------- Examples ----------
pub type Length = Quantity<1, 0>;
pub type Time = Quantity<0, 1>;

pub type Area = dim!([(Length, 2)]); // Dim<2, 0>
pub type Velocity = dim!([(Length, 1), (Time, -1)]); // Dim<1, -1>
pub type Scalar = dim!([]); // Dim<0, 0>

#[macro_export]
macro_rules! assert_same_dims {
    ($a:ty, $b:ty) => {
        const _: () = {
            let _ = [(); 0 - !(<$a as Dimensions>::L == <$b as Dimensions>::L) as usize];
            let _ = [(); 0 - !(<$a as Dimensions>::T == <$b as Dimensions>::T) as usize];
        };
    };
}

pub fn test() {
    type Acc1 = dim!([(Velocity, 1), (Time, -1)]);
    type Acc2 = Quantity<1, -2>;

    assert_same_dims!(Acc1, Acc2); // compiles
}
