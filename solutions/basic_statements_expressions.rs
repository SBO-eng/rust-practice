fn main() {
    // Statements
    let x = 5;
    println!("x = {}", x);

    // Expression example
    let y = {
        let a = 3;
        a + 2
    };
    assert_eq!(y, 5);

    println!("Statements & Expressions OK ✅");
}
