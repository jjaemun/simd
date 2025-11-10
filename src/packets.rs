#![feature(portable_simd)]


#[derive(Clone, Copy, Debug)]
pub struct Packet<T, const N: usize> {
    pub v: Simd<T, N>,
}


impl<T, const N: usize> std::ops::Deref for Packet<T, N> {
    type Target = Simd<T, N>;
    fn deref(&self) -> &Self::Target {
        &self.v
    }
}


impl<T, const N: usize> std::ops::DerefMut for Packet<T, N> {
    type Target = Simd<T, N>;
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.v
    }
}


impl<T: Copy, const N: usize> Packet<T, N> {
    
    #[inline(always)]
    pub fn new(v: [T; N]) -> Self {
        Self { 
            v: std::simd::Simd::from_array(v) 
        }
    }
    
    #[inline(always)]
    pub fn splat(v: T) -> Self {
        Self { 
            v: std::simd::Simd::splat(v)
        }
    }
}


macro_rules! declare_packet {
    ($name:ident, $n: expr) => {
        pub type $name<T> = Packet<T, $n>;
    };
}


declare_packet!(Packet2, 2);
declare_packet!(Packet4, 4);
declare_packet!(Packet8, 8);
declare_packet!(Packet16, 16);
