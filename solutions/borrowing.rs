fn main() {
    // 1. Immutable borrow
    let s = String::from("hello");
    let len = calculate_length(&s);
    println!("Length of '{}' is {}", s, len);

    // 2. Mutable borrow
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);

    // 3. Multiple immutable borrows
    let s = String::from("rust");
    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);

    // 4. Mutable + immutable (scope)
    let mut s = String::from("ownership");
    {
        let r = &mut s;
        r.push_str(" rules");
    }
    println!("{}", s);

    // 5. Dangling reference (fixed)
    let s = no_dangle();
    println!("{}", s);

    println!("Borrowing OK ✅");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(" world");
}

fn no_dangle() -> String {
    String::from("no dangle")
}
