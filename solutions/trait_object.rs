// solutions/trait_object.rs

fn main() {
    ex1_return_dyn();
    ex2_array_trait_objects();
    ex3_ref_and_box();

    println!("trait_object: OK ✅");
}

// 1) Returning Traits with dyn
fn ex1_return_dyn() {
    trait Bird {
        fn quack(&self) -> String;
    }
    struct Duck;
    impl Duck {
        fn swim(&self) {
            println!("Look, the duck is swimming")
        }
    }
    struct Swan;
    impl Swan {
        fn fly(&self) {
            println!("Look, the duck.. oh sorry, the swan is flying")
        }
    }

    impl Bird for Duck {
        fn quack(&self) -> String {
            "duck duck".to_string()
        }
    }
    impl Bird for Swan {
        fn quack(&self) -> String {
            "swan swan".to_string()
        }
    }

    let duck = Duck;
    duck.swim();

    let bird = hatch_a_bird(2);
    assert_eq!(bird.quack(), "duck duck");

    let bird = hatch_a_bird(1);
    assert_eq!(bird.quack(), "swan swan");

    println!("Success!");

    fn hatch_a_bird(species: u8) -> Box<dyn Bird> {
        match species {
            1 => Box::new(Swan),
            _ => Box::new(Duck),
        }
    }
}

// 2) Array with trait objects
fn ex2_array_trait_objects() {
    trait Bird {
        fn quack(&self);
    }

    struct Duck;
    struct Swan;

    impl Bird for Duck {
        fn quack(&self) {
            println!("duck duck");
        }
    }
    impl Bird for Swan {
        fn quack(&self) {
            println!("swan swan");
        }
    }

    let birds: [Box<dyn Bird>; 2] = [Box::new(Duck), Box::new(Swan)];

    for bird in birds {
        bird.quack();
    }
}

// 3) &dyn and Box<dyn>
fn ex3_ref_and_box() {
    trait Draw {
        fn draw(&self) -> String;
    }

    impl Draw for u8 {
        fn draw(&self) -> String {
            format!("u8: {}", *self)
        }
    }
    impl Draw for f64 {
        fn draw(&self) -> String {
            format!("f64: {}", *self)
        }
    }

    fn draw_with_box(x: Box<dyn Draw>) {
        println!("{}", x.draw());
    }
    fn draw_with_ref(x: &dyn Draw) {
        println!("{}", x.draw());
    }

    let x = 1.1f64;
    let y = 8u8;

    draw_with_box(Box::new(x));
    draw_with_ref(&y);
}

