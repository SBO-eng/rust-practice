fn main() {
    // 1. Ownership move
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s2);

    // 2. Clone
    let s3 = String::from("world");
    let s4 = s3.clone();
    println!("{}, {}", s3, s4);

    // 3. Copy types
    let x = 5;
    let y = x;
    println!("{}, {}", x, y);

    // 4. Ownership and functions
    let s = String::from("rust");
    takes_ownership(s);

    let x = 10;
    makes_copy(x);
    println!("{}", x);

    // 5. Return ownership
    let s = gives_ownership();
    println!("{}", s);

    let s = String::from("back");
    let s = takes_and_gives_back(s);
    println!("{}", s);

    println!("Ownership OK ✅");
}

fn takes_ownership(s: String) {
    println!("{}", s);
}

fn makes_copy(x: i32) {
    println!("{}", x);
}

fn gives_ownership() -> String {
    String::from("hello")
}

fn takes_and_gives_back(s: String) -> String {
    s
}
