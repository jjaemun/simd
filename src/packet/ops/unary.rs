use crate::packet::Packet;
use std::ops::{Neg, Not};
use std::simd::{Simd, SimdElement};

impl<T, const N: usize> Neg for Packet<T, N>
where
    T: SimdElement,
    Simd<T, N>: Neg<Output = Simd<T, N>>,
{
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self { v: -self.v }
    }
}

impl<T, const N: usize> Not for Packet<T, N>
where
    T: SimdElement,
    Simd<T, N>: Not<Output = Simd<T, N>>,
{
    type Output = Self;

    #[inline]
    fn not(self) -> Self::Output {
        Self { v: !self.v }
    }
}
