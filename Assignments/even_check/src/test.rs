#[cfg(test)]
use crate::question1::even_check::even_check;
#[test]
pub fn even_check_fail() {
    assert_ne!(even_check(11), Ok("EVEN".to_string()));
}

#[test]
pub fn even_check_success() {
    assert_eq!(even_check(13), Err("Provide correct number".to_string()));
}

#[test]
pub fn even_check_successful() {
    assert_eq!(even_check(8), Ok("EVEN".to_string()));
}

#[test]
pub fn even_check_failure() {
    assert_ne!(even_check(1), Ok("EVEN".to_string()));
}
