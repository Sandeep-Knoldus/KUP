pub mod operations;
pub mod test;

pub mod modules {
    pub mod armstrong_number;
    pub mod automorphic_number;
    pub mod matrix_multiply;
    pub mod neon_number;
    pub mod palindrome;
}

pub use modules::{
    armstrong_number::armstrong_no, automorphic_number::automorphic_no,
    matrix_multiply::matrix_a_b, neon_number::neon_check, palindrome::palindrome_seq,
};
pub use std::io;

/// Function 'main' is used to call other functions
///
/// #Arguments
///
/// No Arguments
///
/// #Return
///
/// No Return
pub fn main() {
    env_logger::init();
    operations::op();
    operations::operations_main();
}
