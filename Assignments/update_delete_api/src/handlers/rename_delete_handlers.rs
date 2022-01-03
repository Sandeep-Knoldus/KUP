use serde_json::json;
pub use std::fs;
pub use std::fs::File;

/// Function 'delete_file' deletes a file from a specific location
///
/// #Arguments
///
/// path: It is of type String for the path of the location to the file to be deleted
///
/// #Return
///
/// Returns serde_json::Result<String>
pub async fn delete_file(path: String) -> serde_json::Result<String> {
    log::info!("In the delete_file function");
    let remove_file = fs::remove_file(path).is_ok();
    let json_print = json!({ "status": remove_file });
    serde_json::to_string_pretty(&json_print)
}

/// Function 'rename_file' renames a file in a specific location
///
/// #Arguments
///
/// from: It is of type String for the current name of the file to be changed
/// to: It is of type String for the new name of the file
///
/// #Return
///
/// Returns serde_json::Result<String>
pub async fn rename_file(from: String, to: String) -> serde_json::Result<String> {
    log::info!("In the rename_file function");
    let rename_file = fs::rename(from, to).is_ok();
    let json_print = json!({ "status": rename_file });
    serde_json::to_string_pretty(&json_print)
}

/// Function 'create_file' creates a file at a specific location
///
/// #Arguments
///
/// path: It is of type String for the path of the location to the file to be created
///
/// #Return
///
/// Returns serde_json::Result<String>
pub async fn create_file(path: String) -> serde_json::Result<String> {
    log::info!("In the create_file function");
    let create_file = File::create(path).is_ok();
    let json_print = json!({ "status": create_file });
    serde_json::to_string_pretty(&json_print)
}
