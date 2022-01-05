pub use crate::operations::{op, operations_main};
pub use std::io;

/// Function 'neon_check' checks if a number is Neon or not
///
/// #Arguments
///
/// input: It is of Integer i32 type taking the number to be tested
///
/// #Return
///
/// Returns Boolean bool; true if Neon and false if not
pub fn neon_check(input: i32) -> bool {
    let mut sum = 0;
    let mut sqr = input * input;
    while sqr != 0 {
        let digit = sqr % 10;
        sum += digit;
        sqr /= 10;
    }
    input == sum
}
