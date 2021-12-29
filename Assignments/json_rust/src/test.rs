#[cfg(test)]
mod tests {
    use crate::json_response;
    pub use actix_web;
    pub use reqwest::Response;

    // #[test]
    // pub fn pos() {
    //     let value = "base_happiness".to_string();
    //     assert_eq!(
    //         json_response(value).unwrap(),
    //         70
    //     );
    // }
    // #[test]
    // pub fn neg() {
    //     let value = "base_appiness".to_string();
    //     assert!(
    //         matches!(
    //             json_response(value),
    //             Ok(0)
    //         )
    //     );
    // }

    #[test]
    pub fn string_pos() {
        let value = "name".to_string();
        assert_eq!(json_response(value).unwrap(), "purple".to_string());
    }
    #[test]
    pub fn string_neg() {
        let value = "nnnn".to_string();
        assert_eq!(json_response(value).unwrap(), "Error".to_string());
    }
}
