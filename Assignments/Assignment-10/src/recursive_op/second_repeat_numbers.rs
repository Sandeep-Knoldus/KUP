pub use log::*;
/// Enum 'List' consists of 2 arms Sans(i32, Box<List>) and None
pub enum List {
    Sans(i32, Box<List>),
    None,
}
use List::{None, Sans};

/// Function 'second_repeat_element' calls the 'repeat_element' for finding the second consecutively repeating element
///
/// #Arguments
///
/// comparing_element - It is of type integer i32
/// given_input - It is of type List enum
///
/// #Return
///
/// Return i32
pub fn second_repeat_element(comparing_element: i32, given_input: List) -> i32 {
    info!("Second consecutively repeating element");
    info!("Inside 'second_repeat_element' function");
    match given_input {
        Sans(element, modified_input) => {
            if element == 0 {
                0
            } else if comparing_element == element {
                repeat_element(element, *modified_input)
            } else {
                second_repeat_element(element, *modified_input)
            }
        }
        None => 0,
    }
}

/// Function 'repeat_element' finds the second consecutively repeating element
///
/// #Arguments
///
/// comparing_element_2: It is of type i32
/// modified_input: It is of type List enum
///
/// #Return
///
/// Returns i32 (the second consecutively repeating element)
pub fn repeat_element(comparing_element_2: i32, modified_input: List) -> i32 {
    info!("Inside 'repeat_element' function");
    match modified_input {
        Sans(element, next_modified_input) => {
            if comparing_element_2 == element {
                element
            } else {
                repeat_element(element, *next_modified_input)
            }
        }
        None => 0,
    }
}
