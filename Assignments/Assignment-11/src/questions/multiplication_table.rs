pub use futures::future::join;
pub use std::{thread, time};

/// Asynchronous function 'async_multiply' prints a table of 2 and 3.
///
/// #Arguments
///
/// No Arguments
///
/// #Return
///
/// No Return
pub async fn async_multiply() {
    log::info!("Multiplication table of 2 and 3");
    let multiply_by_2 = async {
        for i in 1..=10 {
            log::debug!("2 x {} = {}", i, 2 * i);
            thread::sleep(time::Duration::from_millis(10));
        }
    };
    let multiply_by_3 = async {
        for i in 1..=10 {
            log::debug!("3 x {} = {}", i, 3 * i);
            thread::sleep(time::Duration::from_millis(10));
        }
    };
    join(multiply_by_2, multiply_by_3).await;
}
