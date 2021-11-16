/// Function 'duplicate' adds duplicate value of the iterated value
///
/// #Arguments
///
/// vect: Taking vector Vec<i32> from 'test.rs'
///
/// #Return
///
/// Returns vector Vec<i32> (the new vector with the duplicate values)
pub fn duplicate(vect: Vec<i32>) -> Vec<i32> {
    let mut vec_2 = Vec::new();
    for values in &vect {
        for _loop1 in 0..2 {
            vec_2.push(*values);
        }
    }
    vec_2
}
