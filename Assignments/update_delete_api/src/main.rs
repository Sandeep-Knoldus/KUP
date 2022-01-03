pub mod test;
pub mod handlers {
    pub mod rename_delete_handlers;
}
pub use actix_web::{web, App, HttpServer};
pub use handlers::rename_delete_handlers::{create_file, delete_file, rename_file};
pub use serde_json;

/// Function 'main' creates a new HTTP Server
///
/// #Arguments
///
/// No Arguments
///
/// #Return
///
/// Returns std::io::Result<()>
#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    log::info!("Starting a HTTP Server");

    HttpServer::new(|| {
        App::new()
            .route("/delete_file", web::delete().to(delete_file))
            .route("/rename_file", web::put().to(rename_file))
            .route("/create_file", web::post().to(create_file))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
