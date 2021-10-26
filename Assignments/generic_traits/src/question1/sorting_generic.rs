/// Function 'sorting_number' sorts a given list
///
/// #Arguments
///
/// list: Taking generic type [T] mutable vector from 'test.rs'
///
/// #Return
///
/// Returns the generic type [T] vector (the sorted list)
pub fn sorting_numbers<T: PartialOrd>(list: &mut [T]) -> &[T] {
    for loop1 in 0..list.len() {
        let mut small = loop1;
        for loop2 in (loop1 + 1)..list.len() {
            if list[loop2] < list[small] {
                small = loop2;
            }
        }
        list.swap(small, loop1);
    }
    list
}
