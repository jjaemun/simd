use std::simd::{Mask, Simd, SimdElement};

#[repr(transparent)]
pub struct Packet<T, const N: usize> 
    where
        T: SimdElement,
{
    pub v: Simd<T, N>,
}

impl<T, const N: usize> Packet<T, N> 
    where
        T: SimdElement,
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
    
    #[must_use]
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn load_select_ptr(
        addr: *const T, 
        enable: Mask<<T as SimdElement>::Mask, N>,
        or: Self
    ) -> Self {
        unsafe {
            Self {
                v: Simd::load_select_ptr(addr, enable, or.v)
            }
        }
    }

    #[must_use]
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn load_select_unchecked(
        slice: &[T],
        enable: Mask<<T as SimdElement>::Mask, N>,
        or: Self,
    ) -> Self {
        unsafe {
            Self {
                v: Simd::load_select_unchecked(slice, enable, or.v)
            }
        }
    }


    #[must_use]
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn load_select(        
        slice: &[T],
        enable: Mask<<T as SimdElement>::Mask, N>,
        or: Self,
    ) -> Self {
        Self {
            v: Simd::load_select(slice, enable, or.v)
        }
    }

    #[must_use]
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn load_select_or_default(slice: &[T], enable: Mask<<T as SimdElement>::Mask, N>) -> Self 
    where 
        T: Default,
    {
        Self {
            v: Simd::load_select(slice, enable, Default::default())
        }
    }


    #[must_use]
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn load_or(slice: &[T], or: Self) -> Self {
        Self {
            v: Simd::load_or(slice, or.v)
        }
    }

    #[must_use]
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn load_or_default(slice: &[T]) -> Self 
    where 
        T: Default,
    {
        Self {
            v: Simd::load_or_default(slice)
        }
    }

    #[must_use]
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn gather_select_ptr(
        src: Packet<*const T, N>,
        enable: Mask<isize, N>,
        or: Self,
    ) -> Self {
        unsafe {
            Self {
                v: Simd::gather_select_ptr(src.v, enable, or.v)
            }
        }
    }

    #[must_use]
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn gather_ptr(src: Packet<*const T, N>) -> Self 
    where 
        T: Default,
    {
        unsafe {
            Self {
                v: Simd::gather_ptr(src.v)
            }
        }
    }

    #[must_use]
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn gather_select_unchecked(
        slice: &[T],
        enable: Mask<isize, N>,
        idxs: Packet<usize, N>,
        or: Self
    ) -> Self {
        unsafe {
            Self {
                v: Simd::gather_select_unchecked(slice, enable, idxs.v, or.v)
            }
        }
    }

    #[must_use]
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn gather_select(
        slice: &[T],
        enable: Mask<isize, N>,
        idxs: Packet<usize, N>,
        or: Self
    ) -> Self {
        Self {
            v: Simd::gather_select(slice, enable, idxs.v, or.v)
        }
    }

    #[must_use]
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn gather_or_default(slice: &[T], idxs: Packet<usize, N>) -> Self 
    where
        T: Default
    {
        Self {
            v: Simd::gather_or_default(slice, idxs.v)
        }
    }

    #[must_use]
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn gather_or(slice: &[T], idxs: Packet<usize, N>, or: Self) -> Self {
        Self {
            v: Simd::gather_or(slice, idxs.v, or.v)
        }
    }

    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn store_select(self, slice: &mut [T], enable: Mask<<T as SimdElement>::Mask, N>) {
        self.v.store_select(slice, enable)
    }

    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn store_select_unchecked(self, slice: &mut [T], enable: Mask<<T as SimdElement>::Mask, N>) {
        unsafe { self.v.store_select_unchecked(slice, enable) }
    }


    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn store_select_ptr(self, addr: *mut T, enable: Mask<<T as SimdElement>::Mask, N>) {
        unsafe { self.v.store_select_ptr(addr, enable) }
    }

    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn scatter(self, slice: &mut [T], idxs: Packet<usize, N>) {
        self.v.scatter(slice, idxs.v)
    }

    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn scatter_select(self, slice: &mut [T], enable: Mask<isize, N>, idxs: Packet<usize, N>) {
        self.v.scatter_select(slice, enable, idxs.v)
    }

    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn scatter_select_unchecked(self, slice: &mut [T], enable: Mask<isize, N>, idxs: Packet<usize, N>) {
        unsafe { self.v.scatter_select_unchecked(slice, enable, idxs.v) }
    }

    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn scatter_ptr(self, dst: Packet<*mut T, N>) {
        unsafe { self.v.scatter_ptr(dst.v) }
    }

    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn scatter_select_ptr(self, dst: Packet<*mut T, N>, enable: Mask<isize, N>) {
        unsafe { self.v.scatter_select_ptr(dst.v, enable) }
    }
} 


impl<T, const N: usize> Copy for Packet<T, N> 
where 
    T: SimdElement,
{}

impl<T, const N: usize> Clone for Packet<T, N> 
where 
    T: SimdElement,
{
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T, const N: usize> Default for Packet<T, N> 
where 
    T: SimdElement + Default,
{
    #[inline]
    fn default() -> Self {
        Self::splat(T::default())
    }
}

impl<T, const N: usize> AsRef<[T; N]> for Packet<T, N>
where
    T: SimdElement,
{
    #[inline]
    fn as_ref(&self) -> &[T; N] {
        self.as_array()
    }
}

impl<T, const N: usize> AsMut<[T; N]> for Packet<T, N>
where
    T: SimdElement,
{
    #[inline]
    fn as_mut(&mut self) -> &mut [T; N] {
        self.as_mut_array()
    }
}

impl<T, const N: usize> AsRef<[T]> for Packet<T, N>
where
    T: SimdElement,
{
    #[inline]
    fn as_ref(&self) -> &[T] {
        self.as_array()
    }
}

impl<T, const N: usize> AsMut<[T]> for Packet<T, N>
where
    T: SimdElement,
{
    #[inline]
    fn as_mut(&mut self) -> &mut [T] {
        self.as_mut_array()
    }
}

impl<T, const N: usize> From<[T; N]> for Packet<T, N>
where
    T: SimdElement,
{
    #[inline]
    fn from(array: [T; N]) -> Self {
        Self::from_array(array)
    }
}

impl<T, const N: usize> From<Simd<T, N>> for Packet<T, N>
where
    T: SimdElement,
{
    #[inline]
    fn from(simd: Simd<T, N>) -> Self {
        Self::from_simd(simd)
    }
}

impl<T, const N: usize> From<Packet<T, N>> for [T; N]
where
    T: SimdElement,
{
    #[inline]
    fn from(packet: Packet<T, N>) -> Self {
        packet.to_array()
    }
}

impl<T, const N: usize> From<Packet<T, N>> for Simd<T, N>
where
    T: SimdElement,
{
    #[inline]
    fn from(packet: Packet<T, N>) -> Self {
        packet.to_simd()
    }
}

impl<T, const N: usize> TryFrom<&[T]> for Packet<T, N>
where
    T: SimdElement,
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
