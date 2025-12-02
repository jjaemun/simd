use std::simd::{Simd, SimdElement, LaneCount, SupportedLaneCount};
use crate::packet::Packet;

impl<T, const N: usize> PartialEq for Packet<T, N>
where
    T: SimdElement + PartialEq,
    LaneCount<N>: SupportedLaneCount,
    Simd<T, N>: PartialEq,
{
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
