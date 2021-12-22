#[cfg(test)]
pub mod tests{
    pub use crate::post_rust::posting;
    #[test]
    pub fn binding_success() {
        env_logger::init();
        assert!(
            matches!(
                posting("127.0.0.1:8080".to_string()),
                Ok(())
            )
        );
    }
}
