fn main() {
    // 1) slice из массива
    let arr = [1, 2, 3, 4, 5];
    let s: &[i32] = &arr[1..4];
    assert_eq!(s, &[2, 3, 4]);

    // 2) передача slice в функцию
    assert_eq!(sum_slice(&arr[..]), 15);
    assert_eq!(sum_slice(&arr[2..]), 12);

    // 3) slice из Vec
    let v = vec![10, 20, 30];
    let vs = &v[..];
    assert_eq!(vs, &[10, 20, 30]);

    println!("compound_slice OK ✅");
}

fn sum_slice(s: &[i32]) -> i32 {
    s.iter().sum()
}
