#!feature[(portable_simd)]


use std::simd::{*};


pub trait TriviallyCopyableSimdType : Copy + std::simd::SimdElement {}
impl<T: Copy + std::simd::SimdElement> TrivialSimdType for T {}


pub trait FloatingSimdType : TriviallyCopyableSimdType + PartialEq {}
