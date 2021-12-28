#[cfg(test)]
mod tests {
    pub use actix_web;
    pub use reqwest::Response;
    use std::process;

    #[actix_web::test]
    pub async fn number_test_positive() {
        let value = 70;
        let response: serde_json::Value = reqwest::Client::new()
            .get("https://pokeapi.co/api/v2/pokemon-species/ditto")
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        let value1 = response.get("base_happiness");
        let value_1 = match value1 {
            Some(value_1) => value_1,
            None => process::exit(0),
        };
        assert_eq!(value.to_string(), value_1.to_string())
    }

    #[actix_web::test]
    pub async fn string_test_positive() {
        let value = "\"purple\"".to_string();
        let response: serde_json::Value = reqwest::Client::new()
            .get("https://pokeapi.co/api/v2/pokemon-species/ditto")
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        let value1 = response.get("color").and_then(|name| name.get("name"));
        let value_1 = match value1 {
            Some(value_1) => value_1,
            None => process::exit(0),
        };
        assert_eq!(value.to_string(), value_1.to_string())
    }

    #[actix_web::test]
    pub async fn string_test_negative() {
        let value = None;
        let response: serde_json::Value = reqwest::Client::new()
            .get("https://pokeapi.co/api/v2/pokemon-species/ditto")
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        let value1 = response.get("color").and_then(|name| name.get("na"));

        assert_eq!(value, value1)
    }
}
