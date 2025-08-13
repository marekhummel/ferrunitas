// ================= core type =================

pub const DIMENSIONS: usize = 7;
pub type Dimension = i16;
pub type DimensionVector = [Dimension; DIMENSIONS];
pub type EncodedDimensionVector = i64; // cant use unsigned due to overflow problems

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Quantity<const EDV: EncodedDimensionVector>;

// Expose a type's encoded exponents as a single const
pub trait EncodedDimensions {
    const EDV: EncodedDimensionVector;
}
impl<const EDV: EncodedDimensionVector> EncodedDimensions for Quantity<EDV> {
    const EDV: EncodedDimensionVector = EDV;
}

// ---------- packing layout ----------
// Each dimension is stored in a lane within those 64 bits, with a bias to allow negative dimensionality.
pub mod enc {
    use super::{Dimension, DimensionVector, EncodedDimensionVector, DIMENSIONS};

    const BITS: Dimension = (EncodedDimensionVector::BITS as usize / DIMENSIONS) as Dimension;
    const MASK: EncodedDimensionVector = (1 << BITS) - 1;
    const BIAS: EncodedDimensionVector = 1 << (BITS - 1);

    #[inline(always)]
    pub const fn pack(e: DimensionVector) -> EncodedDimensionVector {
        let mut edv: EncodedDimensionVector = 0;
        let mut i = 0;
        while i < DIMENSIONS {
            let v = (e[i] as EncodedDimensionVector) + BIAS;
            edv |= (v & MASK) << (i as Dimension * BITS);
            i += 1;
        }
        edv
    }

    #[inline(always)]
    pub const fn unpack(code: EncodedDimensionVector) -> DimensionVector {
        let mut dv = [0; DIMENSIONS];
        let mut i = 0;
        while i < DIMENSIONS {
            let raw = (code >> (i as Dimension * BITS)) & MASK;
            dv[i] = (raw - BIAS) as Dimension;
            i += 1;
        }
        dv
    }

    #[inline(always)]
    pub const fn add(
        a: EncodedDimensionVector,
        b: EncodedDimensionVector,
    ) -> EncodedDimensionVector {
        let ea = unpack(a);
        let eb = unpack(b);
        let mut r = [0; DIMENSIONS];
        let mut i = 0;
        while i < DIMENSIONS {
            r[i] = ea[i] + eb[i];
            i += 1;
        }
        pack(r)
    }

    #[inline(always)]
    pub const fn scale(a: EncodedDimensionVector, k: Dimension) -> EncodedDimensionVector {
        let ea = unpack(a);
        let mut r = [0; DIMENSIONS];
        let mut i = 0;
        while i < DIMENSIONS {
            r[i] = ea[i] * k;
            i += 1;
        }
        pack(r)
    }

    pub const ZERO: EncodedDimensionVector = pack([0, 0, 0, 0, 0, 0, 0]);
}

// pub mod reg {
//     use super::EncodedDimensionVector;
//     use std::collections::HashMap;
//     use std::sync::LazyLock;
//     use std::sync::Mutex;

//     static QUANTITY_REGISTRY: LazyLock<Mutex<HashMap<EncodedDimensionVector, &'static str>>> =
//         LazyLock::new(|| Mutex::new(HashMap::new()));
// }

// ================= macros =================
#[macro_export]
macro_rules! quantity {
    // Dimension vector with named dimensions
    ( $name:expr, [ L$l:expr, M$m:expr, T$t:expr, I$i:expr, Th$th:expr, N$n:expr, J$j:expr ] ) => {
        $crate::quantity::Quantity::<{ $crate::quantity::enc::pack([$l, $m, $t, $i, $th, $n, $j]) }>
    };

    // List of (Quantity, Power) pairs
    ( $name:expr, [ $( ($ty:ty, $pow:expr) ),* $(,)? ] ) => {
        $crate::quantity::Quantity::<{
            let mut acc = $crate::quantity::enc::ZERO;
            $( acc = $crate::quantity::enc::add(acc,  $crate::quantity::enc::scale(<$ty as $crate::quantity::EncodedDimensions>::EDV, $pow)); )*
            acc
        }>
    };

    // Empty => dimensionless
    ( [] ) => { $crate::Quantity::<{ $crate::enc::ZERO }> };
}

// #[macro_export]
// macro_rules! quantity2 {
//     (name: $name:expr, dim: $dim:expr) => {{
//         let edv = $dim;
//         {
//             let mut reg = $crate::quantity::QUANTITY_REGISTRY.lock().unwrap();
//             reg.insert(edv, $name);
//         }
//         Quantity::<{ edv }>
//     }};
//     (dim: $dim:expr) => {{
//         let edv = $dim;
//         let name = {
//             let reg = $crate::quantity::QUANTITY_REGISTRY.lock().unwrap();
//             reg.get(&edv).copied()
//         };
//         // You can use the name for display, etc.
//         Quantity::<{ edv }>
//     }};
// }
