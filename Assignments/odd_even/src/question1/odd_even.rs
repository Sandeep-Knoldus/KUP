/// Function 'odd_even_test' checks a given number in even or odd
///
/// #Arguments
///
/// result: Taking i32 as input and generating Result of the number
///
/// #Return
///
/// Returns Result type and handling error
pub fn odd_even_test(number: i32) -> Result<String, String> {
    let result_match;
    if number % 2 == 0 {
        result_match = Ok(());
    } else {
        result_match = Err(());
    }
    match result_match {
        Ok(_) => Ok("EVEN".to_string()),
        Err(_) => Err("Provide correct number".to_string()),
    }
}
