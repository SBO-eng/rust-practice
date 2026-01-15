#![allow(overflowing_literals)]

fn main() {
    // 1
    {
        let decimal = 97.123_f32;

        let integer: u8 = decimal as u8;

        let _c1: char = decimal as char; // не важно, просто чтобы компилилось
        let _c2 = integer as char;

        assert_eq!(integer, 'b' as u8);
        println!("as #1 OK");
    }

    // 2
    {
        assert_eq!(u8::MAX, 255);
        let _v = 1000 as u8; // ok (срезается по модулю 256)
        println!("as #2 OK");
    }

    // 3
    {
        assert_eq!(1000 as u16, 1000);
        assert_eq!(1000 as u8, 232);

        assert_eq!(-1_i8 as u8, 255);

        // saturating cast float->int
        assert_eq!(300.1_f32 as u8, 255);
        assert_eq!(-100.1_f32 as u8, 0);

        println!("as #3 OK");
    }

    // 4
    {
        let mut values: [i32; 2] = [1, 2];
        let p1: *mut i32 = values.as_mut_ptr();
        let first_address: usize = p1 as usize;
        let second_address = first_address + 4;
        let p2: *mut i32 = second_address as *mut i32;

        unsafe {
            *p2 += 1;
        }

        assert_eq!(values[1], 3);
        println!("as #4 OK");
    }

    // 5
    {
        let arr: [u64; 13] = [0; 13];
        assert_eq!(std::mem::size_of_val(&arr), 8 * 13);

        let a: *const [u64] = &arr;
        let b = a as *const [u8];

        unsafe {
            assert_eq!(std::mem::size_of_val(&*b), 13);
        }

        println!("as #5 OK");
    }

    println!("typeconv_as: ALL OK ✅");
}
