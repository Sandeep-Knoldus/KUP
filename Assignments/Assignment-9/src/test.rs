#[cfg(test)]
pub use crate::question2::palindrome_vec::check_palindrome;
#[test]
pub fn check_palindrome_success() {
    assert_eq!(check_palindrome(vec![1,2,3,2,1]), true);
}
#[test]
pub fn check_palindrome_successfully() {
    assert_eq!(check_palindrome(vec![1,2,3,4,5]), false);
}

pub use crate::question2::reverse_list::reverse;
#[test]
pub fn reverse_success() {
    assert_eq!(reverse(vec![1,2,3,4,5]), vec![5,4,3,2,1]);
}
#[test]
pub fn reverse_successfully() {
    assert_eq!(reverse(vec![1,2,3,4,5,6]), vec![6,5,4,3,2,1]);
}

pub use crate::question2::first_even::even;
#[test]
pub fn even_success() {
    assert_eq!(even(vec![1,2,3,4,5]), 2);
}
#[test]
pub fn even_successfully() {
    assert_eq!(even(vec![1,7,4,2,1]), 4);
}

pub use crate::question2::remove_cont::remove_int;
#[test]
pub fn remove_success() {
    assert_eq!(remove_int(vec![1,1,1,1,2,2,1,4,4,4,5,1]), vec![1,2,1,4,5,1]);
}
#[test]
pub fn remove_successfully() {
    assert_eq!(remove_int(vec![1,1,1,1,2,3,3,1,1,4,5,5,5,5]), vec![1,2,3,1,4,5]);
}

pub use crate::question2::add_duplicate::duplicate;
#[test]
pub fn add_duplicate_success() {
    assert_eq!(duplicate(vec![1,2,3,3,4]), vec![1,1,2,2,3,3,3,3,4,4]);
}
#[test]
pub fn add_duplicate_successfully() {
    assert_eq!(duplicate(vec![3,2,1,5,5]), vec![3,3,2,2,1,1,5,5,5,5]);
}

pub use crate::question2::remove_nth::remove_value;
#[test]
pub fn remove_value_success() {
    assert_eq!(remove_value(vec![1,2,3,3,4], 3), vec![1,2,4]);
}
#[test]
pub fn remove_value_successfully() {
    assert_eq!(remove_value(vec![9,9,1,2,5,7], 9), vec![1,2,5,7]);
}

pub use crate::question1::hashmap_age::sum_conditional;
#[test]
pub fn sum_conditional_success() {
    use std::collections::HashMap;
    let mut names_age = HashMap::new();
    names_age.insert(String::from("anurag"), 24);
    names_age.insert(String::from("daniel"), 23);
    names_age.insert(String::from("anushka"), 30);

    assert_eq!(sum_conditional(names_age, String::from("anu")), 54);
}
#[test]
pub fn sum_conditional_successfully() {
    use std::collections::HashMap;
    let mut names_age = HashMap::new();
    names_age.insert(String::from("sandeep"), 24);
    names_age.insert(String::from("sanju"), 23);
    names_age.insert(String::from("priya"), 30);

    assert_eq!(sum_conditional(names_age, String::from("san")), 47);
}