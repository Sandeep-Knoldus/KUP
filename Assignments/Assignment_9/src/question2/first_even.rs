/// Function 'even' removes the first occurring even number in a vector
///
/// #Arguments
///
/// vect: vect is a vector Vec<i32>
///
/// #Return
///
/// Returns i32 (the first occurring even number in the vector)
pub fn even(vect: Vec<i32>) -> i32 {
    let mut result: i32 = 0;
    for values in vect {
        if values % 2 == 0 {
            result = values;
            break;
        } else {
            continue;
        }
    }
    result
}
