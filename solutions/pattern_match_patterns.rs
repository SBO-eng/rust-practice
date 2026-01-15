// solutions/pattern_match_patterns.rs

fn main() {
    ex1_or_and_range();
    ex2_at_binding();
    ex3_fix_enum_patterns();
    ex4_match_guard();
    ex5_ignore_with_dots();
    ex6_match_mut_ref();

    println!("patterns: ALL OK ✅");
}

// 1) Use `|` to match several values
fn ex1_or_and_range() {
    fn match_number(n: i32) {
        match n {
            1 => println!("One!"),
            2 | 3 | 4 | 5 => println!("match 2 -> 5"),
            6..=10 => {
                println!("match 6 -> 10")
            }
            _ => {
                println!("match -infinite -> 0 or 11 -> +infinite")
            }
        }
    }

    match_number(1);
    match_number(3);
    match_number(8);
    match_number(100);
}

// 2) `@` binding
fn ex2_at_binding() {
    struct Point {
        x: i32,
        y: i32,
    }

    // чтобы попало во вторую ветку:
    // x в 0..=5, y в (10|20|30)
    let p = Point { x: 0, y: 10 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point {
            x: 0..=5,
            y: y @ (10 | 20 | 30),
        } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

// 3) Fix the errors
fn ex3_fix_enum_patterns() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello { id: id @ 3..=7 } => {
            println!("Found an id in range [3, 7]: {}", id)
        }
        Message::Hello {
            id: newid @ (10 | 11 | 12),
        } => {
            println!("Found an id in another range [10, 12]: {}", newid)
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}

// 4) Match guard (MUST use `split`)
fn ex4_match_guard() {
    let num = Some(4);
    let split = 5;

    match num {
        Some(x) if x < split => assert!(x < split),
        Some(x) => assert!(x >= split),
        None => (),
    }

    println!("Success!");
}

// 5) Ignoring remaining parts with `..`
fn ex5_ignore_with_dots() {
    let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);

    match numbers {
        (first, .., last) => {
            assert_eq!(first, 2);
            assert_eq!(last, 2048);
        }
    }

    println!("Success!");
}

// 6) Match a mutable reference safely (least change, keep all lines)
fn ex6_match_mut_ref() {
    let mut v = String::from("hello,");
    let r = &mut v;

    match r {
        value => value.push_str(" world!"),
    }

    // просто чтобы не было сомнений:
    assert_eq!(v, "hello, world!");
}
