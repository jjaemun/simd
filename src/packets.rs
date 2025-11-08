#![feature(portable_simd)]

use std::simd::{*};


#[derive(Clone, Copy, Debug)]
pub struct Packet<T, const LANES: usize>(pub Simd<T, LANES>);


macro_rules! declare_packet {
    ($name: ident, $ty: ty, $lanes: expr) => {
        pub type $name = Packet<$ty, $lanes>;
    };
}


declare_packet!(Packet2d, f64, 2);
declare_packet!(Packet4i, i32, 4);
declare_packet!(Packet8i, i32, 8);
declare_packet!(Packet4f, f32, 4);
declare_packet!(Packet8f, f32, 8);
declare_packet!(Packet8d, f64, 8);
