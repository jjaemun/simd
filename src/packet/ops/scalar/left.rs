use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign, RemAssign, BitAndAssign, BitOrAssign, BitXorAssign, ShlAssign, ShrAssign}; 
use std::ops::{Add, Sub, Mul, Div, Rem, BitAnd, BitOr, BitXor, Shl, Shr}; 
use std::simd::{Simd, SimdElement};

use crate::packet::Packet;

// Left scalar operations --symmetrizing.
macro_rules! binary {
    (impl<const N: usize> $trait:ident<$packet:ty> for $T:ty {
            fn $call:ident 
        }) => {
        impl<const N: usize> $trait<$packet> for $T
        where
            $T: SimdElement,
            Simd<$T, N>: $trait<Simd<$T, N>, Output = Simd<$T, N>>,
        {
            type Output = $packet;

            #[inline]
            fn $call(self, rhs: $packet) -> Self::Output {
                Packet {
                    v: Packet::<$T, N>::splat(self).v.$call(rhs.v)
                }
            }
        }
    };
}

macro_rules! lhs {
    (impl<const N: usize> $trait:ident<$packet:ty> for $T:ty {
            fn $call:ident 
        }) => {
        impl<const N: usize> $trait<$packet> for &$T
        where
            $T: SimdElement,
            Simd<$T, N>: $trait<Simd<$T, N>, Output = Simd<$T, N>>,
        {
            type Output = $packet;

            #[inline]
            fn $call(self, rhs: $packet) -> Self::Output {
                Packet {
                    v: Packet::<$T, N>::splat(*self).v.$call(rhs.v)
                }
            }
        }
    };
}

macro_rules! rhs {
    (impl<const N: usize> $trait:ident<$packet:ty> for $T:ty {
            fn $call:ident 
        }) => {
        impl<const N: usize> $trait<&$packet> for $T
        where
            $T: SimdElement,
            Simd<$T, N>: $trait<Simd<$T, N>, Output = Simd<$T, N>>,
        {
            type Output = $packet;

            #[inline]
            fn $call(self, rhs: &$packet) -> Self::Output {
                Packet {
                    v: Packet::<$T, N>::splat(self).v.$call((*rhs).v)
                }
            }
        }
    };
}

macro_rules! all {
    (impl<const N: usize> $trait:ident<$packet:ty> for $T:ty {
            fn $call:ident 
        }) => {
        impl<'a, 'b, const N: usize> $trait<&'b $packet> for &'a $T
        where
            $T: SimdElement,
            Simd<$T, N>: $trait<Simd<$T, N>, Output = Simd<$T, N>>,
        {
            type Output = $packet;

            #[inline]
            fn $call(self, rhs: &'b $packet) -> Self::Output {
                Packet {
                    v: Packet::<$T, N>::splat((*self)).v.$call((*rhs).v)
                }
            }
        }
    };
}

macro_rules! derefs {
    (impl<const N: usize> $trait:ident<$packet:ty> for $T:ty {
            fn $call:ident 
        }) => {
        lhs! {
            impl<const N: usize> $trait<$packet> for $T {
                fn $call
            }
        }

        rhs! {
            impl<const N: usize> $trait<$packet> for $T {
                fn $call
            }
        }
    
        all! {
            impl<const N: usize> $trait<$packet> for $T {
                fn $call
            }
        }
    }
}

macro_rules! scalar {
    ($(impl<const N: usize> $trait:ident<$packet:ty> for $T:ty {
            fn $call:ident 
        })*) => {
        $(
            binary! {
                impl<const N: usize> $trait<$packet> for $T {
                    fn $call
                }
            }

            derefs! {
                impl<const N: usize> $trait<$packet> for $T {
                    fn $call
                }
            }
        )*  
    }
}

macro_rules! bitwise {
    ($($T:ty)*) => {
        $(
            scalar! {
                impl<const N: usize> BitAnd<Packet<$T, N>> for $T {
                    fn bitand
                }

                impl<const N: usize> BitOr<Packet<$T, N>> for $T {
                    fn bitor
                }
    
                impl<const N: usize> BitXor<Packet<$T, N>> for $T {
                    fn bitxor
                }
    
                impl<const N: usize> Shl<Packet<$T, N>> for $T {
                    fn shl
                }

                impl<const N: usize> Shr<Packet<$T, N>> for $T {
                    fn shr
                }
            }
        )*
    }
}

bitwise! {
    (u8)
    (u16)
    (u32)
    (u64)
    
    (i8)
    (i16)
    (i32)
    (i64)
} 


macro_rules! arithmetic {
    ($($T:ty)*) => {
        $(
            scalar! {
                impl<const N: usize> Add<Packet<$T, N>> for $T{
                    fn add
                }

                impl<const N: usize> Mul<Packet<$T, N>> for $T {
                    fn mul
                }

                impl<const N: usize> Sub<Packet<$T, N>> for $T {
                    fn sub
                }

                impl<const N: usize> Div<Packet<$T, N>> for $T {
                    fn div
                }
                
                impl<const N: usize> Rem<Packet<$T, N>> for $T {
                    fn rem
                }
           }
        )*
    }
}

arithmetic! {
    (u8)
    (u16)
    (u32)
    (u64)
    
    (i8)
    (i16)
    (i32)
    (i64)   

    (f32)
    (f64)
}
