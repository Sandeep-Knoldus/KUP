pub use std::collections::HashMap;
/// Function 'sum_conditional' fetches the value of the key, containing the sub-string given and adds them
///
/// #Arguments
///
/// map: Is a Hashmap with name as key and age as value
/// str: Is a String for the sub-string
///
/// #Return
///
/// Returns i32 (the addition of the values)
pub fn sum_conditional(map: HashMap<String, i32>, str: String) -> i32 {
    let mut result = 0;
    let length = str.len();
    for (key, value) in &map {
        for index in 0..key.len() - length + 1 {
            if key[index..(index + length)] == str {
                result += value;
            }
        }
    }
    result
}
