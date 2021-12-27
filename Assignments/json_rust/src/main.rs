pub mod source_code {
    pub mod module;
}
use crate::source_code::module::json_response;


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