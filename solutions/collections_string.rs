use std::mem;

fn main() {
    // 1) Basic operations
    {
        // 1. Don't use `to_string()`
        // 2. Don't add/remove any code line
        let mut s: String = String::from("hello, ");
        s.push_str("world");
        s.push('!');

        move_ownership(&s);

        assert_eq!(s, "hello, world!");

        println!("Success!");
    }

    // 2) String and &str
    {
        let mut s = String::from("hello, world");
        let slice1: &str = &s; // In two ways (можно еще: s.as_str())
        assert_eq!(slice1, "hello, world");

        let slice2 = &s[0..5];
        assert_eq!(slice2, "hello");

        let mut slice3: String = s;
        slice3.push('!');
        assert_eq!(slice3, "hello, world!");

        println!("Success!");
    }

    // 3) Heap allocations question (код уже корректный)
    {
        let s: String = String::from("hello, world!");
        let slice: &str = &s;
        let s: String = slice.to_string();

        assert_eq!(s, "hello, world!");
        println!("Success!");
    }

    // 4) UTF-8 & Indexing
    {
        let s = String::from("hello, 世界");
        let slice1 = &s[0..1]; // 'h' = 1 byte
        assert_eq!(slice1, "h");

        let slice2 = &s[7..10]; // '世' starts at byte 7, len 3 bytes
        assert_eq!(slice2, "世");

        // Iterate through all chars in s (byte index + char)
        for (i, c) in s.char_indices() {
            if i == 7 {
                assert_eq!(c, '世')
            }
        }

        println!("Success!");
    }

    // 5) from_utf8
    {
        let mut s = String::new();
        s.push_str("hello");

        let v = vec![104, 101, 108, 108, 111];
        let s1 = String::from_utf8(v).unwrap();

        assert_eq!(s, s1);

        println!("Success!");
    }

    // 6) Capacity (print 25, 25, 25)
    {
        let mut s = String::with_capacity(25);

        println!("{}", s.capacity());

        for _ in 0..2 {
            s.push_str("hello");
            println!("{}", s.capacity());
        }

        println!("Success!");
    }

    // 7) ManuallyDrop + from_raw_parts
    {
        let story = String::from("Rust By Practice");
        let mut story = mem::ManuallyDrop::new(story);

        let ptr = story.as_mut_ptr();
        let len = story.len();
        let capacity = story.capacity();

        assert_eq!(16, len);

        let s = unsafe { String::from_raw_parts(ptr, len, capacity) };
        assert_eq!(*story, s);

        println!("Success!");
    }
}

fn move_ownership(s: &String) {
    println!("ownership of \"{}\" is moved here!", s)
}
