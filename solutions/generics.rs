// solutions/generics.rs
use std::ops::Add;

fn main() {
    // 1) Fill blanks
    struct A;
    struct S(A);
    struct SGen<T>(T);

    fn reg_fn(_s: S) {}
    fn gen_spec_t(_s: SGen<A>) {}
    fn gen_spec_i32(_s: SGen<i32>) {}
    fn generic<T>(_s: SGen<T>) {}

    reg_fn(S(A));
    gen_spec_t(SGen(A));
    gen_spec_i32(SGen(7_i32));
    generic::<char>(SGen('a'));
    generic(SGen('b'));

    // 2) Implement generic sum
    assert_eq!(5, sum(2i8, 3i8));
    assert_eq!(50, sum(20, 30));
    assert_eq!(2.46, sum(1.23_f64, 1.23_f64));

    // 3) Implement Point<T>
    let _integer = Point { x: 5, y: 10 };
    let _float = Point { x: 1.0, y: 4.0 };

    // 4) Modify Point to allow different types for x/y
    let _p = Point2 { x: 5, y: "hello".to_string() };

    // 5) Make Val generic
    let x = Val { val: 3.0 };
    let y = Val { val: "hello".to_string() };
    println!("{}, {}", x.value(), y.value());

    // 6) Implement mixup
    let p1 = PointMix { x: 5, y: 10 };
    let p2 = PointMix { x: "Hello", y: '中' };
    let p3 = p1.mixup(p2);
    assert_eq!(p3.x, 5);
    assert_eq!(p3.y, '中');

    // 7) Fix Point<f32> method usage
    let p = PointF32 { x: 5.0_f32, y: 10.0_f32 };
    println!("{}", p.distance_from_origin());

    println!("generics: ALL OK ✅");
}

// Exercise 2
fn sum<T>(a: T, b: T) -> T
where
    T: Add<Output = T>,
{
    a + b
}

// Exercise 3
#[allow(dead_code)]
struct Point<T> {
    x: T,
    y: T,
}

// Exercise 4
#[allow(dead_code)]
struct Point2<T, U> {
    x: T,
    y: U,
}

// Exercise 5
struct Val<T> {
    val: T,
}
impl<T> Val<T> {
    fn value(&self) -> &T {
        &self.val
    }
}

// Exercise 6
struct PointMix<T, U> {
    x: T,
    y: U,
}
impl<T, U> PointMix<T, U> {
    fn mixup<V, W>(self, other: PointMix<V, W>) -> PointMix<T, W> {
        PointMix {
            x: self.x,
            y: other.y,
        }
    }
}

// Exercise 7
struct PointF32 {
    x: f32,
    y: f32,
}
impl PointF32 {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
