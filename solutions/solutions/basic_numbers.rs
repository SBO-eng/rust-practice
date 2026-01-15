fn main() {
    // 1) cast
    let _v: u16 = 38_u8 as u16;

    // 2) type_of
    let x = 5;
    assert_eq!(type_of(&x), "i32".to_string());

    // 3) max values
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);

    // 4) overflow without panic
    let v1 = 251_u8.wrapping_add(8);
    let (v2, _) = 251_u8.overflowing_add(8);
    println!("{v1}, {v2}");

    // 5) literals sum
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1597);

    // 6) float 0.1+0.2
    let f = 0.1_f64 + 0.2_f64;
    assert!((f - 0.3).abs() < 1e-10);

    println!("Numbers OK ✅");
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
