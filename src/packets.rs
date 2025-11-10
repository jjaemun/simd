#![feature(portable_simd)]

use std::simd::*;


#[derive(Clone, Copy, Debug)]
pub struct Packet<T, const N: usize> {
    pub v: Simd<T, N>,
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
