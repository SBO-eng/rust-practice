use std::fmt::Result as FmtResult;
use std::io::Result as IoResult;

// 2) Вставка "в одну строку" (2 способа возможны; я даю самый нормальный)
use std::collections::{BTreeMap, HashMap, HashSet};

fn main() {
    // 1) просто чтобы не ругалось на неиспользование алиасов
    let _a: Option<FmtResult> = None;
    let _b: Option<IoResult<()>> = None;

    // 2)
    let _c1: HashMap<&str, i32> = HashMap::new();
    let mut c2 = BTreeMap::new();
    c2.insert(1, "a");
    let _c3: HashSet<i32> = HashSet::new();

    // 3) Re-exporting names with pub use:
    // Это проверяется в `hello-package/src/lib.rs`, где есть:
    // pub use crate::front_of_house::hosting;
    //
    // Поэтому вот так будет работать (если бы проект собирался Cargo):
    // assert_eq!(hello_package::hosting::seat_at_table(), "sit down please");
    // assert_eq!(hello_package::eat_at_restaurant(), "yummy yummy!");
}
