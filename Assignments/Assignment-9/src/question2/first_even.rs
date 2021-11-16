/// Function 'even' removes the first occurring even number in a vector
///
/// #Arguments
///
/// vect: Taking vector Vec<i32> from 'test.rs'
///
/// #Return
///
/// Returns integer i32 (the first occurring even number in the vector)
pub fn even(vect: Vec<i32>) -> i32 {
    let mut result: i32 = 0;
    for i in 0..vect.len() {
        if vect[i] % 2 == 0 {
            result = vect[i];
            break;
        } else {
            continue;
        }
    }
    result
}