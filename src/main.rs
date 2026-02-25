#![feature(portable_simd)]

use simd::packet::cmp::*;
use simd::packet::ops::*;
use simd::packet::*;
use std::ops::Mul;

fn main() {
    let a = Packet4::<f32>::from_array([1.0, 2.0, 3.0, 4.0]);
    let b = Packet4::<f32>::from_array([1.0, 2.0, 3.0, 4.0]);

    let c = a + b;
    let d = -c;
    let e = -a + b;
    let f = -a + b;

    println!("a = {:?}", a);
    println!("b = {:?}", b);
    println!("c = {:?}", c);
    println!("d = {:?}", d);
    println!("e = {:?}", e);
    println!("f = {:?}", f);

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

    println!("min(g, h) = {:?}", simd::PacketOrd::min(g, h));
    println!("max(g, h) = {:?}", simd::PacketOrd::max(g, h));

    let hi = Packet8::<i32>::from_array([10, 4, 4, 5, 6, 5, 6, 6]);
    let lo = Packet8::<i32>::from_array([4, 0, 0, 0, 0, 0, 0, 0]);

    println!("clamp(g, lo, hi) = {:?}", simd::PacketOrd::clamp(g, lo, hi));
}
