fn main() {
    // 1) создать String
    let mut s = String::from("Hello");
    s.push(',');
    s.push_str(" Rust");
    assert_eq!(s, "Hello, Rust");

    // 2) &str -> String
    let s2: String = "abc".to_string();
    assert_eq!(s2, "abc");

    // 3) конкатенация
    let a = String::from("foo");
    let b = String::from("bar");
    let c = a + &b; // a moved, b borrowed
    assert_eq!(c, "foobar");

    // 4) format! не двигает владение
    let x = String::from("A");
    let y = String::from("B");
    let z = format!("{}{}", x, y);
    assert_eq!(z, "AB");
    assert_eq!(x, "A");
    assert_eq!(y, "B");

    // 5) срез строки (осторожно с UTF-8)
    let hello = String::from("hello");
    let sub = &hello[0..2];
    assert_eq!(sub, "he");

    println!("compound_string OK ✅");
}
