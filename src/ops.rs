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


macro_rules! declare_sv_arithmetic_overloads {
    ($trait: ident, $func: ident, $op: tt) => {
        impl<T, const N: usize> $trait<Packet<T,N>> for T
        where 
            T: Copy + $trait<Output = T>,  
        {
            type Output = Packet<T, N>;
            fn $func(self, rhs: Packet<T, N>) -> Self::Output {
                Packet { v: Simd::splat(self) $op rhs.v }
            }
        }
    };
}

declare_sv_arithmetic_overloads!(Add, add, +);
declare_sv_arithmetic_overloads!(Sub, sub, -);
declare_sv_arithmetic_overloads!(Mul, mul, *);
declare_sv_arithmetic_overloads!(Div, div, /);


macro_rules! declare_vs_arithmetic_overloads {
    ($trait: ident, $func: ident, $op: tt) => {
        impl<T, const N: usize> $trait<T> for Packet<T, N>
        where 
            T: Copy + $trait<Output = T>,  
        {
            type Output = Packet<T, N>;
            fn $func(self, rhs: T) -> Self::Output {
                Packet { v: self.v $op Simd::splat(rhs) }
            }
        }
    };
}

declare_vs_arithmetic_overloads!(Add, add, +);
declare_vs_arithmetic_overloads!(Sub, sub, -);
declare_vs_arithmetic_overloads!(Mul, mul, *);
declare_vs_arithmetic_overloads!(Div, div, /);


macro_rules! declare_assign_overloads {
    ($trait: ident, $func: ident, $op: tt) => {
        impl <T, const N: usize> $trait for Packet<T, N> 
        where
            T: Copy + $trait<Output = T>,
        {
            fn $func(&mut self, rhs: Self) {
                self.v = self.v $op rhs.v;
            }
        }
    };
}

declare_assign_overloads!(AddAssign, add_assign, +);
declare_assign_overloads!(SubAssign, sub_assign, -);
declare_assign_overloads!(MulAssign, mul_assign, *);
declare_assign_overloads!(DivAssign, div_assign, /);


macro_rules! declare_scalar_assign_overloads {
    ($trait: ident, $func: ident, $op: tt) => {
        impl <T, const N: usize> $trait<T> for Packet<T, N> 
        where
            T: Copy + $trait<Output = T>,
        {
            fn $func(&mut self, rhs: T) {
                self.v = self.v $op Simd::splat(rhs);
            }
        }
    };
}

declare_scalar_assign_overloads!(AddAssign, add_assign, +);
declare_scalar_assign_overloads!(SubAssign, sub_assign, -);
declare_scalar_assign_overloads!(MulAssign, mul_assign, *);
declare_scalar_assign_overloads!(DivAssign, div_assign, /);
