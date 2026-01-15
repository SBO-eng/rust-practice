fn main() {
    // 1) массив фиксированной длины
    let a = [1, 2, 3, 4, 5];
    assert_eq!(a.len(), 5);
    assert_eq!(a[0], 1);

    // 2) явный тип и длина
    let b: [i32; 3] = [10, 20, 30];
    assert_eq!(b[2], 30);

    // 3) заполнить одним значением
    let c = [0u8; 4];
    assert_eq!(c, [0, 0, 0, 0]);

    // 4) итерация
    let mut sum = 0;
    for v in a {
        sum += v;
    }
    assert_eq!(sum, 15);

    println!("compound_array OK ✅");
}
