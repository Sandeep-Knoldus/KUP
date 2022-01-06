pub use crate::{armstrong_no, automorphic_no, matrix_a_b, neon_check, palindrome_seq};
pub use std::io;
use text_io::read;

/// Function 'op' prints the Mathematics Menu
///
/// #Arguments
///
/// No Arguments
///
/// #Return
///
/// No Return
pub fn op() {
    log::info!("-------Mathematics Menu-------");
    log::info!("     1: Armstrong Number      ");
    log::info!("     2: Automorphic Number    ");
    log::info!("     3: Matrix Multiplication ");
    log::info!("     4: Neon Number           ");
    log::info!("     5: Palindrome Number     ");
    log::info!(r#"(Press "-1" to exit)"#);
}

/// Function 'operations_main' controls the main operation of switch case
///
/// #Arguments
///
/// No Arguments
///
/// #Return
///
/// No Return
pub fn operations_main() {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Not ok");
        let trimmed = input.trim();
        let int_e = trimmed.parse::<i32>();
        match int_e {
            Ok(int_e) => match int_e {
                1 => {
                    log::info!("Enter a number: ");
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).expect("Not ok");
                    let trimmed: i32 = input.trim().parse().expect("Not integer");
                    let result = armstrong_no(trimmed);
                    if result {
                        log::info!("Armstrong")
                    } else {
                        log::error!("Not Armstrong")
                    }
                    op();
                    operations_main();
                    break;
                }
                2 => {
                    log::info!("Enter a number: ");
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).expect("Not ok");
                    let trimmed: i32 = input.trim().parse().expect("Not Integer");
                    let result = automorphic_no(trimmed);
                    if result {
                        log::info!("Automorphic")
                    } else {
                        log::error!("Not Automorphic")
                    }
                    op();
                    operations_main();
                    break;
                }
                3 => {
                    log::info!("Enter no. of rows of 1st matrix");
                    let row_1: i32 = read!();
                    log::info!("Enter no. of columns of 1st matrix");
                    let column_1: i32 = read!();
                    let mut first_matrix: Vec<Vec<i32>> = Vec::new();
                    log::info!("Enter the values of 1st matrix");
                    for _loop1 in 0..row_1 {
                        let mut input: Vec<i32> = Vec::new();
                        for _loop2 in 0..column_1 {
                            input.push(read!())
                        }
                        first_matrix.push(input);
                    }
                    log::info!("Enter no. of rows of 2nd matrix");
                    let row_2: i32 = read!();
                    log::info!("Enter no. of columns of 2nd matrix");
                    let column_2: i32 = read!();
                    let mut second_matrix: Vec<Vec<i32>> = Vec::new();
                    log::info!("Enter the values of 2nd matrix");
                    for _loop1 in 0..row_2 {
                        let mut vec: Vec<i32> = Vec::new();
                        for _loop2 in 0..column_2 {
                            vec.push(read!())
                        }
                        second_matrix.push(vec);
                    }
                    log::debug!("{:?}", matrix_a_b(&first_matrix, &second_matrix));
                    op();
                    operations_main();
                    break;
                }
                4 => {
                    log::info!("Enter a number: ");
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).expect("Not ok");
                    let trimmed: i32 = input.trim().parse().expect("Not integer");
                    let result = neon_check(trimmed);
                    if result {
                        log::info!("Neon")
                    } else {
                        log::error!("Not Neon")
                    }
                    op();
                    operations_main();
                    break;
                }
                5 => {
                    log::info!("Enter a number: ");
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).expect("Not ok");
                    let trimmed: i32 = input.trim().parse().expect("Not integer");
                    let result = palindrome_seq(trimmed);
                    if result {
                        log::info!("Palindrome")
                    } else {
                        log::error!("Not Palindrome")
                    }
                    op();
                    operations_main();
                    break;
                }
                -1 => break,
                _ => {
                    log::debug!(r#"Choose between "1" to "5" or "-1" to exit"#);
                }
            },
            Err(_) => log::error!("Not integer"),
        }
    }
}
