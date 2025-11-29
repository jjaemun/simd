use std::simd::{
    Simd, 
    SimdElement, 
    LaneCount, 
    SupportedLaneCount
};

use std::ops::{
    Add, 
    Sub, 
    Mul, 
    Div,
};

use crate::packets::Packet;


macro_rules! declare_packet_op {
    ($trait: ident, $func:ident, $op:tt) => {
        impl<T, const N: usize> $trait<Packet<T, N>> for Packet<T, N> 
        where 
            T: SimdElement + $trait<Output = T>,
            LaneCount<N>: SupportedLaneCount,
            Simd<T, N>: $trait<Output = Simd<T, N>>,
        {
            type Output = Packet<T, N>;

            #[inline(always)]
            fn $func(self, rhs: Packet<T, N>) -> Self::Output {
                Packet {
                    v: self.v $op rhs.v
                }
            }
        }

        impl<T, const N: usize> $trait<&Packet<T, N>> for Packet<T, N> 
        where 
            T: SimdElement + $trait<Output = T>,
            LaneCount<N>: SupportedLaneCount,
            Simd<T, N>: $trait<Output = Simd<T, N>>,
        {
            type Output = Packet<T, N>;

            #[inline(always)]
            fn $func(self, rhs: &Packet<T, N>) -> Self::Output {
                Packet {
                    v: self.v $op rhs.v
                }
            }
        }

        impl<T, const N: usize> $trait<Packet<T, N>> for &Packet<T, N> 
        where 
            T: SimdElement + $trait<Output = T>,
            LaneCount<N>: SupportedLaneCount,
            Simd<T, N>: $trait<Output = Simd<T, N>>,
        {
            type Output = Packet<T, N>;

            #[inline(always)]
            fn $func(self, rhs: Packet<T, N>) -> Self::Output {
                Packet {
                    v: self.v $op rhs.v
                }
            }
        }

        impl<T, const N: usize> $trait<&Packet<T, N>> for &Packet<T, N> 
        where 
            T: SimdElement + $trait<Output = T>,
            LaneCount<N>: SupportedLaneCount,
            Simd<T, N>: $trait<Output = Simd<T, N>>,
        {
            type Output = Packet<T, N>;

            #[inline(always)]
            fn $func(self, rhs: &Packet<T, N>) -> Self::Output {
                Packet {
                    v: self.v $op rhs.v
                }
            }
        }

        impl<T, const N: usize> $trait<T> for Packet<T, N> 
        where 
            T: SimdElement + $trait<Output=T>,
            LaneCount<N>: SupportedLaneCount,
            Simd<T, N>: $trait<Output=Simd<T, N>>,
        {
            type Output = Packet<T, N>;

            #[inline(always)]
            fn $func(self, rhs: T) -> Self::Output {
                Packet {
                    v: self.v $op Self::splat(rhs).v
                }
            }
        }
    
        impl<T, const N: usize> $trait<T> for &Packet<T, N> 
        where 
            T: SimdElement + $trait<Output=T>,
            LaneCount<N>: SupportedLaneCount,
            Simd<T, N>: $trait<Output=Simd<T, N>>,
        {
            type Output = Packet<T, N>;

            #[inline(always)]
            fn $func(self, rhs: T) -> Self::Output {
                Packet {
                    v: self.v $op Packet::splat(rhs).v
                }
            }
        }
    };
}


declare_packet_op!(Add, add, +);
declare_packet_op!(Sub, sub, -);
declare_packet_op!(Mul, mul, *);
declare_packet_op!(Div, div, /);
