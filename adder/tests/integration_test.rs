/*
Part -2 Continued
*/
use adder;
// Each file in the tests directory is a separate crate, so we need to bring our library into each test crate’s scope. For that reason we add use adder at the top of the code, which we didn’t need in the unit tests.
mod common;
#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}

//
//To run only integration test
//cargo test --test integration_test
// tests/common.rs will run the common.rs
// To avoid the above problem : we’ve created tests/common/mod.rs
//
//
//we can’t create integration tests in the tests directory and bring functions defined in the src/main.rs file into scope with a use statement.
//Only library crates expose functions that other crates can use; binary crates are meant to be run on their own.