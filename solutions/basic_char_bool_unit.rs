fn main() {
    task1_char_size();
    task2_print_char();
    task3_bool_logic();
    task4_unit_type();

    println!("Char/Bool/Unit OK ✅");
}

fn task1_char_size() {
    let c = '中';
    assert_eq!(std::mem::size_of_val(&c), 4);
}

fn task2_print_char() {
    let c = 'z';
    println!("Char printed: {}", c);
}

fn task3_bool_logic() {
    let t = true;
    let f = false;
    assert!(t && !f);
    assert!(t || f);
}

fn task4_unit_type() {
    let u: () = ();
    println!("Unit value: {:?}", u);
}
