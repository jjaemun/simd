use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign, RemAssign, BitAndAssign, BitOrAssign, BitXorAssign, ShlAssign, ShrAssign}; 
use std::ops::{Add, Sub, Mul, Div, Rem, BitAnd, BitOr, BitXor, Shl, Shr}; 
use std::simd::{Simd, SimdElement};

use crate::packet::Packet;


macro_rules! binary {
    (impl<T, const N: usize> $trait:ident for $packet:ty {
            fn $call:ident 
        }) => {
        impl<T, const N: usize> $trait<T> for Packet<T, N>
        where
            T: SimdElement,
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

macro_rules! all {
    (impl<T, const N: usize> $trait:ident for $packet:ty {
            fn $call:ident
        }) => {
        impl<'a, 'b, T, const N: usize> $trait<&'b T> for &'a $packet
        where
            T: SimdElement,
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
    
        all! {
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


macro_rules! ops {
    (impl<const N: usize> Ops for $T:ty {
            into $packet:ty
        }) => {
        impl<const N: usize> Add<Packet<$T, N>> for $T
        where
            $T: SimdElement,
            Simd<$T, N>: Add<Simd<$T, N>, Output = Simd<$T, N>>,
        {
            type Output = Packet<$T, N>;
                
            fn add(self, rhs: Packet<$T, N>) -> Self::Output {
                Packet {
                    v: Packet::<$T, N>::splat(self).v.add(rhs.v)
                }
            }
        }

        impl<const N: usize> Mul<Packet<$T, N>> for $T 
        where 
            $T: SimdElement,
            Simd<$T, N>: Mul<Simd<$T, N>, Output = Simd<$T, N>>,
        {
            type Output = Packet<$T, N>;

            #[inline]
            fn mul(self, rhs: Packet<$T, N>) -> Self::Output {
                Packet {
                    v: Packet::<$T, N>::splat(self).v.mul(rhs.v)
                }
            }
        }

        impl<const N: usize> Sub<Packet<$T, N>> for $T
        where
            $T: SimdElement,
            Simd<$T, N>: Sub<Simd<$T, N>, Output = Simd<$T, N>>,
        {
            type Output = Packet<$T, N>;
                
            fn sub(self, rhs: Packet<$T, N>) -> Self::Output {
                Packet {
                    v: Packet::<$T, N>::splat(self).v.sub(rhs.v)
                }
            }
        }

        impl<const N: usize> Div<Packet<$T, N>> for $T
        where
            $T: SimdElement,
            Simd<$T, N>: Div<Simd<$T, N>, Output = Simd<$T, N>>,
        {
            type Output = Packet<$T, N>;
                
            fn div(self, rhs: Packet<$T, N>) -> Self::Output {
                Packet {
                    v: Packet::<$T, N>::splat(self).v.div(rhs.v)
                }
            }
        }

        impl<const N: usize> Rem<Packet<$T, N>> for $T
        where
            $T: SimdElement,
            Simd<$T, N>: Rem<Simd<$T, N>, Output = Simd<$T, N>>,
        {
            type Output = Packet<$T, N>;
                
            fn rem(self, rhs: Packet<$T, N>) -> Self::Output {
                Packet {
                    v: Packet::<$T, N>::splat(self).v.rem(rhs.v)
                }
            }
        }

        impl<const N: usize> BitAnd<Packet<$T, N>> for $T
        where
            $T: SimdElement,
            Simd<$T, N>: BitAnd<Simd<$T, N>, Output = Simd<$T, N>>,
        {
            type Output = Packet<$T, N>;
                
            fn bitand(self, rhs: Packet<$T, N>) -> Self::Output {
                Packet {
                    v: Packet::<$T, N>::splat(self).v.bitand(rhs.v)
                }
            }
        }
        
        impl<const N: usize> BitOr<Packet<$T, N>> for $T
        where
            $T: SimdElement,
            Simd<$T, N>: BitOr<Simd<$T, N>, Output = Simd<$T, N>>,
        {
            type Output = Packet<$T, N>;
                
            fn bitor(self, rhs: Packet<$T, N>) -> Self::Output {
                Packet {
                    v: Packet::<$T, N>::splat(self).v.bitor(rhs.v)
                }
            }
        }
        
        impl<const N: usize> BitXor<Packet<$T, N>> for $T
        where
            $T: SimdElement,
            Simd<$T, N>: BitXor<Simd<$T, N>, Output = Simd<$T, N>>,
        {
            type Output = Packet<$T, N>;
                
            fn bitxor(self, rhs: Packet<$T, N>) -> Self::Output {
                Packet {
                    v: Packet::<$T, N>::splat(self).v.bitxor(rhs.v)
                }
            }
        }
        
        impl<const N: usize> Shl<Packet<$T, N>> for $T
        where
            $T: SimdElement,
            Simd<$T, N>: Shl<Simd<$T, N>, Output = Simd<$T, N>>,
        {
            type Output = Packet<$T, N>;
                
            fn shl(self, rhs: Packet<$T, N>) -> Self::Output {
                Packet {
                    v: Packet::<$T, N>::splat(self).v.shl(rhs.v)
                }
            }
        }

        impl<const N: usize> Shr<Packet<$T, N>> for $T
        where
            $T: SimdElement,
            Simd<$T, N>: Shr<Simd<$T, N>, Output = Simd<$T, N>>,
        {
            type Output = Packet<$T, N>;
                
            fn shr(self, rhs: Packet<$T, N>) -> Self::Output {
                Packet {
                    v: Packet::<$T, N>::splat(self).v.shr(rhs.v)
                }
            }
        }
    };
}


macro_rules! ops {
    (impl<const N: usize> Ops<$packet:ty> for $T:ty {
        $(
            impl $trait:ident {
                fn $call:ident 
            })*
        }) => {
        $(
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
        )*
    };
}

ops! {
    impl<const N: usize> Ops<Packet<u32, N>> for u32 {
        impl Add  {
            fn add
        }

        impl Mul {
            fn mul
        }

        impl Sub {
            fn sub
        }

        impl Div {
            fn div
        }

        impl Rem {
            fn rem
        }

        impl BitAnd {
            fn bitand
        }
    
        impl BitOr {
            fn bitor
        }

        impl BitXor {
            fn bitxor
        }

        impl Shl {
            fn shl
        }

        impl Shr {
            fn shr
        }
    }
}
