#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

fn main() {
    // 1) создать enum
    let m1 = Message::Quit;
    let m2 = Message::Move { x: 10, y: 20 };
    let m3 = Message::Write("hi".to_string());
    let m4 = Message::ChangeColor(255, 0, 0);

    // 2) match
    assert_eq!(handle(m1), "quit");
    assert_eq!(handle(m2), "move to 10 20");
    assert_eq!(handle(m3), "write hi");
    assert_eq!(handle(m4), "color 255 0 0");

    // 3) Option (часто рядом с enum)
    let x: Option<i32> = Some(5);
    assert_eq!(x.unwrap(), 5);

    println!("compound_enum OK ✅");
}

fn handle(m: Message) -> String {
    match m {
        Message::Quit => "quit".to_string(),
        Message::Move { x, y } => format!("move to {} {}", x, y),
        Message::Write(s) => format!("write {}", s),
        Message::ChangeColor(r, g, b) => format!("color {} {} {}", r, g, b),
    }
}
