pub use futures::future::join;
pub use std::{thread, time};

/// Asynchronous function 'async_fashion' prints data in a Asynchronous Fashion
///
/// #Arguments
///
/// Null
///
/// #Return
///
/// Null
pub async fn async_fashion() {
    log::info!("Data in Asynchronous fashion");
    let async_fashion = thread::spawn(|| {
        for i in 1..11 {
            log::debug!("{}", i);
            thread::sleep(time::Duration::from_millis(10));
        }
    });
    for i in 1..11 {
        log::debug!("{}", i);
        thread::sleep(time::Duration::from_millis(10));
    }
    async_fashion.join().unwrap();
}
