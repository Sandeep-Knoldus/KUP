pub use crate::module::json_response;
pub mod module;

/// Function 'main' calls for the 'json_response' function
///
/// #Arguments
///
/// No Arguments
///
/// #Return
///
/// No Return
fn main() {
    json_response().ok();
}