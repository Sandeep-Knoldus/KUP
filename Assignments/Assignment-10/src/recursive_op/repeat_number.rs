pub use log::*;
/// Enum 'List' consists of 2 arms Cons(i32, Box<List>) and Nil
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}
use List::{Cons, Nil};

/// Function 'first_repeat' finds the first consecutively repeating element
///
/// #Arguments
///
/// comparing_element - It is of type integer i32
/// given_input - It is of type List enum
///
/// #Return
///
/// Return i32 (the first consecutively repeating element)
pub fn first_repeat(comparing_element: i32, given_input: List) -> i32 {
    info!("First consecutively repeating element");
    info!("Inside 'first_repeat' function");
    match given_input {
        Cons(element, modified_input) => {
            if element == 0 {
                0
            } else if comparing_element == element {
                element
            } else {
                first_repeat(element, *modified_input)
            }
        }
        Nil => 0,
    }
}
