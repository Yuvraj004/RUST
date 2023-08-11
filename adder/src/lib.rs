// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)] -> tells the compiler to only run the following program on cargo test
// cfg -> configuration
// mod tests {
//     use super::*;

//     #[test]
//     fn exploration() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//         //The example function body uses the assert_eq! macro to assert that result, which contains the result of adding 2 and 2, equals 4. This assertion serves as an example of the format for a typical test.
//     }
//     #[test]
//     fn another() {
//         panic!("Make the test fail");
//     }
// }

// Checking Results with the assert! Macro
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }
// #[cfg(test)]
// mod tests {
//     use super::*;// Because the tests module is an inner module, we need to bring the code under test in the outer module into the scope of the inner module.

//     #[test]
//     fn larger_can_hold_smaller() {
//         let larger = Rectangle {
//             width: 8,
//             height: 7,
//         };
//         let smaller = Rectangle {
//             width: 5,
//             height: 1,
//         };

//         assert!(larger.can_hold(&smaller));
//     }
//     #[test]
//     fn smaller_cannot_hold_larger() {
//         let larger = Rectangle {
//             width: 8,
//             height: 7,
//         };
//         let smaller = Rectangle {
//             width: 5,
//             height: 1,
//         };

//         assert!(!smaller.can_hold(&larger));
//     }
// }
//
//
//Testing Equality with the assert_eq! and assert_ne! Macros
//
//
// These macros compare two arguments for equality or inequality, respectively.
//They’ll also print the two values if the assertion fails, which makes it easier to see why the test failed; 
//conversely, the assert! macro only indicates that it got a false value for the == expression, without printing the values that led to the false value.
pub fn add_two(a: i32) -> i32 {
    a + 2
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_adds_two() {
//         assert_eq!(4, add_two(2));
//     }
// }
pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn greeting_contains_name() {
//         let result = greeting("Carol");
//         assert!(result.contains("Carol"));
//     }
// }
//
//
//
//using Result<T,E> to return an error instead of panicking
// #[cfg(test)]
// mod tests {
//     #[test]
//     //You can’t use the #[should_panic] annotation on tests that use Result<T, E>.
//     fn it_works() -> Result<(), String> {
//         if 2 + 2 == 4 {
//             Ok(())
//         } else {
//             Err(String::from("two plus two does not equal four"))
//         }
//     }
// }

//
//
//testing features
// in default ,all the test run parallel => tests should not depend on previous one
//for aoviding parallel tests threads
//
// cargo test -- --test-threads=1
//We set the number of test threads to 1, telling the program not to use any parallelism. 
//
//$ cargo test -- --show-output  :If we want to see printed values for passing tests as well, we can tell Rust to also show the output of successful tests with --show-output.
//
// to run a single test whose fn name  = hello
//cargo test hello
//
//If two functions start with a common word ,test will run on both of te fn
//cargo test add  => al the functions starting with add will be tested
//
//To ignore running a particular test
//#[test]
// fn it_works() {
//     assert_eq!(2 + 2, 4);
// }

// #[test]
// #[ignore]
// fn expensive_test() {
//     // code that takes an hour to run
// }
//
//The expensive_test function is listed as ignored. If we want to run only the ignored tests, we can use :::   cargo test -- --ignored:
//
//If you want to run all tests whether they’re ignored or not, you can run :   cargo test -- --include-ignored.

//
//Part-2
//private functions can also be tested in RUST
//using  : use super::* ->syntax
//Continued in tests folder
