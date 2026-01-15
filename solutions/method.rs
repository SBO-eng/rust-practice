fn main() {
    ex1_rectangle_area();
    ex2_self_vs_ref_self();
    ex3_self_alias_and_mut_self();
    ex4_associated_function_new();
    ex5_multiple_impl_blocks();
    ex6_enum_methods();

    println!("method: ALL OK ✅");
}

/* 1) Complete area() method */
fn ex1_rectangle_area() {
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    let rect1 = Rectangle { width: 30, height: 50 };
    assert_eq!(rect1.area(), 1500);
    println!("Ex1 OK");
}

/* 2) Fill blanks: don't take ownership, use &self */
fn ex2_self_vs_ref_self() {
    #[derive(Debug)]
    struct TrafficLight {
        color: String,
    }

    impl TrafficLight {
        pub fn show_state(&self) {
            println!("the current state is {}", self.color);
        }
    }

    let light = TrafficLight {
        color: "red".to_owned(),
    };

    light.show_state(); // не забираем ownership
    println!("{:?}", light);

    println!("Ex2 OK");
}

/* 3) Use Self + then &mut self without using Self variants */
fn ex3_self_alias_and_mut_self() {
    struct TrafficLight {
        color: String,
    }

    impl TrafficLight {
        // Using `Self`
        pub fn show_state(self: &Self) {
            println!("the current state is {}", self.color);
        }

        // DON'T use any variants of `Self` here
        pub fn change_state(&mut self) {
            self.color = "green".to_string()
        }
    }

    let mut light = TrafficLight {
        color: "red".to_string(),
    };
    light.show_state();
    light.change_state();
    assert_eq!(light.color, "green");

    println!("Ex3 OK");
}

/* 4) Associated function new() must use Self and return red */
fn ex4_associated_function_new() {
    #[derive(Debug)]
    struct TrafficLight {
        color: String,
    }

    impl TrafficLight {
        // Must use `Self`, don't use `TrafficLight`
        pub fn new() -> Self {
            Self {
                color: "red".to_string(),
            }
        }

        pub fn get_state(&self) -> &str {
            &self.color
        }
    }

    let light = TrafficLight::new();
    assert_eq!(light.get_state(), "red");
    println!("Ex4 OK");
}

/* 5) Multiple impl blocks */
fn ex5_multiple_impl_blocks() {
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    let r1 = Rectangle { width: 8, height: 7 };
    let r2 = Rectangle { width: 5, height: 6 };

    assert_eq!(r1.area(), 56);
    assert!(r1.can_hold(&r2));

    println!("Ex5 OK");
}

/* 6) Implement method for enum */
fn ex6_enum_methods() {
    #[derive(Debug)]
    enum TrafficLightColor {
        Red,
        Yellow,
        Green,
    }

    impl TrafficLightColor {
        fn color(&self) -> &'static str {
            match self {
                TrafficLightColor::Red => "red",
                TrafficLightColor::Yellow => "yellow",
                TrafficLightColor::Green => "green",
            }
        }
    }

    let c = TrafficLightColor::Yellow;
    assert_eq!(c.color(), "yellow");
    println!("{:?}", c);

    println!("Ex6 OK");
}
