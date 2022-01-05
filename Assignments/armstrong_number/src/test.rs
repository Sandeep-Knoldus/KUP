#[cfg(test)]
pub mod tests {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    pub use crate::modules::{armstrong_number, automorphic_number, neon_number, palindrome, matrix_multiply};
    pub use crate::operations::{op, operations_main};
    use crate::{armstrong_no, automorphic_no, neon_check, palindrome_seq, matrix_a_b};

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
        let mut f = BufReader::new(File::open("src/matrix1").expect("Cannot open file"));

        let mut num_line = String::new();
        f.read_line(&mut num_line).expect("Cannot read line");
        let n: usize = num_line[1..].trim().parse().expect("Not integer");

        let mut arr = vec![vec![0i32; n]; n];
        for (i, line) in f.lines().enumerate() {
            for (j, number) in line.unwrap().split(char::is_whitespace).enumerate() {
                arr[i][j] = number.trim().parse().unwrap();
            }
        }

        let mut f = BufReader::new(File::open("src/matrix2").expect("Cannot open file"));

        let mut num_line = String::new();
        f.read_line(&mut num_line).expect("Cannot read line");
        let n: usize = num_line[1..].trim().parse().expect("Not integer");

        let mut arr2 = vec![vec![0i32; n]; n];
        for (i, line) in f.lines().enumerate() {
            for (j, number) in line.unwrap().split(char::is_whitespace).enumerate() {
                arr2[i][j] = number.trim().parse().unwrap();
            }
        }
        assert_eq!(matrix_a_b(arr, arr2), [[6, 6, 6], [15, 15, 15], [24, 24, 24]])
    }

}
