/// Function 'even_check' checks a given number is even or not
///
/// #Arguments
///
/// number: Taking i32 as input
///
/// #Return
///
/// Returns Result<String, String>
pub fn even_check(number: i32) -> Result<String, String> {
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
