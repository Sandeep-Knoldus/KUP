#[cfg(test)]
mod tests {
    pub use crate::question::insert::insert_into_db;
    pub use crate::question::view::view_db_table;
    #[test]
    fn insert_success() {
        env_logger::init();
        assert_eq!(
            insert_into_db(
                "INSERT INTO emp_table (emp_name) VALUES ('Rohit')".to_string(),
                "mysql://root:12345678@localhost:3306/db_test".to_string()
            ),
            Some(())
        );
    }
    #[test]
    fn view_db_table_success() {
        assert_eq!(
            view_db_table(
                "SELECT emp_id, emp_name from emp_table".to_string(),
                "mysql://root:12345678@localhost:3306/db_test".to_string()
            ),
            Some(())
        );
    }
}
