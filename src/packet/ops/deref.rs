use std::ops::{Add, Sub, Mul, Div, Rem, BitAnd, BitOr, BitXor, Shl, Shr}; 
use std::simd::{Simd, SimdElement, LaneCount, SupportedLaneCount};

use crate::packet::Packet;


macro_rules! deref_lhs {
    (impl<T, const N: usize> $trait:ident for $packet:ty {
            fn $call:ident
        }) => {
        impl<T, const N: usize> $trait<$packet> for &$packet
        where
            T: SimdElement,
            LaneCount<N>: SupportedLaneCount,
            Simd<T, N>: $trait<Simd<T, N>, Output = Simd<T, N>>,
        {
            type Output = Packet<T, N>;
    
            #[inline]
            fn $call(self, rhs: $packet) -> $packet {
                Packet {
                    v: (*self).v.$call(rhs.v)
                }
            }
        }
    };
}

macro_rules! deref_rhs {
    (impl<T, const N: usize> $trait:ident for $packet:ty {
            fn $call:ident
        }) => {
        impl<T, const N: usize> $trait<&$packet> for $packet
        where
            T: SimdElement,
            LaneCount<N>: SupportedLaneCount,
            Simd<T, N>: $trait<Simd<T, N>, Output = Simd<T, N>>,
        {
            type Output = Packet<T, N>;
    
            #[inline]
            fn $call(self, rhs: &$packet) -> $packet {
                Packet {
                    v: self.v.$call((*rhs).v)
                }
            }
        }
    };
}

macro_rules! derefs {
    ($(impl<T, const N: usize> $trait:ident for $packet:ty {
            fn $call:ident
        })*) => {
        $(
            deref_lhs! {
                impl<T, const N: usize> $trait for $packet {
                    fn $call
                }
            }

            deref_rhs! {
                impl<T, const N: usize> $trait for $packet {
                    fn $call
                }
            }

            impl<'a, 'b, T, const N: usize> $trait<&'b $packet> for &'a $packet
            where
                T: SimdElement,
                LaneCount<N>: SupportedLaneCount,
                Simd<T, N>: $trait<Simd<T, N>, Output = Simd<T, N>>,
            {
                type Output = Packet<T, N>;

                #[inline]
                fn $call(self, rhs: &'b $packet) -> $packet {
                    Packet {
                        v: (*self).v.$call((*rhs).v)
                    }
                }
            }
        )*
    }
}


derefs! {
    impl<T, const N: usize> Add for Packet<T, N> {
        fn add
    }

    impl<T, const N: usize> Mul for Packet<T, N> {
        fn mul
    }

    impl<T, const N: usize> Sub for Packet<T, N> {
        fn sub
    }

    impl<T, const N: usize> Div for Packet<T, N> {
        fn div
    }

    impl<T, const N: usize> Rem for Packet<T, N> {
        fn rem
    }

    impl<T, const N: usize> BitAnd for Packet<T, N> {
        fn bitand
    }

    impl<T, const N: usize> BitOr for Packet<T, N> {
        fn bitor
    }

    impl<T, const N: usize> BitXor for Packet<T, N> {
        fn bitxor
    }

    impl<T, const N: usize> Shl for Packet<T, N> {
        fn shl
    }

    impl<T, const N: usize> Shr for Packet<T, N> {
        fn shr
    }
}
