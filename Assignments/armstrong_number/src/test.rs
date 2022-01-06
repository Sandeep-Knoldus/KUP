#[cfg(test)]
pub mod tests {
    pub use crate::modules::{
        armstrong_number, automorphic_number, matrix_multiply, neon_number, palindrome,
    };
    pub use crate::operations::{op, operations_main};
    use crate::{armstrong_no, automorphic_no, matrix_a_b, neon_check, palindrome_seq};

    #[test]
    pub fn armstrong_success() {
        assert_eq!(armstrong_no(153), true);
    }
    #[test]
    pub fn armstrong_failure() {
        assert_eq!(armstrong_no(15), false);
    }
    #[test]
    pub fn automorphic_success() {
        assert_eq!(automorphic_no(5), true);
    }
    #[test]
    pub fn automorphic_failure() {
        assert_eq!(automorphic_no(9), false);
    }
    #[test]
    pub fn neon_success() {
        assert_eq!(neon_check(9), true);
    }
    #[test]
    pub fn neon_failure() {
        assert_eq!(neon_check(8), false);
    }
    #[test]
    pub fn palindrome_success() {
        assert_eq!(palindrome_seq(1234321), true)
    }
    #[test]
    pub fn palindrome_failure() {
        assert_eq!(palindrome_seq(123412), false)
    }
    #[test]
    fn multiplication_first_success() {
        let first = [[1, 2, 3].to_vec(), [4, 5, 6].to_vec(), [7, 8, 9].to_vec()].to_vec();
        let second = [[1, 0, 0].to_vec(), [0, 1, 0].to_vec(), [0, 0, 1].to_vec()].to_vec();
        let right = [[1, 2, 3].to_vec(), [4, 5, 6].to_vec(), [7, 8, 9].to_vec()].to_vec();
        assert_eq!(matrix_a_b(&first, &second), Ok(right));
    }
    #[test]
    fn multiplication_second_success() {
        let first = [[1, 2].to_vec(), [3, 4].to_vec()].to_vec();
        let second = [[1, 0].to_vec(), [0, 1].to_vec()].to_vec();
        let right = [[1, 2].to_vec(), [3, 4].to_vec()].to_vec();
        assert_eq!(matrix_a_b(&first, &second), Ok(right));
    }
    #[test]
    fn multiplication_first_failure() {
        let first = [[1, 2].to_vec(), [3, 4].to_vec()].to_vec();
        let second = [[1, 2, 3].to_vec(), [4, 5, 6].to_vec(), [7, 8, 9].to_vec()].to_vec();
        assert_eq!(
            matrix_a_b(&first, &second),
            Err("Not possible".to_string())
        );
    }
    #[test]
    fn multiplication_second_failure() {
        let first = [[1, 2, 3].to_vec(), [4, 5, 6].to_vec(), [7, 8, 9].to_vec()].to_vec();
        let second = [[1, 2].to_vec(), [3, 4].to_vec()].to_vec();
        assert_eq!(
            matrix_a_b(&first, &second),
            Err("Not possible".to_string())
        );
    }
}
