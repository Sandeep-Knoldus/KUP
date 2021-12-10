pub extern crate mysql;

/// Structure 'Emp' having 2 fields
///
/// #Fields
///
/// emp_id: Is of i32 type
/// emp_name: Is of String type
pub struct Emp {
    emp_id: i32,
    emp_name: String,
}

/// Function 'view_db_table' prints the contents of the table in database
///
/// #Arguments
///
/// query_test: Is if type String
/// link: Is of type String
///
/// #Return
///
/// Returns Option<()>
pub fn view_db_table(query_test: String, link: String) -> Option<()> {
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
