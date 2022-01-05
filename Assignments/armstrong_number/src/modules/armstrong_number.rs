pub use crate::operations::{op, operations_main};

/// Function 'armstrong_no' checks if a number is Armstrong or not
///
/// #Arguments
///
/// input: It is of mutable Integer i32 type taking the number to be tested
///
/// #Return
///
/// Returns Boolean bool; true if Armstrong and false if not
pub fn armstrong_no(input: i32) -> bool {
    let no_of_digits = order(input);
    let mut temp = input;
    let mut sum = 0;
    while temp != 0 {
        let rem = temp % 10;
        sum += power(rem, no_of_digits);
        temp /= 10;
    }
    sum == input
}

/// Function 'order' counts the number of digits of the input number
///
/// #Arguments
///
/// input: It is of mutable Integer i32 type taking to number to count
///
/// #Return
///
/// Returns i32, the count of the digits
pub fn order(mut input: i32) -> i32 {
    let mut count = 0;
    while input != 0 {
        count += 1;
        input /= 10;
    }
    count
}

/// Function 'power' calculates the power base ^ power
///
/// #Arguments
///
/// rem: It is of Integer i32 as the base
/// no_of_digits: It is of Integer i32 as the power
///
/// #Return
///
/// Returns i32, the calculated power
pub fn power(rem: i32, no_of_digits: i32) -> i32 {
    if no_of_digits == 0 {
        return 1;
    }
    if no_of_digits % 2 == 0 {
        return power(rem, no_of_digits / 2) * power(rem, no_of_digits / 2);
    }
    rem * power(rem, no_of_digits / 2) * power(rem, no_of_digits / 2)
}
