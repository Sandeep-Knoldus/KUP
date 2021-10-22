#[cfg(test)]
use crate::question1::odd_even::odd_even_test;
#[test]
pub fn check_no_test1() {
    assert_ne!(odd_even_test(11), Ok("EVEN".to_string()));
}

#[test]
pub fn check_no_test2() {
    assert_eq!(odd_even_test(13), Err("Provide correct number".to_string()));
}

#[test]
pub fn check_no_test3() {
    assert_eq!(odd_even_test(8), Ok("EVEN".to_string()));
}

#[test]
pub fn check_no_test4() {
    assert_ne!(odd_even_test(1), Ok("EVEN".to_string()));
}
