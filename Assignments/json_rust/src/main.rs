pub mod test;
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
    let value = "sandeep".to_string();
    json_response(value).ok();
}
