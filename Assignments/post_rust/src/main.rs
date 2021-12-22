use actix_web::{web, App, HttpServer};

/// Function 'add_user' prints the information to POST
///
/// #Arguments
///
/// No arguments
///
/// #Return
///
/// Returns String
async fn add_user() -> String {
    log::info!("Calling the 'add_user' function");
    "This is called from the 'add_user' function".to_string()
}

/// Function 'main' starts a HTTP Server
///
/// #Arguments
///
/// No arguments
///
/// #Return
///
/// Returns std::io::Result<()>
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    log::info!("Starting a HTTP Server");
    HttpServer::new(|| {
        App::new()
            .route("/sandy", web::post().to(add_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}