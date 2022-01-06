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
    pub fn matrix_success() {
        let mat_1 = [[1, 2], [3, 4]];
        let mat_2 = [[5, 6], [7, 8]];
        assert_eq!(matrix_a_b(mat_1, mat_2), [[19, 22], [43, 50]]);
    }
    #[test]
    pub fn matrix_failure() {
        let mat_1 = [[3, 1], [8, 9]];
        let mat_2 = [[5, 6], [7, 8]];
        assert_ne!(matrix_a_b(mat_1, mat_2), [[19, 22], [43, 50]]);
    }
}
