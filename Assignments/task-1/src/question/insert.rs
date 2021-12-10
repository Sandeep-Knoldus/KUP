pub extern crate mysql;

/// Function 'insert_into_db' inserts values to the table in the database
///
/// #Arguments
///
/// query_test: Is of type String taking the insert query
/// link: Is of type String
///
/// #Return
///
/// Returns Option<()>
pub fn insert_into_db(query_test: String, link: String) -> Option<()> {
    let pool = mysql::Pool::new(link).ok()?;
    pool.prep_exec(query_test, ()).ok()?;
    Some(())
}
