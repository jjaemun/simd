use std::simd::{Simd, SimdElement, StdFloat};
use crate::packet::Packet;




#[inline(always)]
pub fn sqrt<T: SimdElement, const N: usize>(a: Packet<T, N>) -> Packet<T, N> 
where
    Simd<T, N>: StdFloat,
    LaneCount<N>: SupportedLaneCount,
{
    Packet { v: a.v.sqrt() }
}

