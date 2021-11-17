/// Function 'remove_int' removes the continuously repeating integers from a vector
///
/// #Arguments
///
/// vect: vect is a vector Vec<i32>
///
/// #Return
///
/// Returns Vec<i32> (the new vector without the continuously repeating values)
pub fn remove_int(vect: Vec<i32>) -> Vec<i32> {
    let mut vec_2 = Vec::new();
    let length = vect.len();
    vec_2.push(vect[0]);
    for loop1 in 1..length {
        if vect[loop1] != vect[loop1 - 1] {
            vec_2.push(vect[loop1]);
        }
    }
    vec_2
}
