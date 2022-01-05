pub use crate::operations::{op, operations_main};
pub use std::io;

/// Function 'palindrome_seq' checks if an Integer/String is Palindrome or not
///
/// #Arguments
///
/// input: It is of mutable Integer i32 type taking the number to be tested
///
/// #Return
///
/// Returns Boolean bool; true if Palindrome and false if not
pub fn palindrome_seq(mut input: i32) -> bool {
    let copy = input;
    let mut rem: i32;
    let mut rev = 0;
    while input != 0 {
        rem = input % 10;
        rev = rev * 10 + rem;
        input /= 10;
    }
    rev == copy
}
