/// Function 'check_palindrome' checks whether the given vector is palindrome or not
///
/// #Arguments
///
/// vect: Taking vector Vec<i32> from 'test.rs'
///
/// #Return
///
/// Returns boolean bool (true -> if palindrome; false -> if not_palindrome)
pub fn check_palindrome(vect: Vec<i32>) -> bool {
    let mut vect_2 = Vec::new();
    let length = vect.len() - 1;
    for i in 0..=length {
        if vect[i] == vect[length - i] {
            vect_2.push(vect[i]);
        }
    }
    return if vect == vect_2 {
        true
    } else {
        false
    }
}