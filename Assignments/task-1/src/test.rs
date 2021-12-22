#[cfg(test)]
pub mod tests {
    pub use crate::question::insert::insert_into_table_in_database;
    pub use crate::question::view::view_table_in_database;
    #[test]
    pub fn insert_success() {
        assert!(matches!(
            insert_into_table_in_database(
                "INSERT INTO emp (emp_name) VALUES ('XYZ')".to_string(),
                "mysql://root:12345678@localhost:3306/db_test".to_string()
            )
            .as_deref(),
            Ok("Connection Established")
        ));
    }
    #[test]
    pub fn insert_db_failure() {
        assert!(matches!(
            insert_into_table_in_database(
                "INSERT INTO emp (emp_name) VALUES ('Rohit')".to_string(),
                "mysql://root:12345678@localhost:3306/db".to_string() // No such database named 'db'
            ),
            Err(_)
        ));
        log::error!("No such database named 'db'");
    }
    #[test]
    pub fn insert_table_failure() {
        assert!(matches!(
            insert_into_table_in_database(
                "INSERT INTO emp_tab (emp_name) VALUES ('Rohit')".to_string(), // No such table named 'emp'
                "mysql://root:12345678@localhost:3306/db_test".to_string()
            ),
            Err(_)
        ));
        log::error!("No such table named 'emp'");
    }
    #[test]
    pub fn view_db_table_success() {
        env_logger::init();
        assert!(matches!(
            view_table_in_database(
                "SELECT * from emp".to_string(),
                "mysql://root:12345678@localhost:3306/db_test".to_string()
            )
            .as_deref(),
            Ok("Connection Established")
        ));
    }
    #[test]
    pub fn view_no_db_failure() {
        assert!(matches!(
            view_table_in_database(
                "SELECT * from emp".to_string(),
                "mysql://root:12345678@localhost:3306/db".to_string() // No such database named 'db'
            ),
            Err(_)
        ));
        log::error!("No such database named 'db'");
    }
    #[test]
    pub fn view_no_table_failure() {
        assert!(matches!(
            view_table_in_database(
                "SELECT * from emp_tab".to_string(), // No such table named 'emp'
                "mysql://root:12345678@localhost:3306/db_test".to_string()
            ),
            Err(_)
        ));
        log::error!("No such table named 'emp'");
    }
}
