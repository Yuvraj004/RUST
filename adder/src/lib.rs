// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
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
    a + 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
}