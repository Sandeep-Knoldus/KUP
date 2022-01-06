pub use crate::operations::{op, operations_main};

/// Function 'matrix_a_b' calculates the multiplication of 2 matrices
///
/// #Arguments
///
/// arr1: It is of type [[usize; 2]; 2] taking the first matrix
/// arr2: It is of type [[usize; 2]; 2] taking the second matrix
///
/// #Return
///
/// Returns [[i32; 2]; 2] the multiplied matrix
pub fn matrix_a_b(arr1: [[usize; 2]; 2], arr2: [[usize; 2]; 2]) -> [[usize; 2]; 2] {
    let mut new_arr: [[usize; 2]; 2] = [[0_usize; 2]; 2];
    for loop1 in 0..2 {
        for loop2 in 0..2 {
            for (loop3, _i) in arr2.iter().enumerate().take(2) {
                new_arr[loop1][loop2] += arr1[loop1][loop3] * arr2[loop3][loop2];
            }
        }
    }
    new_arr
}
