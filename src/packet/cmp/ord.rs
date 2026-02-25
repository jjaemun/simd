use std::simd::{Mask, Simd, SimdElement, prelude::SimdPartialOrd, prelude::SimdOrd};
use crate::packet::{Packet, cmp::PacketPartialEq};


impl<T, const N: usize> PartialOrd for Packet<T, N>
where
    T: SimdElement + PartialOrd,
    Simd<T, N>: PartialOrd,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.v.partial_cmp(&other.v)
    }
}

impl<T, const N: usize> Ord for Packet<T, N>
where
    T: SimdElement + Ord,
    Simd<T, N>: Ord,
{
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.v.cmp(&other.v)
    }
}

pub trait PacketPartialOrd: PacketPartialEq{

    // Mask returning comparisons.

    #[must_use]
    fn cmplt(self, other: Self) -> Self::Mask;

    #[must_use]
    fn cmple(self, other: Self) -> Self::Mask;

    #[must_use]
    fn cmpgt(self, other: Self) -> Self::Mask;

    #[must_use]
    fn cmpge(self, other: Self) -> Self::Mask;
}

pub trait PacketOrd: PacketPartialOrd {

    // Full order implies the existence of
    // max and min.
    
    #[must_use]
    fn max(self, other: Self) -> Self;

    #[must_use]
    fn min(self, other: Self) -> Self;

    #[must_use]
    fn clamp(self, min: Self, max: Self) -> Self;
}



impl<T, const N: usize> PacketPartialOrd for Packet<T, N>
where
    T: SimdElement,
    Simd<T, N>: SimdPartialOrd,
{
    #[inline]
    fn cmplt(self, other: Self) -> Self::Mask {
        self.v.simd_lt(other.v)
    }

    #[inline]
    fn cmple(self, other: Self) -> Self::Mask {
        self.v.simd_le(other.v)
    }

    #[inline]
    fn cmpgt(self, other: Self) -> Self::Mask {
        self.v.simd_gt(other.v)
    }

    #[inline]
    fn cmpge(self, other: Self) -> Self::Mask {
        self.v.simd_ge(other.v)
    }
}


impl<T, const N: usize> PacketOrd for Packet<T, N> 
where
    T: SimdElement,
    Simd<T, N>: SimdOrd,
{
    #[inline] 
    fn  min(self, other: Self) -> Self {
        Self {
            v: self.v.simd_min(other.v),
        }
    }

    #[inline] 
    fn  max(self, other: Self) -> Self {
        Self {
            v: self.v.simd_max(other.v),
        }
    }

    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    fn clamp(self, min: Self, max: Self) -> Self {
        Self {
            v: self.v.simd_clamp(min.v, max.v),
        }
    }
}

