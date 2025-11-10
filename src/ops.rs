#![feature(portable_simd)]

use crate::packets::Packet;


use std::simd::*;
use std::ops::*;



impl<T: Copy, const N: usize> Packet<T, N> {
    pub fn new(v: [T; N]) -> Self {
        Self { v: Simd::from_array(v) }
    }
}


impl<T, const N: usize> Deref for Packet<T, N> {
    type Target = Simd<T, N>;
    fn deref(&self) -> &Self::Target {
        &self.v
    }
}


impl<T, const N: usize> DerefMut for Packet<T, N> {
    type Target = Simd<T, N>;
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.v
    }
}


macro_rules! declare_arithmetic_overloads {
    ($trait: ident, $func: ident, $op: tt) => {
        impl<T, const N: usize> $trait for Packet<T, N> 
        where 
            T: Copy + $trait<Output = T>,  
        {
            type Output = Self;
            fn $func(self, rhs: Self) -> Self::Output {
                Packet { v: self.v $op rhs.v }
            }
        }
    };
}

declare_arithmetic_overloads!(Add, add, +);
declare_arithmetic_overloads!(Sub, sub, -);
declare_arithmetic_overloads!(Mul, mul, *);
declare_arithmetic_overloads!(Div, div, /);
