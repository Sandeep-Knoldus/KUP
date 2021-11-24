pub use futures::future::join;
pub use std::{thread, time};

/// Asynchronous function 'async_fashion' prints data in a Asynchronous Fashion
///
/// #Arguments
///
/// No Arguments
///
/// #Return
///
/// No Return
pub async fn async_fashion() {
    log::info!("Data in Asynchronous fashion");
    let async_fashion = thread::spawn(|| {
        for print_number in 1..11 {
            log::debug!("{}", print_number);
            thread::sleep(time::Duration::from_millis(10));
        }
    });
    for print_number in 1..11 {
        log::debug!("{}", print_number);
        thread::sleep(time::Duration::from_millis(10));
    }
    async_fashion.join().unwrap();
}
