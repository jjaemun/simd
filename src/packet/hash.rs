use std::simd::{Simd, SimdElement, LaneCount, SupportedLaneCount};
use std::hash::{Hash};
use crate::packet::Packet;

impl<T, const N: usize> Hash for Packet<T, N>
where
    T: SimdElement + Hash,
    LaneCount<N>: SupportedLaneCount,
    Simd<T, N>: Hash,
{
    #[inline]
    fn hash<H>(&self, state: &mut H) 
    where H: std::hash::Hasher,
    {
        self.as_array().hash(state)
    }
}

