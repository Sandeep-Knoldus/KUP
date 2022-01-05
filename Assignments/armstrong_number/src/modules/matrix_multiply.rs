pub use crate::operations::{op, operations_main};

/// Function 'matrix_a_b' calculates the multiplication of 2 matrices
///
/// #Arguments
///
/// mat_1: It is of type Vec<Vec<i32>> taking the first matrix
/// mat_2: It is of type Vec<Vec<i32>> taking the second matrix
///
/// #Return
///
/// Returns [[i32; 3]; 3]
pub fn matrix_a_b(mat_1: Vec<Vec<i32>>, mat_2: Vec<Vec<i32>>) -> [[i32; 3]; 3] {
    let mut new_a: [[i32; 3]; 3] = [[0, 0, 0], [0, 0, 0], [0, 0, 0]];
    for loop1 in 0..3{
        for loop2 in 0..3{
            for loop3 in 0..3{
                new_a[loop1][loop2] += mat_1[loop1][loop3] * mat_2[loop3][loop2];
            }
        }
    }
    return new_a;
}
