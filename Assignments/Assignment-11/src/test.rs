#[cfg(test)]
pub use crate::questions::asynchronous_fashion::async_fashion;
pub use crate::questions::multiplication_table::async_multiply;
pub use futures::executor::block_on;
#[test]
pub fn multiplication_table_success() {
    env_logger::init();
    assert_eq!(block_on(async_multiply()), ());
}

#[test]
pub fn asynchronous_fashion_success() {
    assert_eq!(block_on(async_fashion()), ());
}
