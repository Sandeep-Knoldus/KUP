pub use log::*;
/// Enum 'List' consists of 2 arms Const(i32, Box<List>) and Space
pub enum List {
    Const(i32, Box<List>),
    Space,
}
use List::{Const, Space};

/// Function 'nth_number' finds the element at a given index
///
/// #Arguments
///
/// comparing_element - It is of type integer i32
/// index- It is of type integer i32
/// given_input - It is of type List enum
///
/// #Return
///
/// Return i32 (the element at the given index)
pub fn nth_number(comparing_element: i32, index: i32, given_input: List) -> i32 {
    info!("Nth element finder");
    info!("Inside 'nth_number' function");
    match given_input {
        Const(element, modified_input) => {
            if comparing_element == index {
                element
            } else {
                nth_number(comparing_element + 1, index, *modified_input)
            }
        }
        Space => 0,
    }
}
