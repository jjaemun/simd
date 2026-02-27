#![feature(portable_simd)]

use std::simd::{SimdElement};

use simd::packet::cmp::*;
use simd::packet::ops::*;
use simd::packet::*;
use std::ops::Mul;


fn splat<T, const N: usize>(scalar: T) -> Packet<T, N> 
where T: SimdElement,
{
    Packet::<T, N>::splat(scalar)
}

macro_rules! splat {
    ($scalar:expr) => {
        splat($scalar)
    };
}

fn main() {
    let a = Packet4::<f32>::from_array([1.0, 2.0, 3.0, 4.0]);
    let b = Packet4::<f32>::from_array([1.0, 2.0, 3.0, 4.0]);

    let c = a + b;
    let d = -c;
    let e = -a + b;
    let f = -a -b * 2.0f32;
    let r = splat(2.0) + a * 3.0 - splat!(4.0) * b + (a - 1.0) * (b + 2.0) - (splat!(-3.0) + a * b) + 5.0;

    println!("a = {:?}", a);
    println!("b = {:?}", b);
    println!("c = a + b = {:?}", c);
    println!("d = -c = {:?}", d);
    println!("e = -a + b = {:?}", e);
    println!("f = -a - b = {:?}", f);

    println!("lane-wise operaitons -----------");
    println!("\n");

    println!("a == b : {:?}", a.cmpeq(b));
    println!("a != d : {:?}", a.cmpne(d));
    println!("a <  c : {:?}", a.cmplt(c));
    println!("d <= c : {:?}", d.cmple(c));
    println!("f >  e : {:?}", f.cmpgt(e));
    println!("f >= b : {:?}", f.cmpge(b));

    let g = Packet8::<i32>::from_array([1, 2, 3, 4, 5, 6, 7, 8]);
    let h = Packet8::<i32>::from_array([8, 7, 6, 5, 4, 3, 2, 1]);
    let p = Packet8::<u32>::from_array([8, 7, 6, 5, 4, 3, 2, 1]);

    let q = 2 * p;

    println!("min(g, h) = {:?}", simd::PacketOrd::min(g, h));
    println!("max(g, h) = {:?}", simd::PacketOrd::max(g, h));
    println!("q = {:?}", q);

    let hi = Packet8::<i32>::from_array([10, 4, 4, 5, 6, 5, 6, 6]);
    let lo = Packet8::<i32>::from_array([4, 0, 0, 0, 0, 0, 0, 0]);

    println!("clamp(g, lo, hi) = {:?}", simd::PacketOrd::clamp(g, lo, hi));
}
