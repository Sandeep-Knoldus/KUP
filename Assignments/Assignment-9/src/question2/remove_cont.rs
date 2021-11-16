/// Function 'remove_int' removes the continuously repeating integers from a vector
///
/// #Arguments
///
/// vect: Taking vector Vec<i32> from 'test.rs'
///
/// #Return
///
/// Returns vector Vec<i32> (the new vector without the continuously repeating values)
pub fn remove_int(vect: Vec<i32>) -> Vec<i32> {
    let mut vec_2 = vec![];
    vec_2.push(vect[0]);
    for i in 1..vect.len() {
        if vect[i] != vect[i-1] {
            vec_2.push(vect[i]);
        }
    }
    vec_2
}
