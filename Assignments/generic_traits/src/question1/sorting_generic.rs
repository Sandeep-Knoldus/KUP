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
    for i in 0..list.len() {
        let mut small = i;
        for j in (i + 1)..list.len() {
            if list[j] < list[small] {
                small = j;
            }
        }
        list.swap(small, i);
    }
    list
}
