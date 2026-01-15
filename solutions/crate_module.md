[package]
name = "hello-package"
version = "0.1.0"
edition = "2021"
// hello-package/src/lib.rs

pub mod front_of_house;
pub mod back_of_house;

// чтобы работало и для module.html (через front_of_house::hosting...)
// и для use-pub.html (через hello_package::hosting::...)
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() -> String {
    // абсолютный путь
    crate::front_of_house::hosting::add_to_waitlist();

    // относительный путь
    front_of_house::hosting::add_to_waitlist();

    back_of_house::cook_order();

    String::from("yummy yummy!")
}
// hello-package/src/back_of_house.rs

pub fn fix_incorrect_order() {
    cook_order();

    // 3 варианта (достаточно одного, но пусть будет явно):
    // super (если бы мы были внутри модуля), тут мы в модуле crate::back_of_house,
    // поэтому используем абсолютный путь:
    crate::front_of_house::serving::serve_order();
}

pub fn cook_order() {}
// hello-package/src/front_of_house/mod.rs

pub mod hosting;
pub mod serving;
// hello-package/src/front_of_house/serving.rs

pub fn take_order() {}
pub fn serve_order() {}
pub fn take_payment() {}

// private
fn complain() {}
// hello-package/src/main.rs

fn main() {
    assert_eq!(
        hello_package::front_of_house::hosting::seat_at_table(),
        "sit down please"
    );
    assert_eq!(hello_package::eat_at_restaurant(), "yummy yummy!");
}
