use std::simd::{Simd, SimdElement, LaneCount, SupportedLaneCount};

#[repr(transparent)]
pub struct Packet<T, const N: usize> 
    where
        T: SimdElement,
        LaneCount<N>: SupportedLaneCount,
{
    pub v: Simd<T, N>,
}

impl<T, const N: usize> Packet<T, N> 
    where
        T: SimdElement,
        LaneCount<N>: SupportedLaneCount,
{
    pub const LEN: usize = N;

    #[inline]
    pub const fn len(&self) -> usize {
        Self::LEN
    }
    
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn splat(v: T) -> Self {
        Self { 
            v: Simd::splat(v)
        }
    }

    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub const fn as_array(&self) -> &[T; N] {
        self.v.as_array()
    }

    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn as_mut_array(&mut self) -> &mut [T; N] {
        self.v.as_mut_array()
    }

    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn from_array(array: [T; N]) -> Self {
        Self { 
            v: Simd::from_array(array) 
        }
    }

    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn to_array(self) -> [T; N] {
        self.v.to_array()
    }

    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn from_simd(simd: Simd<T, N>) -> Self {
        Self { 
            v: simd
        }
    }
    
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub const fn as_simd(&self) -> &Simd<T, N> {
        &self.v
    }
    
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub const fn as_simd_mut(&mut self) -> &mut Simd<T, N> {
        &mut self.v
    }


    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn to_simd(self) -> Simd<T, N> {
        self.v
    }

    
    #[must_use]
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub const fn from_slice(slice: &[T]) -> Self {
        Self {
            v: Simd::from_slice(slice)
        }
    }
   
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn copy_to_slice(self, slice: &mut [T]) {
        self.v.copy_to_slice(slice)
    }
}


impl<T, const N: usize> Copy for Packet<T, N> 
where 
    T: SimdElement,
    LaneCount<N>: SupportedLaneCount,
{}

impl<T, const N: usize> Clone for Packet<T, N> 
where 
    T: SimdElement,
    LaneCount<N>: SupportedLaneCount,
{
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T, const N: usize> Default for Packet<T, N> 
where 
    T: SimdElement + Default,
    LaneCount<N>: SupportedLaneCount,
{
    #[inline]
    fn default() -> Self {
        Self::splat(T::default())
    }
}

impl<T, const N: usize> AsRef<[T; N]> for Packet<T, N>
where
    T: SimdElement,
    LaneCount<N>: SupportedLaneCount,
{
    #[inline]
    fn as_ref(&self) -> &[T; N] {
        self.as_array()
    }
}

impl<T, const N: usize> AsMut<[T; N]> for Packet<T, N>
where
    T: SimdElement,
    LaneCount<N>: SupportedLaneCount,
{
    #[inline]
    fn as_mut(&mut self) -> &mut [T; N] {
        self.as_mut_array()
    }
}

impl<T, const N: usize> AsRef<[T]> for Packet<T, N>
where
    T: SimdElement,
    LaneCount<N>: SupportedLaneCount,
{
    #[inline]
    fn as_ref(&self) -> &[T] {
        self.as_array()
    }
}

impl<T, const N: usize> AsMut<[T]> for Packet<T, N>
where
    T: SimdElement,
    LaneCount<N>: SupportedLaneCount,
{
    #[inline]
    fn as_mut(&mut self) -> &mut [T] {
        self.as_mut_array()
    }
}

impl<T, const N: usize> From<[T; N]> for Packet<T, N>
where
    T: SimdElement,
    LaneCount<N>: SupportedLaneCount,
{
    #[inline]
    fn from(array: [T; N]) -> Self {
        Self::from_array(array)
    }
}

impl<T, const N: usize> From<Simd<T, N>> for Packet<T, N>
where
    T: SimdElement,
    LaneCount<N>: SupportedLaneCount,
{
    #[inline]
    fn from(simd: Simd<T, N>) -> Self {
        Self::from_simd(simd)
    }
}

impl<T, const N: usize> From<Packet<T, N>> for [T; N]
where
    T: SimdElement,
    LaneCount<N>: SupportedLaneCount,
{
    #[inline]
    fn from(packet: Packet<T, N>) -> Self {
        packet.to_array()
    }
}

impl<T, const N: usize> From<Packet<T, N>> for Simd<T, N>
where
    T: SimdElement,
    LaneCount<N>: SupportedLaneCount,
{
    #[inline]
    fn from(packet: Packet<T, N>) -> Self {
        packet.to_simd()
    }
}

impl<T, const N: usize> TryFrom<&[T]> for Packet<T, N>
where
    T: SimdElement,
    LaneCount<N>: SupportedLaneCount,
{
    type Error = core::array::TryFromSliceError;

    #[inline]
    fn try_from(slice: &[T]) -> Result<Self, Self::Error> {
        Ok(Self::from_array(slice.try_into()?))
    }
}

impl<T, const N: usize> TryFrom<&mut [T]> for Packet<T, N>
where
    T: SimdElement,
    LaneCount<N>: SupportedLaneCount,
{
    type Error = core::array::TryFromSliceError;

    #[inline]
    fn try_from(slice: &mut [T]) -> Result<Self, Self::Error> {
        Ok(Self::from_array(slice.try_into()?))
    }
}

macro_rules! packets {
    ($(pub struct $name:ident<T: SimdElement> {
            LANES: $n:expr
        })*) => {
        $(
            pub type $name<T> = Packet<T, $n>;
        )*
    };
}


packets! {
    pub struct Packet2<T: SimdElement> {
        LANES: 2
    }

    pub struct Packet4<T: SimdElement> {
        LANES: 4
    }

    pub struct Packet8<T: SimdElement> {
        LANES: 8
    }

    pub struct Packet16<T: SimdElement> {
        LANES: 16
    }
}
