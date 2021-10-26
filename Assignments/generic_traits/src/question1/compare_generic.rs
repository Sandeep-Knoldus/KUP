/// Function 'compare_numbers' compares two given numbers to find the smaller number
///
/// #Arguments
///
/// first_number: Taking generic type T as input from 'test.rs'
/// second_number: Taking generic type T as input from 'test.rs'
///
/// #Return
///
/// Returns generic type T (the smaller number)
pub fn compare_numbers<T: PartialOrd>(first_number: T, second_number: T) -> T {
    if first_number < second_number {
        return first_number;
    }
    second_number
}
