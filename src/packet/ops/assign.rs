use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Rem, Shl, Shr, Sub};
use std::ops::{
    AddAssign, BitAndAssign, BitOrAssign, BitXorAssign, DivAssign, MulAssign, RemAssign, ShlAssign,
    ShrAssign, SubAssign,
};
use std::simd::{Simd, SimdElement};

use crate::packet::Packet;

macro_rules! assign {
        ($(impl<T, U, const N: usize> $assign:ident<U> for Packet<T, N>
        where
            Self: $trait:ident,
        {
            fn $assign_call:ident(rhs: U) {
                $call:ident
            }
        })*) => {
        $(impl<T, U, const N: usize> $assign<U> for Packet<T, N>
        where
            T: SimdElement,
            Packet<T, N>: $trait<U, Output = Packet<T, N>>,
        {
            #[inline]
            fn $assign_call(&mut self, rhs: U) {
                *self = (*self).$call(rhs);
            }
        })*
    }
}

assign! {
    impl<T, U, const N: usize> AddAssign<U> for Packet<T, N>
    where
        Self: Add,
    {
        fn add_assign(rhs: U) {
            add
        }
    }

    impl<T, U, const N: usize> MulAssign<U> for Packet<T, N>
    where
        Self: Mul,
    {
        fn mul_assign(rhs: U) {
            mul
        }
    }

    impl<T, U, const N: usize> SubAssign<U> for Packet<T, N>
    where
        Self: Sub,
    {
        fn sub_assign(rhs: U) {
            sub
        }
    }

    impl<T, U, const N: usize> DivAssign<U> for Packet<T, N>
    where
        Self: Div,
    {
        fn div_assign(rhs: U) {
            div
        }
    }
    impl<T, U, const N: usize> RemAssign<U> for Packet<T, N>
    where
        Self: Rem,
    {
        fn rem_assign(rhs: U) {
            rem
        }
    }

    impl<T, U, const N: usize> BitAndAssign<U> for Packet<T, N>
    where
        Self: BitAnd,
    {
        fn bitand_assign(rhs: U) {
            bitand
        }
    }

    impl<T, U, const N: usize> BitOrAssign<U> for Packet<T, N>
    where
        Self: BitOr,
    {
        fn bitor_assign(rhs: U) {
            bitor
        }
    }

    impl<T, U, const N: usize> BitXorAssign<U> for Packet<T, N>
    where
        Self: BitXor,
    {
        fn bitxor_assign(rhs: U) {
            bitxor
        }
    }

    impl<T, U, const N: usize> ShlAssign<U> for Packet<T, N>
    where
        Self: Shl,
    {
        fn shl_assign(rhs: U) {
            shl
        }
    }

    impl<T, U, const N: usize> ShrAssign<U> for Packet<T, N>
    where
        Self: Shr,
    {
        fn shr_assign(rhs: U) {
            shr
        }
    }
}
