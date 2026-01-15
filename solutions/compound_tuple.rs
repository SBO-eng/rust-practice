fn main() {
    // 1) создать кортеж
    let t: (i32, f64, char) = (1, 2.5, 'a');
    assert_eq!(t.0, 1);
    assert_eq!(t.1, 2.5);
    assert_eq!(t.2, 'a');

    // 2) деструктуризация
    let (x, y, z) = t;
    assert_eq!(x, 1);
    assert_eq!(y, 2.5);
    assert_eq!(z, 'a');

    // 3) вложенные кортежи
    let nested = ((1, 2), (3, 4));
    assert_eq!((nested.0).1, 2);

    // 4) unit tuple / unit type
    let u: () = ();
    let _t1 = (u, 1);

    println!("compound_tuple OK ✅");
}
