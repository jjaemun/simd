use std::simd::{Mask, prelude::SimdPartialEq, Simd, SimdElement, LaneCount, SupportedLaneCount};
use crate::packet::Packet;


impl<T, const N: usize> PartialEq for Packet<T, N>
where
    T: SimdElement + PartialEq,
    LaneCount<N>: SupportedLaneCount,
    Simd<T, N>: PartialEq,
{
    // Rust's operator overloading constrains equality
    // comparisons to return a pure bool type. In this
    // They are included for completeness but are very 
    // uninteresting in the sense of simd operations.

    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.v.eq(&other.v)
    }

    #[inline]
    #[allow(clippy::partialeq_ne_impl)]
    fn ne(&self, other: &Self) -> bool {
        self.v.ne(&other.v)
    }
}

impl<T, const N: usize> Eq for Packet<T, N>
where
    T: SimdElement + Eq,
    LaneCount<N>: SupportedLaneCount,
    Simd<T, N>: Eq,
{}


pub trait PacketPartialEq {

    // Mask returning equality comparisons. 

    type Mask;

    #[must_use]
    fn cmpeq(self, other: Self) -> Self::Mask;

    #[must_use]
    fn cmpne(self, other: Self) -> Self::Mask;
}


impl<T, const N: usize> PacketPartialEq for Packet<T, N>
where
    T: SimdElement,
    LaneCount<N>: SupportedLaneCount,
    Simd<T, N>: SimdPartialEq,
{
    type Mask = <Simd<T, N> as SimdPartialEq>::Mask;

    #[inline]
    fn cmpeq(self, other: Self) -> Self::Mask {
        self.v.simd_eq(other.v)
    }

    #[inline]
    fn cmpne(self, other: Self) -> Self::Mask {
        self.v.simd_ne(other.v)
    }
}








