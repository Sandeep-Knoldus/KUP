pub use actix_web::{web, App, HttpServer};
pub use crate::info;

/// Function 'posting' starts a HTTP POST Server
///
/// #Arguments
///
/// bind_address: Is of type String which is the binding address
///
/// #Return
///
/// Returns std::io::Result<()>
#[actix_web::main]
pub async fn posting(bind_address: String) -> std::io::Result<()> {
    log::info!("Starting a HTTP Server");

    HttpServer::new(|| {
        App::new()
            .route("/sandy", web::post().to(info::add_user))
    })
    .bind(bind_address)?
    .run()
    .await
}
