pub use log::*;
/// Enum 'List' consists of 2 arms Constructor(i32, Box<List>) and Nothing
pub enum List {
    Constructor(i32, Box<List>),
    Nothing,
}
use List::{Constructor, Nothing};

/// Function 'first_odd_number' calls for the 'second_odd_number' function
///
/// #Arguments
///
/// given_input: It is of type List enum
///
/// #Return
///
/// Returns i32
pub fn first_odd_number(given_input: List) -> i32 {
    info!("Third Odd element");
    info!("Inside 'first_odd_number' function");
    match given_input {
        Constructor(element, modified_input) => {
            if element % 2 != 0 {
                second_odd_number(*modified_input)
            } else {
                first_odd_number(*modified_input)
            }
        }
        Nothing => 0,
    }
}

/// Function 'second_odd_number' calls for the 'third_odd_number' function
///
/// #Arguments
///
/// modified_input: It is of type List enum
///
/// #Return
///
/// Returns i32
pub fn second_odd_number(modified_input: List) -> i32 {
    info!("Inside 'second_odd_number' function");
    match modified_input {
        Constructor(element, next_modified_input) => {
            if element % 2 != 0 {
                third_odd_number(*next_modified_input)
            } else {
                second_odd_number(*next_modified_input)
            }
        }
        Nothing => 0,
    }
}

/// Function 'third_odd_number' finds the third odd element in the list
///
/// #Arguments
///
/// next_modified_input: It is of type List enum
///
/// #Return
///
/// Returns i32 (the third odd element)
pub fn third_odd_number(next_modified_input: List) -> i32 {
    info!("Inside 'third_odd_number' function");
    match next_modified_input {
        Constructor(element, next_input) => {
            if element % 2 != 0 {
                element
            } else {
                third_odd_number(*next_input)
            }
        }
        Nothing => 0,
    }
}
