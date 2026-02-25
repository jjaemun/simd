use crate::packet::Packet;
use std::fmt;
use std::simd::{Simd, SimdElement};

impl<T, const N: usize> fmt::Debug for Packet<T, N>
where
    T: SimdElement + fmt::Debug,
    Simd<T, N>: fmt::Debug,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Simd<T, N> as fmt::Debug>::fmt(self.as_simd(), f)
    }
}
