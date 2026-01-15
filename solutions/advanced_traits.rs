// solutions/advanced_traits.rs
use std::fmt;
use std::ops::Sub;

fn main() {
    ex1_associated_types();
    ex2_default_generic_params();
    ex3_fully_qualified_syntax();
    ex4_supertraits();
    ex5_orphan_rule_newtype();

    println!("advanced_traits: OK ✅");
}

// 1) Associated types: re-implement Contains
fn ex1_associated_types() {
    struct Container(i32, i32);

    trait Contains {
        type A;
        type B;
        fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
        fn first(&self) -> i32;
        fn last(&self) -> i32;
    }

    impl Contains for Container {
        type A = i32;
        type B = i32;

        fn contains(&self, a: &i32, b: &i32) -> bool {
            &self.0 == a && &self.1 == b
        }
        fn first(&self) -> i32 {
            self.0
        }
        fn last(&self) -> i32 {
            self.1
        }
    }

    fn difference<C: Contains>(container: &C) -> i32 {
        container.last() - container.first()
    }

    let number_1 = 3;
    let number_2 = 10;
    let container = Container(number_1, number_2);

    assert!(container.contains(&number_1, &number_2));
    assert_eq!(difference(&container), 7);
}

// 2) Default generic type parameters (Sub)
fn ex2_default_generic_params() {
    #[derive(Debug, PartialEq)]
    struct Point<T> {
        x: T,
        y: T,
    }

    impl<T: Sub<Output = T>> Sub for Point<T> {
        type Output = Self;

        fn sub(self, other: Self) -> Self::Output {
            Point {
                x: self.x - other.x,
                y: self.y - other.y,
            }
        }
    }

    assert_eq!(
        Point { x: 2, y: 3 } - Point { x: 1, y: 0 },
        Point { x: 1, y: 3 }
    );
}

// 3) Fully qualified syntax
fn ex3_fully_qualified_syntax() {
    trait Pilot {
        fn fly(&self) -> String;
    }
    trait Wizard {
        fn fly(&self) -> String;
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) -> String {
            "This is your captain speaking.".to_string()
        }
    }
    impl Wizard for Human {
        fn fly(&self) -> String {
            "Up!".to_string()
        }
    }
    impl Human {
        fn fly(&self) -> String {
            "*waving arms furiously*".to_string()
        }
    }

    let person = Human;

    assert_eq!(Pilot::fly(&person), "This is your captain speaking.");
    assert_eq!(Wizard::fly(&person), "Up!");
    assert_eq!(person.fly(), "*waving arms furiously*");
}

// 4) Supertraits
fn ex4_supertraits() {
    trait Person {
        fn name(&self) -> String;
    }
    trait Student: Person {
        fn university(&self) -> String;
    }
    trait Programmer {
        fn fav_language(&self) -> String;
    }
    trait CompSciStudent: Programmer + Student {
        fn git_username(&self) -> String;
    }

    fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
        format!(
            "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
            student.name(),
            student.university(),
            student.fav_language(),
            student.git_username()
        )
    }

    struct CSStudent {
        name: String,
        university: String,
        fav_language: String,
        git_username: String,
    }

    impl Person for CSStudent {
        fn name(&self) -> String {
            self.name.clone()
        }
    }
    impl Student for CSStudent {
        fn university(&self) -> String {
            self.university.clone()
        }
    }
    impl Programmer for CSStudent {
        fn fav_language(&self) -> String {
            self.fav_language.clone()
        }
    }
    impl CompSciStudent for CSStudent {
        fn git_username(&self) -> String {
            self.git_username.clone()
        }
    }

    let student = CSStudent {
        name: "Sunfei".to_string(),
        university: "XXX".to_string(),
        fav_language: "Rust".to_string(),
        git_username: "sunface".to_string(),
    };

    println!("{}", comp_sci_student_greeting(&student));
}

// 5) Orphan rules -> newtype
fn ex5_orphan_rule_newtype() {
    struct Pretty(String);

    impl fmt::Display for Pretty {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "\"{}\"", self.0.clone() + ", world")
        }
    }

    let w = Pretty("hello".to_string());
    println!("w = {}", w);
}
