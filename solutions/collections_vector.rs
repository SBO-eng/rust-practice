fn main() {
    // 1)
    {
        let arr: [u8; 3] = [1, 2, 3];

        let v = Vec::from(arr);
        is_vec(&v);

        let v = vec![1, 2, 3];
        is_vec(&v);

        let v = vec!(1, 2, 3);
        is_vec(&v);

        // rewrite v1: Vec<u8> (not Vec<[u8;3]>)
        let mut v1 = Vec::new();
        for i in arr {
            v1.push(i);
        }
        is_vec(&v1);

        assert_eq!(v, v1);
        println!("Success!");
    }

    // 2) extend
    {
        let mut v1 = Vec::from([1, 2, 4]);
        v1.pop();
        v1.push(3);

        let mut v2 = Vec::new();
        v2.extend([1, 2, 3]);

        assert_eq!(v1, v2);

        println!("Success!");
    }

    // 3) Turn X Into Vec
    {
        let arr = [1, 2, 3];
        let v1 = Vec::from(arr);
        let v2: Vec<i32> = arr.to_vec();

        assert_eq!(v1, v2);

        let s = "hello".to_string();
        let v1: Vec<u8> = s.into();

        let s = "hello".to_string();
        let v2 = s.into_bytes();
        assert_eq!(v1, v2);

        let s = "hello";
        let v3 = Vec::from(s);
        assert_eq!(v2, v3);

        let v4: Vec<i32> = [0; 10].into_iter().collect();
        assert_eq!(v4, vec![0; 10]);

        println!("Success!");
    }

    // 4) Indexing (no panic + implement)
    {
        let mut v = Vec::from([1, 2, 3]);
        for i in 0..5 {
            println!("{:?}", v.get(i));
        }

        for i in 0..5 {
            if i < v.len() {
                v[i] += 1;
            } else {
                v.push(i as i32 + 2);
            }
        }

        assert_eq!(v, vec![2, 3, 4, 5, 6]);
        println!("Success!");
    }

    // 5) Slicing (fix bounds + mut slice)
    {
        let mut v = vec![1, 2, 3];
        let slice1 = &v[..];
        let slice2 = &v[0..v.len()];

        assert_eq!(slice1, slice2);

        let vec_ref: &mut Vec<i32> = &mut v;
        (*vec_ref).push(4);

        let slice3 = &mut v[0..4];
        slice3[3] = 42;

        assert_eq!(slice3, &[1, 2, 3, 42]);
        assert_eq!(v, vec![1, 2, 3, 42]);

        println!("Success!");
    }

    // 6) Capacity
    {
        let mut vec = Vec::with_capacity(10);

        assert_eq!(vec.len(), 0);
        assert_eq!(vec.capacity(), 10);

        for i in 0..10 {
            vec.push(i);
        }
        assert_eq!(vec.len(), 10);
        assert_eq!(vec.capacity(), 10);

        vec.push(11);
        assert_eq!(vec.len(), 11);
        assert!(vec.capacity() >= 11);

        let mut vec = Vec::with_capacity(100);
        for i in 0..100 {
            vec.push(i);
        }

        assert_eq!(vec.len(), 100);
        assert_eq!(vec.capacity(), 100);

        println!("Success!");
    }

    // 7) Distinct types via enum
    {
        #[derive(Debug, PartialEq)]
        enum IpAddr {
            V4(String),
            V6(String),
        }

        let v: Vec<IpAddr> = vec![
            IpAddr::V4("127.0.0.1".to_string()),
            IpAddr::V6("::1".to_string()),
        ];

        assert_eq!(v[0], IpAddr::V4("127.0.0.1".to_string()));
        assert_eq!(v[1], IpAddr::V6("::1".to_string()));

        println!("Success!");
    }

    // 8) Distinct types via trait objects
    {
        trait IpAddr {
            fn display(&self);
        }
        struct V4(String);
        impl IpAddr for V4 {
            fn display(&self) {
                println!("ipv4: {:?}", self.0)
            }
        }
        struct V6(String);
        impl IpAddr for V6 {
            fn display(&self) {
                println!("ipv6: {:?}", self.0)
            }
        }

        let v: Vec<Box<dyn IpAddr>> = vec![
            Box::new(V4("127.0.0.1".to_string())),
            Box::new(V6("::1".to_string())),
        ];

        for ip in v {
            ip.display();
        }
    }
}

fn is_vec(_v: &Vec<u8>) {}
