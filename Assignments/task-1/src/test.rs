#[cfg(test)]
pub mod tests {
    pub use crate::question::insert::insert_into_table_in_database;
    pub use crate::question::view::view_table_in_database;
    #[test]
    pub fn insert_success() {
        env_logger::init();
        assert_eq!(
            insert_into_table_in_database(
                "INSERT INTO emp_table (emp_name) VALUES ('Rohit')".to_string(),
                "mysql://root:12345678@localhost:3306/db_test".to_string()
            ),
            Some(())
        );
    }
    #[test]
    pub fn insert_failure() {
        assert_eq!(
            insert_into_table_in_database(
                "INSERT INTO emp_table (emp_name) VALUES ('Rohit')".to_string(),
                "mysql://root:12345678@localhost:3306/db".to_string() // No such database named 'db'
            ),
            None
        );
        log::error!("No such database named 'db'");
    }
    #[test]
    pub fn insert_fail() {
        assert_eq!(
            insert_into_table_in_database(
                "INSERT INTO emp (emp_name) VALUES ('Rohit')".to_string(), // No such table named 'emp'
                "mysql://root:12345678@localhost:3306/db_test".to_string()
            ),
            None
        );
        log::error!("No such table named 'emp'");
    }
    #[test]
    pub fn view_db_table_success() {
        assert_eq!(
            view_table_in_database(
                "SELECT emp_id, emp_name from emp_table".to_string(),
                "mysql://root:12345678@localhost:3306/db_test".to_string()
            ),
            Some(())
        );
    }
    #[test]
    pub fn view_db_table_failure() {
        assert_eq!(
            view_table_in_database(
                "SELECT emp_id, emp_name from emp_table".to_string(),
                "mysql://root:12345678@localhost:3306/db".to_string() // No such database named 'db'
            ),
            None
        );
        log::error!("No such database named 'db'");
    }
    #[test]
    pub fn view_db_table_fail() {
        assert_eq!(
            view_table_in_database(
                "SELECT emp_id, emp_name from emp".to_string(), // No such table named 'emp'
                "mysql://root:12345678@localhost:3306/db_test".to_string()
            ),
            None
        );
        log::error!("No such table named 'emp'");
    }
}
