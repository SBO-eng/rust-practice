fn main() {
    // call functions
    assert_eq!(add(2, 3), 5);
    hello();
    println!("Functions OK ✅");
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn hello() {
    println!("Hello from a function!");
}
