use std::simd::{Simd, SimdElement, LaneCount, SupportedLaneCount};
use std::fmt;
use crate::packet::Packet;


impl<T, const N: usize> fmt::Debug for Packet<T, N> 
where 
    T: SimdElement + fmt::Debug,
    LaneCount<N>: SupportedLaneCount,
    Simd<T, N>: fmt::Debug,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Simd<T, N> as fmt::Debug>::fmt(self.as_simd(), f)
    }
}
