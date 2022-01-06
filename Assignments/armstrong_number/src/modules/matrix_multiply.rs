pub use crate::operations::{op, operations_main};

/// Function 'matrix_a_b' calculates the multiplication of 2 matrices
///
/// #Arguments
///
/// first_matrix: It is of type &[Vec<i32>] taking the first matrix
/// second_matrix: It is of type &[Vec<i32>] taking the second matrix
///
/// #Return
///
/// Returns Result<Vec<Vec<i32>>, String>
pub fn matrix_a_b(
    first_matrix: &[Vec<i32>],
    second_matrix: &[Vec<i32>],
) -> Result<Vec<Vec<i32>>, String> {
    if first_matrix[0].len() != second_matrix.len() {
        return Err("Not possible".to_string());
    }
    let mut result: Vec<Vec<i32>> = Vec::new();
    let l_1 = first_matrix.len();
    let l_2 = second_matrix.len();
    for (index_1, _item) in first_matrix.iter().enumerate().take(l_1)  {
        log::info!("{}",index_1);
        let mut vec: Vec<i32> = Vec::new();
        for index_2 in 0..second_matrix[0].len() {
            let mut sum = 0;
            for (index_3, _index) in second_matrix.iter().enumerate().take(l_2){
                sum += first_matrix[index_1][index_3] * second_matrix[index_3][index_2];
            }
            vec.push(sum);
        }
        result.push(vec);
    }
    Ok(result)
}
