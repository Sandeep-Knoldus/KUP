pub use std::collections::HashMap;
/// Function 'sum_conditional' fetches the value of the key
/// containing the sub-string given and adds them
///
/// #Arguments
///
/// map: Taking Hashmap with name as key with String type
/// and age as value with i32 type from 'test.rs'
/// str: Taking String from 'test.rs' for the sub-string
///
/// #Return
///
/// Returns integer i32 (the addition of the values)
pub fn sum_conditional(map: HashMap<String, i32>, str: String) -> i32 {
    let mut result = 0;
    let length = str.len();
    for (key, value) in &map {
        if key[0..length] == str {
            result += value;
        }
    }
    result
}
