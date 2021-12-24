pub use actix_web;
pub use reqwest::Response;

/// Async function 'json_response' is used to get the JSON path and print desired keys
///
/// #Arguments
///
/// No arguments
///
/// #Return
///
/// Returns reqwest::Result<()>
#[actix_web::main]
pub async fn json_response() -> reqwest::Result<()> {
    env_logger::init();
    let response: Response = reqwest::Client::new()
        .get("https://pokeapi.co/api/v2/pokemon-species/ditto")
        .send()
        .await?;
    let value: serde_json::Value = response.json().await?;

    log::info!("{:#?}", value.get("base_happiness"));
    log::info!("{:#?}", value["flavor_text_entries"][22].get("flavor_text"));
    log::info!(
        "{:#?}",
        value.get("color").and_then(|name| name.get("name"))
    );
    Ok(())
}