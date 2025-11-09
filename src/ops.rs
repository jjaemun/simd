#[!feature(portable_simd)]

use crate::stt::TriviallyCopyableSimdType;
use crate::packets::Packet;

use std::ops::{
    Add,
    Sub,
    Mul,
    Div
};


macro_rules! declare_overloads {
    ($trait: ident, $func: ident, $op: tt) => {
        impl<T, const N: usize> $trait for Packet<T, N> {
            fn $func(self, rhs: Self) -> Self::Packet<T, N> {
                Packet { v: self.v $op rhs.v }
            }
        }
    };
}


declare_overloads!(Add, add, +);
declare_overloads!(Sub, sub, -);
declare_overloads!(Mul, mul, *);
declare_overloads!(Div, div, /);
