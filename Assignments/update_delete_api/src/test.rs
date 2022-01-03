#[cfg(test)]
pub mod tests {
    pub use crate::handlers::rename_delete_handlers::{create_file, delete_file, rename_file};
    pub use serde_json;
    pub use serde_json::json;

    /// Testing creation of file at the specific location
    /// Positive test case [status: true]
    #[actix_web::test]
    pub async fn create_pos() {
        env_logger::init();
        assert_eq!(
            create_file("src/a.txt".to_string()).await.unwrap(),
            serde_json::to_string_pretty(&json!({"status": true})).unwrap()
        );
        log::info!("File Created Successfully");
    }
    /// Negative test case [status: false]
    #[actix_web::test]
    pub async fn create_neg() {
        assert_eq!(
            create_file("sac/a.txt".to_string()).await.unwrap(),
            serde_json::to_string_pretty(&json!({"status": false})).unwrap()
        );
        log::info!("Path not found");
    }

    /// Testing renaming of file at the specific location
    /// Positive test case [status: true]
    #[actix_web::test]
    pub async fn name_change_pos() {
        assert_eq!(
            rename_file("src/a.txt".to_string(), "src/b.txt".to_string())
                .await
                .unwrap(),
            serde_json::to_string_pretty(&json!({"status": true})).unwrap()
        );
        log::info!("File Renamed Successfully");
    }
    /// Negative test case [status: false]
    #[actix_web::test]
    pub async fn name_change_neg() {
        assert_eq!(
            rename_file("sac/a.txt".to_string(), "sac/b.txt".to_string())
                .await
                .unwrap(),
            serde_json::to_string_pretty(&json!({"status": false})).unwrap()
        );
        log::info!("Path not found");
    }

    /// Testing deletion of file at the specific location
    /// Positive test case [status: true]
    #[actix_web::test]
    pub async fn remove_pos() {
        assert_eq!(
            delete_file("src/b.txt".to_string()).await.unwrap(),
            serde_json::to_string_pretty(&json!({"status": true})).unwrap()
        );
        log::info!("File Deleted Successfully");
    }
    /// Negative test case [status: false]
    #[actix_web::test]
    pub async fn remove_neg() {
        assert_eq!(
            delete_file("src/a.txt".to_string()).await.unwrap(),
            serde_json::to_string_pretty(&json!({"status": false})).unwrap()
        );
        log::info!("Path not found");
    }
}
