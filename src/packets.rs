#![feature(portable_simd)]

use std::simd::{*};


#[derive(Clone, Copy, Debug)]
pub struct Packet<T, const N: usize> {
    pub v: Simd<T, N>,
}

macro_rules! declare_packet {
    ($name: ident, $ty: ty, $n: expr) => {
        pub type $name = Packet<$ty, $n>;
    };
}


declare_packet!(Packet4i, i32, 4);
declare_packet!(Packet8i, i32, 8);
declare_packet!(Packet4f, f32, 4);
declare_packet!(Packet8f, f32, 8);
