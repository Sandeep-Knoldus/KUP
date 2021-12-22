pub use mysql::Pool;

/// Function 'insert_into_table_in_database' inserts values in table in the database
///
/// #Arguments
///
/// query_test: Is of type String which takes the INSERT query
/// link: Is of type String which takes the link to the MySql database
///
/// #Return
///
/// Returns Option<String> which returns Some() if database and table exists and None if it doesn't
pub fn insert_into_table_in_database(query_test: String, link: String) -> mysql::Result<String> {
    let pool = mysql::Pool::new(link)?;
    pool.prep_exec(query_test, ())?;
    Ok("Connection Established".to_string())
}
