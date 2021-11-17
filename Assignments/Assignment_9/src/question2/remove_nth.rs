/// Function 'remove_value' removes a given value from a vector
///
/// #Arguments
///
/// vect: vect is a vector Vec<i32>
/// drop_value: drop_value is an integer of i32 type
///
/// #Return
///
/// Returns Vec<i32> (the new vector without the drop_value)
pub fn remove_value(vect: Vec<i32>, drop_value: i32) -> Vec<i32> {
    let mut vec_2 = Vec::new();
    for values in vect {
        if drop_value != values {
            vec_2.push(values);
        }
        continue;
    }
    vec_2
}
