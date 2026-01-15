## crate.html answers

3) Q: What's the difference between package 1 and package 2?
A: Package 1 contains a binary crate (src/main.rs). Package 2 contains a library crate (src/lib.rs).

4) Q: What's the name of the library crate in package `hello-package1`?
A: `hello_package1` (crate name defaults to the package name, with '-' replaced by '_').

5) Add a library crate for `hello-package` and describe files tree:
.
├── Cargo.lock
├── Cargo.toml
├── src
│   ├── lib.rs
│   └── main.rs

6) Package with 3 binary crates (`hello-package`, `main1`, `main2`) + 1 library crate:
.
├── Cargo.toml
├── Cargo.lock
├── src
│   ├── lib.rs
│   ├── main.rs
│   └── bin
│       ├── main1.rs
│       └── main2.rs
├── tests
│   └── some_integration_tests.rs
├── benches
│   └── simple_bench.rs
└── examples
    └── simple_example.rs
