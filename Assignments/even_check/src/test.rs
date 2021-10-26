#[cfg(test)]
use crate::question1::even_check::check;
#[test]
pub fn check_no_ok() {
    assert_eq!(check(10), Ok("EVEN".to_string()));
}

#[test]
pub fn check_no_err() {
    assert_eq!(check(13), Err("Provide correct number".to_string()));
}

#[test]
pub fn check_number_ok() {
    assert_eq!(check(8), Ok("EVEN".to_string()));
}

#[test]
pub fn check_number_err() {
    assert_eq!(check(1), Err("Provide correct number".to_string()));
}

#[test]
pub fn number_ok() {
    assert_eq!(check(98), Ok("EVEN".to_string()));
}

#[test]
pub fn number_err() {
    assert_eq!(check(37), Err("Provide correct number".to_string()));
}

#[test]
pub fn check_ok() {
    assert_eq!(check(-88), Ok("EVEN".to_string()));
}

#[test]
pub fn check_err() {
    assert_eq!(check(-13), Err("Provide correct number".to_string()));
}
