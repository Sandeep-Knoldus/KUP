/// Function 'check_palindrome' checks whether the given vector is palindrome or not
///
/// #Arguments
///
/// vect: vect is a vector Vec<i32>
///
/// #Return
///
/// Returns bool (true -> if palindrome; false -> if not_palindrome)
pub fn check_palindrome(vect: Vec<i32>) -> bool {
    let mut vect_2 = Vec::new();
    let length = vect.len() - 1;
    for loop1 in 0..=length {
        if vect[loop1] == vect[length - loop1] {
            vect_2.push(vect[loop1]);
        }
    }
    vect == vect_2
}
