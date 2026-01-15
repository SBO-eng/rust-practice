fn main() {
    ex1_if_else();
    ex2_if_else_expression();
    ex3_for_range();
    ex4_for_names_numbers();
    ex5_for_enumerate();
    ex6_while_fizzbuzz();
    ex7_break();
    ex8_continue_break();
    ex9_loop_continue_break();
    ex10_loop_return_value();
    ex11_labels();

    println!("Flow control: ALL OK ✅");
}

/* 1) If/else: fill blanks */
fn ex1_if_else() {
    let n = 5;

    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }
}

/* 2) If/else expression in assignments: fix errors */
fn ex2_if_else_expression() {
    let n = 5;
    let big_n = if n < 10 && n > -10 {
        println!(", and is a small number, increase ten-fold");
        10 * n
    } else {
        println!(", and is a big number, halve the number");
        n / 2
    };

    println!("{} -> {}", n, big_n);
}

/* 3) For range: never let 100 run */
fn ex3_for_range() {
    for n in 1..100 {
        if n == 100 {
            panic!("NEVER LET THIS RUN");
        }
    }
    println!("Success!");
}

/* 4) Fix without adding/removing lines (avoid moving Strings) */
fn ex4_for_names_numbers() {
    let names = [String::from("liming"), String::from("hanmeimei")];
    for name in names.iter() {
        let _ = name; // "используем" ссылку
    }

    println!("{:?}", names);

    let numbers = [1, 2, 3];
    for n in numbers {
        let _ = n;
    }

    println!("{:?}", numbers);
}

/* 5) Iterate index + value */
fn ex5_for_enumerate() {
    let a = [4, 3, 2, 1];

    for (i, v) in a.iter().enumerate() {
        println!("The {}th element is {}", i + 1, v);
    }
}

/* 6) While: fill blanks */
fn ex6_while_fizzbuzz() {
    let mut n = 1;
    while n < 10 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        n += 1;
    }

    println!("n reached {}, so loop is over", n);
}

/* 7) break */
fn ex7_break() {
    let mut n = 0;
    for _i in 0..=100 {
        if n == 66 {
            break;
        }
        n += 1;
    }

    assert_eq!(n, 66);
    println!("Success!");
}

/* 8) continue */
fn ex8_continue_break() {
    let mut n = 0;
    for _i in 0..=100 {
        if n != 66 {
            n += 1;
            continue;
        }

        break;
    }

    assert_eq!(n, 66);
    println!("Success!");
}

/* 9) loop + continue/break */
fn ex9_loop_continue_break() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    loop {
        count += 1;

        if count == 3 {
            println!("three");
            continue;
        }

        println!("{}", count);
        if count == 5 {
            println!("OK, that's enough");
            break;
        }
    }

    assert_eq!(count, 5);
    println!("Success!");
}

/* 10) loop returns a value */
fn ex10_loop_return_value() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
    println!("Success!");
}

/* 11) labels */
fn ex11_labels() {
    let mut count = 0;
    'outer: loop {
        'inner1: loop {
            if count >= 20 {
                break 'inner1;
            }
            count += 2;
        }

        count += 5;
        'inner2: loop {
            if count >= 30 {
                break 'outer;
            }
            continue 'outer;
        }
    }

    assert!(count == 30);
    println!("Success!");
}
