/// Function 'remove_value' removes a given value from a vector
///
/// #Arguments
///
/// vect: Taking vector Vec<i32> from 'test.rs'
/// drop_value: Taking i32, as the value to be dropped, from 'test.rs'
///
/// #Return
///
/// Returns vector Vec<i32> (the new vector without the drop_value)
pub fn remove_value(vect: Vec<i32>, drop_value: i32) -> Vec<i32> {
    let mut vec_2 = Vec::new();
    for i in 0..vect.len() {
        if drop_value != vect[i] {
            vec_2.push(vect[i]);
        }
        continue
    }
    vec_2
}