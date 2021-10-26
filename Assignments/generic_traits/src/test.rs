#[cfg(test)]
/// Compare Generic Numbers
pub use crate::question1::compare_generic::compare_numbers;
#[test]
pub fn compare_success() {
    assert_eq!(compare_numbers(10.7, 10.4),10.4);
}
#[test]
pub fn compare_successfully() {
    assert_eq!(compare_numbers(1, 100), 1);
}
#[test]
pub fn min_success() {
    assert_eq!(compare_numbers(33, -1), -1);
}
#[test]
pub fn min_successfully() {
    assert_eq!(compare_numbers(-9, -8), -9);
}

/// Sorting Generic Numbers
pub use crate::question1::sorting_generic::sorting_numbers;
#[test]
pub fn sorting_success() {
    assert_eq!(
        sorting_numbers(&mut vec![5, 1, 7, 8, 2, 9]),
        vec![1, 2, 5, 7, 8, 9]
    );
}
#[test]
pub fn sorting_successfully() {
    assert_eq!(
        sorting_numbers(&mut vec![5.1, 1.3, 7.4, 8.6, 2.7, 9.1]),
        vec![1.3, 2.7, 5.1, 7.4, 8.6, 9.1]
    );
}
#[test]
pub fn sorting_pass() {
    assert_eq!(
        sorting_numbers(&mut vec![5, -1, -2, 8, 2, 9]),
        vec![-2, -1, 2, 5, 8, 9]
    );
}

/// GP Generic
pub use crate::question2::gp_generic::GeometricSeries;
pub use crate::question2::gp_generic::Iterator;
#[test]
pub fn gp_success() {
    let mut values = GeometricSeries {
        first_number: 2,
        ratio: 2,
        current_number: 1,
    };
    assert_eq!(
        values.take(11),
        [2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048]
    );
}
#[test]
pub fn gp_successfully() {
    let mut values = GeometricSeries {
        first_number: 1,
        ratio: 2,
        current_number: 1,
    };
    assert_eq!(
        values.take(11),
        [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024]
    );
}
