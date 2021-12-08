extern crate mysql;

struct Emp {
    emp_id: i32,
    emp_name: String
}

fn main() {
    let pool = mysql::Pool::new("mysql://root:12345678@localhost:3306/db_test").unwrap();
    let emps: Vec<Emp> =
        pool.prep_exec("SELECT emp_id, emp_name from emp_table", ())
            .map(|result| {
                result.map(|x| x.unwrap()).map(|row| {
                    let (emp_id, emp_name) = mysql::from_row(row);
                    Emp {
                        emp_id,
                        emp_name
                    }
                }).collect()
            }).unwrap();
    for emp in emps {
        println!("{}: {}", emp.emp_id, emp.emp_name);
    }
    pool.prep_exec(
        r"INSERT INTO emp_table (emp_id, emp_name) VALUES ('3', 'Rohit')",()
    );
}