#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

#[derive(Debug)]
struct Point(i32, i32); // tuple struct

#[derive(Debug)]
struct Unit; // unit struct

fn main() {
    // 1) обычная структура
    let mut p = Person {
        name: "Alex".to_string(),
        age: 20,
    };
    assert_eq!(p.age, 20);

    // 2) менять поле (нужен mut)
    p.age += 1;
    assert_eq!(p.age, 21);

    // 3) struct update syntax
    let p2 = Person {
        name: "Bob".to_string(),
        ..p // age заберём из p, но p переместится из-за String
    };
    assert_eq!(p2.age, 21);

    // 4) tuple struct
    let pt = Point(3, 4);
    assert_eq!(pt.0, 3);
    assert_eq!(pt.1, 4);

    // 5) unit struct
    let _u = Unit;

    println!("compound_struct OK ✅");
}
