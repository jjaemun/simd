use simd::packet::packet::*;

fn bytes<T>(src: &T) -> &[u8] {
    
    // Returns byte slice representation of any object.

    unsafe {
        std::slice::from_raw_parts(
            src as *const T as *const u8, std::mem::size_of::<T>()
        )
    }
}

#[test]
fn memtest() {
   
    // Verifies byte-wise identity preservation from Simd. 

    macro_rules! memory {
        ($(($type:ty, $lanes:expr))*) => {
            $(
                let packet = Packet::<$type, $lanes>::default();
                let simd = std::simd::Simd::<$type, $lanes>::default();

                assert_eq!(bytes(&packet), bytes(&simd));
            )*   
        };
    }

    memory! {
        (u8, 2)
        (u8, 4)
        (u8, 8)
        (u8, 16)

        (u16, 2)
        (u16, 4)
        (u16, 8)
        (u16, 16)

        (u32, 2)
        (u32, 4)
        (u32, 8)
        (u32, 16)

        (u64, 2)
        (u64, 4)
        (u64, 8)
    
        (i8, 2)
        (i8, 4)
        (i8, 8)
        (i8, 16)
    
        (i16, 2)
        (i16, 4)
        (i16, 8)
        (i16, 16)

        (i32, 2)
        (i32, 4)
        (i32, 8)
        (i32, 16)
    
        (i64, 2)
        (i64, 4)
        (i64, 8)
    
        (f32, 2)
        (f32, 4)
        (f32, 8)
        (f32, 16)
    
        (f64, 2)
        (f64, 4)
        (f64, 8)    
    }
}
