// solutions/const_generics.rs
use std::fmt::Debug;

fn main() {
    // 1) Разные (T, N) => разные типы, поэтому НЕ могут лежать в одном массиве.
    // Решение: сложить их в tuple (или в enum/trait object).
    let _arrays = (
        Array { data: [1, 2, 3] },
        Array { data: [1.0, 2.0, 3.0] },
        Array { data: [1, 2] },
    );

    // 2) print_array с const N
    let arr = [1, 2, 3];
    print_array(arr);

    let arr = ["hello", "world"];
    print_array(arr);

    // 3) generic_const_exprs (на nightly).
    // ВАЖНО: этот кусок требует nightly из-за feature(generic_const_exprs).
    // Но числа — правильные:
    // &str = 16 байт, String = 24 байта, char = 4 байта (на 64-bit).
    // 16 * 47 = 752 < 768
    // 24 * 31 = 744 < 768
    // 4 * 191 = 764 < 768
    #[cfg(any())]
    {
        check_size(["hello你好"; 47]);
        check_size([(); 31].map(|_| "hello你好".to_string()));
        check_size(['中'; 191]);
    }

    println!("const_generics: OK ✅");
}

struct Array<T, const N: usize> {
    data: [T; N],
}

// Exercise 2
fn print_array<T: Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
}

// ===== Exercise 3 (nightly-only) =====
#[cfg(any())]
mod nightly_only {
    #![allow(incomplete_features)]
    #![feature(generic_const_exprs)]

    pub enum Assert<const CHECK: bool> {}
    pub trait IsTrue {}
    impl IsTrue for Assert<true> {}

    pub fn check_size<T>(val: T)
    where
        Assert<{ core::mem::size_of::<T>() < 768 }>: IsTrue,
    {
        let _ = val;
    }
}
#[cfg(any())]
use nightly_only::check_size;
