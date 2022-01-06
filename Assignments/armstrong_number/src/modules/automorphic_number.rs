pub use crate::operations::{op, operations_main};

/// Function 'automorphic_no' checks if a number is Automorphic or not
///
/// #Arguments
///
/// input: It is of Integer i32 type taking the number to be tested
///
/// #Return
///
/// Returns Boolean bool; true if Automorphic and false if not
pub fn automorphic_no(input: i32) -> bool {
    let mut count = 0;
    let sqr = input * input;
    let mut temp = input;

    // Counting the digits
    while temp > 0 {
        count += 1;
        temp /= 10;
    }

    let last_sqr_digits = sqr % power(10, count);
    input == last_sqr_digits
}

/// Function 'power' calculates the power base ^ power
///
/// #Arguments
///
/// base: It is of Integer i32 as the base
/// count: It is of Integer i32 as the power
///
/// #Return
///
/// Returns i32, the calculated power
pub fn power(base: i32, count: i32) -> i32 {
    if count == 0 {
        return 1;
    }
    if count % 2 == 0 {
        return power(base, count / 2) * power(base, count / 2);
    }
    base * power(base, count / 2) * power(base, count / 2)
}
