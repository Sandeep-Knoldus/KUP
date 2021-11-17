/// Function 'reverse' reverse a vector
///
/// #Arguments
///
/// vect: vect is a vector Vec<i32>
///
/// #Return
///
/// Returns Vec<i32> (the new vector with the reversed values)
pub fn reverse(vect: Vec<i32>) -> Vec<i32> {
    let mut vect_2 = Vec::new();
    let length = vect.len() - 1;
    for loop1 in 0..=length {
        vect_2.push(vect[length - loop1]);
    }
    vect_2
}
