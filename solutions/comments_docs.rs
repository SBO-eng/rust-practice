//! This is crate-level documentation.
//!
//! It describes what this crate is about.
//!
//! You can generate HTML docs with:
//! `cargo doc --open`

/// This is a struct-level documentation comment.
///
/// `Person` represents a human with a name and age.
#[derive(Debug)]
struct Person {
    /// The person's name.
    name: String,
    /// The person's age.
    age: u8,
}

impl Person {
    /// Creates a new [`Person`].
    ///
    /// # Examples
    ///
    /// ```
    /// let p = Person::new("Alice", 20);
    /// assert_eq!(p.age, 20);
    /// ```
    pub fn new(name: &str, age: u8) -> Self {
        Self {
            name: name.to_string(),
            age,
        }
    }

    /// Returns a greeting string.
    ///
    /// # Panics
    ///
    /// This function never panics.
    pub fn greet(&self) -> String {
        format!("Hello, my name is {} and I'm {}", self.name, self.age)
    }
}

/// Adds two numbers together.
///
/// # Examples
///
/// ```
/// let sum = add(2, 3);
/// assert_eq!(sum, 5);
/// ```
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    // This is a single-line comment.

    /*
        This is a multi-line comment.
        It can span multiple lines.
    */

    let p = Person::new("Bob", 30);
    println!("{:?}", p);
    println!("{}", p.greet());

    assert_eq!(add(2, 3), 5);

    // TODO: improve greeting message
    // FIXME: this is just a demo, nothing is broken 🙂

    println!("Comments and Docs: ALL OK ✅");
}
