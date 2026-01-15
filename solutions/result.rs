use std::fs::File;
use std::io::{self, Read};
use std::num::ParseIntError;
use std::str::FromStr;

fn main() {
    ex1_result_basics();
    ex2_question_mark();
    ex3_read_file_with_question();
    ex4_map_or_and_then();
    ex5_and_then_plus_map();
    ex6_type_alias();

    println!("result and ?: ALL OK ✅");
}

// 1) FILL blanks + FIX errors :contentReference[oaicite:6]{index=6}
fn ex1_result_basics() {
    fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
        let n1 = n1_str.parse::<i32>()?;
        let n2 = n2_str.parse::<i32>()?;
        Ok(n1 * n2)
    }

    let result = multiply("10", "2");
    assert_eq!(result, Ok(20));

    let result = multiply("t", "2");
    assert_eq!(result.unwrap_or(8), 8);

    println!("Success!");
}

// 2) IMPLEMENT multiply with ? (DON'T use unwrap) :contentReference[oaicite:7]{index=7}
fn ex2_question_mark() {
    fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
        let n1 = n1_str.parse::<i32>()?;
        let n2 = n2_str.parse::<i32>()?;
        Ok(n1 * n2)
    }

    assert_eq!(multiply("3", "4").unwrap(), 12);
    println!("Success!");
}

// 3) read_file2: fill blank with ONE line :contentReference[oaicite:8]{index=8}
fn ex3_read_file_with_question() {
    fn read_file1() -> Result<String, io::Error> {
        let f = File::open("hello.txt");
        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut s = String::new();
        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }

    fn read_file2() -> Result<String, io::Error> {
        let mut s = String::new();

        File::open("hello.txt")?.read_to_string(&mut s)?;

        Ok(s)
    }

    assert_eq!(
        read_file1().unwrap_err().to_string(),
        read_file2().unwrap_err().to_string()
    );
    println!("Success!");
}

// 4) map / and_then :contentReference[oaicite:9]{index=9}
fn ex4_map_or_and_then() {
    fn add_two(n_str: &str) -> Result<i32, ParseIntError> {
        n_str.parse::<i32>().map(|n| n + 2)
        // вариант через and_then:
        // n_str.parse::<i32>().and_then(|n| Ok(n + 2))
    }

    assert_eq!(add_two("4").unwrap(), 6);
    println!("Success!");
}

// 5) Use BOTH and_then + map :contentReference[oaicite:10]{index=10}
fn ex5_and_then_plus_map() {
    fn multiply1(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
        n1_str
            .parse::<i32>()
            .and_then(|n1| n2_str.parse::<i32>().map(|n2| n1 * n2))
    }

    fn print(result: Result<i32, ParseIntError>) {
        match result {
            Ok(n) => println!("n is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }

    print(multiply1("10", "2"));
    print(multiply1("t", "2"));
    println!("Success!");
}

// 6) Type alias :contentReference[oaicite:11]{index=11}
fn ex6_type_alias() {
    type Res<T> = Result<T, ParseIntError>;

    fn multiply(first_number_str: &str, second_number_str: &str) -> Res<i32> {
        first_number_str.parse::<i32>().and_then(|first_number| {
            second_number_str
                .parse::<i32>()
                .map(|second_number| first_number * second_number)
        })
    }

    fn print(result: Res<i32>) {
        match result {
            Ok(n) => println!("n is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }

    print(multiply("10", "2"));
    print(multiply("t", "2"));

    // просто чтобы FromStr тоже был “потроган” (не обяз, но ок)
    let _ = i32::from_str("123").unwrap();

    println!("Success!");
}
