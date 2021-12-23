use actix_web::{web, App, HttpServer, Responder};
use serde::Serialize;

/// Structure 'Country' with 2 fields
///
/// country_code: Is of type String
/// country_name: Is of type String
#[derive(Serialize)]
struct Country {
    country_code: String,
    country_name: String
}

/// Function 'get_state' stores value in the Structure 'Country' format
///
/// #Arguments
///
/// No Arguments
///
/// #Return
///
/// Returns handler
async fn get_state() -> impl Responder {
    let mut vec:Vec<Country> = Vec::new();

    vec.push(Country{country_code: "IND".to_string(), country_name: "India".to_string()});
    vec.push(Country{country_code: "US".to_string(), country_name: "United States".to_string()});
    vec.push(Country{country_code: "UK".to_string(), country_name: "United Kingdom".to_string()});

    web::Json(vec)
}

/// Function 'main' creates a HTTP Server
/// 
/// #Arguments
///
/// No Arguments
///
/// #Return
///
/// Returns std::io::Result<()>
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    log::info!("Creating new HTTP Server");
    HttpServer::new(|| {
        App::new()
            .route("/country_codes", web::post().to(get_state))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
