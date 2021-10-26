/// Function 'even_check' checks a given number is even or not
///
/// #Arguments
///
/// number: Taking i32 as input from 'check' function
///
/// #Return
///
/// Returns Result<String, String> to 'check' function
pub fn even_check(number: i32) -> Result<String, String> {
    let result_match;
    if number % 2 == 0 {
        result_match = Ok("EVEN".to_string());
    } else {
        result_match = Err("Provide correct number".to_string());
    }
    result_match
}

/// Function 'check' takes number as input from the test cases and matches it with 'even_check' function
///
/// #Arguments
///
/// number: Taking i32 as input from test cases
///
/// #Return
///
/// Returns Result<String, String>
pub fn check(number: i32) -> Result<String, String> {
    let result_match = even_check(number);
    match result_match {
        Ok(_) => Ok("EVEN".to_string()),
        Err(_) => Err("Provide correct number".to_string()),
    }
}
