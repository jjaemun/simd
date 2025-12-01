use std::simd::{Simd, SimdElement, LaneCount, SupportedLaneCount};


#[derive(Clone, Copy, Debug)]
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
    pub fn splat(v: T) -> Self {
        Self { 
            v: Simd::splat(v)
        }
    }

    #[inline]
    pub const fn as_array(&self) -> &[T; N] {
        self.v.as_array()
    }

    #[inline]
    pub fn as_mut_array(&mut self) -> &mut [T; N] {
        (*self).v.as_mut_array()
    }

    #[inline]
    pub fn from_array(array: [T; N]) -> Self {
        Self { 
            v: Simd::from_array(array) 
        }
    }

    #[inline]
    pub fn to_array(self) -> [T; N] {
        self.v.to_array()
    }
    
    #[must_use]
    #[inline]
    #[track_caller]
    pub const fn from_slice(slice: &[T]) -> Self {
        Self {
            v: std::simd::Simd::from_slice(slice)
        }
    }
   
    #[inline]
    #[track_caller]
    pub fn copy_to_slice(self, slice: &mut [T]) {
        self.v.copy_to_slice(slice)
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
