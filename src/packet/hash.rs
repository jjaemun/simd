use crate::packet::Packet;
use std::hash::Hash;
use std::simd::{Simd, SimdElement};

impl<T, const N: usize> Hash for Packet<T, N>
where
    T: SimdElement + Hash,
    Simd<T, N>: Hash,
{
    #[inline]
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        self.as_array().hash(state)
    }
}
