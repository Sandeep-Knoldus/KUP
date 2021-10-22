#[cfg(test)]
use crate::question1::even_check::even_check;
#[test]
pub fn check_no_test1() {
    assert_ne!(even_check(11), Ok("EVEN".to_string()));
}

#[test]
pub fn check_no_test2() {
    assert_eq!(even_check(13), Err("Provide correct number".to_string()));
}

#[test]
pub fn check_no_test3() {
    assert_eq!(even_check(8), Ok("EVEN".to_string()));
}

#[test]
pub fn check_no_test4() {
    assert_ne!(even_check(1), Ok("EVEN".to_string()));
}
