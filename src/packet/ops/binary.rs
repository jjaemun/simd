use std::ops::{Add, Sub, Mul, Div, Rem, BitAnd, BitOr, BitXor, Shl, Shr}; 
use std::simd::{Simd, SimdElement, LaneCount, SupportedLaneCount};

use crate::packet::Packet;


macro_rules! operations {
    (
        impl $trait:ident::$call:ident {
            $op: tt
        }
        $($rest:tt)*
    ) => {

        impl<T, const N: usize> $trait<Self> for Packet<T, N>
        where 
            T: SimdElement,
            LaneCount<N>: SupportedLaneCount,
            Simd<T, N>: $trait<Output = Simd<T, N>>,
        {
            type Output = Packet<T, N>;
        
            #[inline]
            fn $call(self, rhs: Packet<T, N>) -> Self::Output {
                Packet {
                    v: self.v $op rhs.v 
                }
            }
        }

        operations! {
            $($rest)*
        }

    };
    ($($done:tt)*) => { 
        // Done.
    }
}
       
operations! {
    impl Add::add { + }

    impl Sub::sub { - }

    impl Mul::mul { * }

    impl Div::div { / }

    impl Rem::rem { % }

    impl BitAnd::bitand { & }

    impl BitOr::bitor { | }

    impl BitXor::bitxor { ^ }

    impl Shl::shl { << }

    impl Shr::shr { >> }
}
