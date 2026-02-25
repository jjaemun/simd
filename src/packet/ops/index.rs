use std::simd::{Simd, SimdElement};

use crate::packet::Packet;

impl<I, T, const N: usize> std::ops::Index<I> for Packet<T, N>
where
    T: SimdElement,
    I: std::slice::SliceIndex<[T]>,
{
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        &self.v.as_array()[index]
    }
}

impl<I, T, const N: usize> std::ops::IndexMut<I> for Packet<T, N>
where
    T: SimdElement,
    I: std::slice::SliceIndex<[T]>,
{
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.v.as_mut_array()[index]
    }
}
