use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign, RemAssign, BitAndAssign, BitOrAssign, BitXorAssign, ShlAssign, ShrAssign}; 
use std::ops::{Add, Sub, Mul, Div, Rem, BitAnd, BitOr, BitXor, Shl, Shr}; 
use std::simd::{Simd, SimdElement, LaneCount, SupportedLaneCount};

use crate::packet::Packet;


macro_rules! binary {
    (impl<T, const N: usize> $trait:ident for $packet:ty {
            fn $call:ident 
        }) => {
        impl<T, const N: usize> $trait<T> for Packet<T, N>
        where
            T: SimdElement,
            LaneCount<N>: SupportedLaneCount,
            Simd<T, N>: $trait<Simd<T, N>, Output = Simd<T, N>>,
        {
            type Output = Packet<T, N>;

            #[inline]
            fn $call(self, rhs: T) -> Self::Output {
                Packet {
                    v: self.v.$call(Self::splat(rhs).v)
                }
            }
        }
    };
}
    
macro_rules! lhs {
    (impl<T, const N: usize> $trait:ident for $packet:ty {
            fn $call:ident
        }) => {
        impl<T, const N: usize> $trait<T> for &$packet
        where
            T: SimdElement,
            LaneCount<N>: SupportedLaneCount,
            Simd<T, N>: $trait<Simd<T, N>, Output = Simd<T, N>>,
        {
            type Output = Packet<T, N>;
    
            #[inline]
            fn $call(self, rhs: T) -> $packet {
                Packet {
                    v: (*self).v.$call(Packet::<T, N>::splat(rhs).v)
                }
            }
        }
    };
}

macro_rules! rhs {
    (impl<T, const N: usize> $trait:ident for $packet:ty {
            fn $call:ident
        }) => {
        impl<T, const N: usize> $trait<&T> for $packet
        where
            T: SimdElement,
            LaneCount<N>: SupportedLaneCount,
            Simd<T, N>: $trait<Simd<T, N>, Output = Simd<T, N>>,
        {
            type Output = Packet<T, N>;
    
            #[inline]
            fn $call(self, rhs: &T) -> $packet {
                Packet {
                    v: self.v.$call(Self::splat(*rhs).v)
                }
            }
        }
    };
}

macro_rules! both {
    (impl<T, const N: usize> $trait:ident for $packet:ty {
            fn $call:ident
        }) => {
        impl<'a, 'b, T, const N: usize> $trait<&'b T> for &'a $packet
        where
            T: SimdElement,
            LaneCount<N>: SupportedLaneCount,
            Simd<T, N>: $trait<Simd<T, N>, Output = Simd<T, N>>,
        {
            type Output = Packet<T, N>;

            #[inline]
            fn $call(self, rhs: &'b T) -> $packet {
                Packet {
                    v: (*self).v.$call(Packet::<T, N>::splat(*rhs).v)
                }
            }
        }
    };
}

macro_rules! derefs {
    (impl<T, const N: usize> $trait:ident for $packet:ty {
            fn $call:ident
        }) => {
        lhs! {
            impl<T, const N: usize> $trait for $packet {
                fn $call
            }
        }

        rhs! {
            impl<T, const N: usize> $trait for $packet {
                fn $call
            }
        }
    
        both! {
            impl<T, const N: usize> $trait for $packet {
                fn $call
            }
        }
    }
}

macro_rules! scalar {
    ($(impl<T, const N: usize> $trait:ident for $packet:ty {
            fn $call:ident
        })*) => {
        $(
            binary! {
                impl<T, const N: usize> $trait for $packet {
                    fn $call
                }
            }

            derefs! {
                impl<T, const N: usize> $trait for $packet {
                    fn $call
                }
            }
        )*  
    }
}


scalar! {
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
