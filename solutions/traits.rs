// solutions/traits.rs
use std::ops::Add;

fn main() {
    // 5) Summary + function param as impl Trait
    let post = Post {
        title: "Popular Rust".to_string(),
        author: "Sunface".to_string(),
        content: "Rust is awesome!".to_string(),
    };
    let weibo = Weibo {
        username: "sunface".to_string(),
        content: "Weibo seems to be worse than Tweet".to_string(),
    };
    summary(&post);
    summary(&weibo);
    println!("{:?}", post);
    println!("{:?}", weibo);

    // 6) Return different types -> use trait object
    let a = random_animal(0.234);
    println!("{}", a.noise());
    let b = random_animal(0.9);
    println!("{}", b.noise());

    // 7) trait bound sum
    assert_eq!(sum(1, 2), 3);

    // 8) Pair cmp_display with Unit
    let pair = Pair { x: Unit(1), y: Unit(3) };
    pair.cmp_display();

    // 9) Cacher
    example_cacher();

    println!("traits: OK ✅");
}

// ===== Exercise 5 =====
trait Summary {
    fn summarize(&self) -> String;
}

#[derive(Debug)]
struct Post {
    title: String,
    author: String,
    content: String,
}
impl Summary for Post {
    fn summarize(&self) -> String {
        format!("The author of post {} is {}", self.title, self.author)
    }
}

#[derive(Debug)]
struct Weibo {
    username: String,
    content: String,
}
impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{} published a weibo {}", self.username, self.content)
    }
}

fn summary(item: &impl Summary) {
    println!("{}", item.summarize());
}

// ===== Exercise 6 =====
struct Sheep;
struct Cow;

trait Animal {
    fn noise(&self) -> String;
}
impl Animal for Sheep {
    fn noise(&self) -> String {
        "baaaaah!".to_string()
    }
}
impl Animal for Cow {
    fn noise(&self) -> String {
        "moooooo!".to_string()
    }
}

fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep)
    } else {
        Box::new(Cow)
    }
}

// ===== Exercise 7 =====
fn sum<T>(x: T, y: T) -> T
where
    T: Add<Output = T>,
{
    x + y
}

// ===== Exercise 8 =====
struct Pair<T> {
    x: T,
    y: T,
}
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
impl<T: std::fmt::Debug + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {:?}", self.x);
        } else {
            println!("The largest member is y = {:?}", self.y);
        }
    }
}
#[derive(Debug, PartialEq, PartialOrd)]
struct Unit(i32);

// ===== Exercise 9 =====
fn example_cacher() {
    struct Cacher<T: Fn(u32) -> u32> {
        calculation: T,
        value: Option<u32>,
    }
    impl<T: Fn(u32) -> u32> Cacher<T> {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None,
            }
        }
        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                }
            }
        }
    }

    let mut cacher = Cacher::new(|x| x + 1);
    assert_eq!(cacher.value(10), 11);
    // Это специально показывает “кеширование” (возвращает прошлое значение)
    assert_eq!(cacher.value(15), 11);
}
