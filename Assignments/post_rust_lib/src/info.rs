/// Function 'add_user' sends the information to print
///
/// #Arguments
///
/// No arguments
///
/// #Return
///
/// Returns String
pub async fn add_user() -> String {
    log::info!("Calling the 'add_user' function");
    format!("This is called from the add_user")
}
