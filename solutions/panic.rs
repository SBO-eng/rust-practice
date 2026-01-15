fn main() {
    ex1_panic_lemonade();
    ex2_fix_common_panics();
    ex3_backtrace_tip();

    println!("panic!: ALL OK ✅");
}

// 1) FILL the blanks (нужно паникнуть на "lemonade") :contentReference[oaicite:2]{index=2}
fn ex1_panic_lemonade() {
    fn drink(beverage: &str) {
        if beverage == "lemonade" {
            println!("Success!");
            panic!("Don't drink too much lemonade!");
        }
        println!("Exercise Failed if printing out this line!");
    }

    // должен вызвать ветку с panic
    // если запустить весь файл, программа упадёт на panic (это нормально для этого упражнения)
    // поэтому мы ловим панику, чтобы остальные упражнения в файле тоже прошли.
    let r = std::panic::catch_unwind(|| drink("lemonade"));
    assert!(r.is_err());
}

// 2) MAKE the code work by fixing all panics :contentReference[oaicite:3]{index=3}
fn ex2_fix_common_panics() {
    // assert bytes: "abc" = [97,98,99]
    assert_eq!("abc".as_bytes(), [97, 98, 99]);

    // index out of bounds
    let v = vec![1, 2, 3];
    let ele = v[2];
    assert_eq!(ele, 3);

    // unwrap on None -> use correct index
    let ele = v.get(2).unwrap();
    assert_eq!(*ele, 3);

    // overflow in speed*cph: use f64 multiplication first
    let _v = production_rate_per_hour(10);
    assert!(_v > 0.0);

    // divide by zero -> don't do that (or check)
    divide(15, 1);

    println!("Success!");
}

fn divide(x: u8, y: u8) {
    if y == 0 {
        println!("cannot divide by zero");
        return;
    }
    println!("{}", x / y)
}

fn production_rate_per_hour(speed: u8) -> f64 {
    let cph: f64 = 221.0;
    match speed {
        1..=4 => (speed as f64) * cph,
        5..=8 => (speed as f64) * cph * 0.9,
        9..=10 => (speed as f64) * cph * 0.77,
        _ => 0.0,
    }
}

// 3) Backtrace tip (это не код, это команда) :contentReference[oaicite:4]{index=4}
fn ex3_backtrace_tip() {
    // Чтобы увидеть полный backtrace при panic:
    // RUST_BACKTRACE=full cargo run
    // (или RUST_BACKTRACE=1 cargo run)
    println!("Backtrace tip: use RUST_BACKTRACE=full cargo run");
}
