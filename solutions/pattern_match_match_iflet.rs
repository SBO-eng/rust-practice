// solutions/pattern_match_match_iflet.rs

fn main() {
    ex1_direction_match();
    ex2_match_expression_assignment();
    ex3_match_enum_data();
    ex4_matches_macro_chars();
    ex5_matches_instead_of_eq();
    ex6_if_let_instead_of_match();
    ex7_if_let_bind_value();
    ex8_match_instead_of_if_let_chain();
    ex9_shadowing();

    println!("match-iflet: ALL OK ✅");
}

// 1) Fill the blanks (Direction)
fn ex1_direction_match() {
    enum Direction {
        East,
        West,
        North,
        South,
    }

    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::South | Direction::North => {
            println!("South or North");
        }
        _ => println!("West"),
    };
}

// 2) Match is an expression
fn ex2_match_expression_assignment() {
    let boolean = true;

    let binary = match boolean {
        true => 1,
        false => 0,
    };

    assert_eq!(binary, 1);
    println!("Success!");
}

// 3) Using match to get data from enum variants
fn ex3_match_enum_data() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msgs = [
        Message::Quit,
        Message::Move { x: 1, y: 3 },
        Message::ChangeColor(255, 255, 0),
    ];

    for msg in msgs {
        show_message(msg)
    }

    println!("Success!");

    fn show_message(msg: Message) {
        match msg {
            Message::Move { x: a, y: b } => {
                assert_eq!(a, 1);
                assert_eq!(b, 3);
            }
            Message::ChangeColor(_, g, b) => {
                assert_eq!(g, 255);
                assert_eq!(b, 0);
            }
            _ => println!("no data in these variants"),
        }
    }
}

// 4) matches! macro
fn ex4_matches_macro_chars() {
    let alphabets = ['a', 'E', 'Z', '0', 'x', '9', 'Y'];

    for ab in alphabets {
        assert!(matches!(
            ab,
            'a'..='z' | 'A'..='Z' | '0'..='9'
        ));
    }

    println!("Success!");
}

// 5) Fix error by changing only one line (no PartialEq)
fn ex5_matches_instead_of_eq() {
    enum MyEnum {
        Foo,
        Bar,
    }

    let mut count = 0;
    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    for e in v {
        if matches!(e, MyEnum::Foo) {
            count += 1;
        }
    }

    assert_eq!(count, 2);
    println!("Success!");
}

// 6) Replace match with if let
fn ex6_if_let_instead_of_match() {
    let o = Some(7);

    if let Some(i) = o {
        println!("This is a really long string and `{:?}`", i);
        println!("Success!");
    }
}

// 7) Fill in the blank (if let binding)
fn ex7_if_let_bind_value() {
    enum Foo {
        Bar(u8),
    }
    let a = Foo::Bar(1);

    if let Foo::Bar(i) = a {
        println!("foobar holds the value: {}", i);
        println!("Success!");
    }
}

// 8) Replace if let chain with match
fn ex8_match_instead_of_if_let_chain() {
    enum Foo {
        Bar,
        Baz,
        Qux(u32),
    }

    let a = Foo::Qux(10);

    match a {
        Foo::Bar => println!("match foo::bar"),
        Foo::Baz => println!("match foo::baz"),
        _ => println!("match others"),
    }
}

// 9) Shadowing fix
fn ex9_shadowing() {
    let age = Some(30);

    if let Some(age) = age {
        // тут age уже i32, не Option<i32>
        assert_eq!(age, 30);
    }

    match age {
        Some(age) => println!("age is a new variable, it's value is {}", age),
        _ => (),
    }
}
