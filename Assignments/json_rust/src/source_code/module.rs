pub use actix_web;
pub use reqwest::Response;
use serde_json::Value::String;

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
pub async fn json_response(val: std::string::String) -> reqwest::Result<std::string::String> {
    let response: Response = reqwest::Client::new()
        .get("https://pokeapi.co/api/v2/pokemon-species/ditto")
        .send()
        .await?;
    let value: serde_json::Value = response.json().await?;

    // let v = value[val].clone().as_i64();
    // let c = match v {
    //     Some(c) => c,
    //     None => 0
    // };
    //
    // log::info!("{:?}", c);

    // log::info!("{:#?}", value["flavor_text_entries"][22]["flavor_text"]);

    let stored_value = value["color"][val].to_owned();
    let raw_value = match stored_value {
        String(raw_value) => raw_value,
        _ => "Error".to_string(),
    };
    log::info!("{:#?}", raw_value);
    Ok(raw_value)
}
