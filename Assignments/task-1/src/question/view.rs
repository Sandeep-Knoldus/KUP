pub extern crate mysql;

/// Structure 'Emp' having 2 fields
///
/// #Fields
///
/// emp_id: Is of i32 type which takes employee id
/// emp_name: Is of String type which takes employee name
pub struct Emp {
    pub emp_id: i32,
    pub emp_name: String,
}

/// Function 'view_table_in_database' prints the contents of the table in database
///
/// #Arguments
///
/// query_test: Is of type String which takes the SELECT query
/// link: Is of type String which takes the link to the MySql database
///
/// #Return
///
/// Returns Option<()> which returns Some() if database and table exists and None if it doesn't
pub fn view_table_in_database(query_test: String, link: String) -> Option<()> {
    let pool = mysql::Pool::new(link).ok()?;
    let emps: Vec<Emp> = pool
        .prep_exec(query_test, ())
        .map(|result| {
            result
                .map(|x| match x {
                    Ok(view) => view,
                    Err(error) => panic!("{:?}", error)
                })
                .map(|row| {
                    let (emp_id, emp_name) = mysql::from_row(row);
                    Emp { emp_id, emp_name }
                })
                .collect()
        })
        .ok()?;

    for emp in emps {
        log::info!("{:?}: {:?}", emp.emp_id, emp.emp_name);
    }
    Some(())
}
