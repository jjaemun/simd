use core::iter::{Product, Sum};
use core::ops::{Add, Mul};
use crate::packet::Packet;

/// Vertical reductions over a collection of Packets:
///     iter.sum() → reduces packets horizontally using Add
///     iter.product() → reduces packets horizontally using Mul
///
/// These do NOT iterate over lanes; they iterate over *collections* of packets.
/// (e.g., Vec<Packet<T, N>>)
impl<T, const N: usize> Sum for Packet<T, N>
where
    T: SimdElement + Default,
    Packet<T, N>: Add<Output = Packet<T, N>>,
{
    #[inline]
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        // Identity for addition = 0 (broadcast)
        Packet::splat(T::default()) + iter.fold(Packet::splat(T::default()), Add::add)
    }
}

impl<'a, T, const N: usize> Sum<&'a Packet<T, N>> for Packet<T, N>
where
    T: SimdElement + Default + Copy,
    Packet<T, N>: Add<Output = Packet<T, N>>,
{
    #[inline]
    fn sum<I: Iterator<Item = &'a Packet<T, N>>>(iter: I) -> Self {
        iter.fold(Packet::splat(T::default()), |acc, p| acc + *p)
    }
}

impl<T, const N: usize> Product for Packet<T, N>
where
    T: SimdElement + Copy + num_traits::One,
    Packet<T, N>: Mul<Output = Packet<T, N>>,
{
    #[inline]
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        // Identity for multiplication = 1 (broadcast)
        iter.fold(Packet::splat(T::one()), Mul::mul)
    }
}

impl<'a, T, const N: usize> Product<&'a Packet<T, N>> for Packet<T, N>
where
    T: SimdElement + Copy + num_traits::One,
    Packet<T, N>: Mul<Output = Packet<T, N>>,
{
    #[inline]
    fn product<I: Iterator<Item = &'a Packet<T, N>>>(iter: I) -> Self {
        iter.fold(Packet::splat(T::one()), |acc, p| acc * *p)
    }
}

