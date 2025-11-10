#![feature(portable_simd)]

use crate::packets::Packet;

use std::ops::{
    Add,
    Sub,
    Mul,
    Div,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
};


macro_rules! declare_arithmetic_overloads {
    ($trait: ident, $func: ident, $op: tt) => {
        impl<T, const N: usize> $trait for Packet<T, N> 
        where 
            T: Copy + $trait<Output = T>,  
        {
            type Output = Self;

            #[inline(always)]
            fn $func(self, rhs: Self) -> Self::Output {
                Packet { v: self.v $op rhs.v }
            }
        }

        impl<T, const N: usize> $trait<T> for Packet<T, N>
        where 
            T: Copy + $trait<Output = T>,  
        {
            type Output = Packet<T, N>;
            
            #[inline(always)]
            fn $func(self, rhs: T) -> Self::Output {
                Packet { v: self.v $op Simd::splat(rhs) }
            }
        }

        impl<T, const N: usize> $trait<Packet<T,N>> for T
        where 
            T: Copy + $trait<Output = T>,  
        {
            type Output = Packet<T, N>;
            
            #[inline(always)]
            fn $func(self, rhs: Packet<T, N>) -> Self::Output {
                Packet { v: Simd::splat(self) $op rhs.v }
            }
        }
    };
}


macro_rules! declare_assign_overloads {
    ($trait: ident, $func: ident, $op: tt) => {
        impl <T, const N: usize> $trait<T> for Packet<T, N> 
        where
            T: Copy + $trait<Output = T>,
        {
            #[inline(always)]
            fn $func(&mut self, rhs: T) {
                self.v = self.v $op Simd::splat(rhs);
            }
        }
        
        impl <T, const N: usize> $trait for Packet<T, N> 
        where
            T: Copy + $trait<Output = T>,
        {
            #[inline(always)]
            fn $func(&mut self, rhs: Self) {
                self.v = self.v $op rhs.v;
            }
        }
    };
}


declare_arithmetic_overloads!(Add, add, +);
declare_arithmetic_overloads!(Sub, sub, -);
declare_arithmetic_overloads!(Mul, mul, *);
declare_arithmetic_overloads!(Div, div, /);

declare_assign_overloads!(AddAssign, add_assign, +);
declare_assign_overloads!(SubAssign, sub_assign, -);
declare_assign_overloads!(MulAssign, mul_assign, *);
declare_assign_overloads!(DivAssign, div_assign, /);
