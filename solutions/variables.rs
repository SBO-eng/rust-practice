fn main() {
    task1_init();
    task2_mutability();
    task3_scope();
    task4_define_x();
    task5_shadowing_asserts();
    task6_remove_line_shadowing();
    task7_unused_vars();
    task8_destructuring_tuple();
    task9_destructuring_assignment();

    println!("Variables: ALL OK ✅");
}

/* 1) A variable can be used only if it has been initialized. */
fn task1_init() {
    let x: i32 = 5; // инициализировали
    let _y: i32; // оставили как в задании (будет warning, если без _)

    assert_eq!(x, 5);
    println!("Task 1 OK");
}

/* 2) Use `mut` to mark a variable as mutable. */
fn task2_mutability() {
    let mut x = 1;
    x += 2;

    assert_eq!(x, 3);
    println!("Task 2 OK");
}

/* 3) Scope */
fn task3_scope() {
    let x: i32 = 10;
    let y: i32 = 5; // переносим y наружу, чтобы был виден после блока

    {
        println!("Inner scope value of x is {} and value of y is {}", x, y);
    }
    println!("Outer scope value of x is {} and value of y is {}", x, y);

    println!("Task 3 OK");
}

/* 4) Fix the error with the use of define_x */
fn task4_define_x() {
    let x = define_x();
    println!("{}, world", x);

    println!("Task 4 OK");
}

fn define_x() -> &'static str {
    "hello"
}

/* 5) Only modify assert_eq! */
fn task5_shadowing_asserts() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x = 42;
    assert_eq!(x, 42);
    println!("{}", x);

    println!("Task 5 OK");
}

/* 6) Remove a line in the code to make it compile
   В оригинале ломается из-за `let x = x;` (после него x становится НЕ mut).
*/
fn task6_remove_line_shadowing() {
    let mut x: i32 = 1;
    x = 7;

    // let x = x; // <- эту строку “удаляем”
    x += 3;

    let y = 4;
    let y = "I can also be bound to text!";

    let _ = (x, y);
    println!("Success!");

    println!("Task 6 OK");
}

/* 7) Fix unused variable warning (покажу 2 способа) */
fn task7_unused_vars() {
    // Способ 1 (самый частый): имя с подчёркиванием
    let _x = 1;

    // Способ 2: “использовать” через let _
    let x2 = 1;
    let _ = x2;

    println!("Task 7 OK");
}

/* 8) Destructuring tuple */
fn task8_destructuring_tuple() {
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Task 8 OK");
}

/* 9) Destructuring assignments */
fn task9_destructuring_assignment() {
    let (x, y);
    (x, ..) = (3, 4);
    [.., y] = [1, 2];

    assert_eq!([x, y], [3, 2]);
    println!("Success!");

    println!("Task 9 OK");
}

