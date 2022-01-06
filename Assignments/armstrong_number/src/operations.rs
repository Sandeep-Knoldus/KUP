pub use crate::{armstrong_no, automorphic_no, matrix_a_b, neon_check, palindrome_seq};
pub use std::io;

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
                    let mut arr1 = [[0_usize; 2]; 2];
                    log::info!("Enter elements of 1st Matrix: ");
                    for row in &mut arr1 {
                        for e in row {
                            let mut input = String::new();
                            io::stdin().read_line(&mut input).expect("Cannot read line");
                            *e = input.trim().parse().expect("Not Integer")
                        }
                    }
                    log::info!("Matrix 1: {:?}", arr1);

                    let mut arr2 = [[0_usize; 2]; 2];
                    log::info!("Enter elements of 2nd Matrix: ");
                    for row in &mut arr2 {
                        for e in row {
                            let mut input = String::new();
                            io::stdin().read_line(&mut input).expect("Cannot read line");
                            *e = input.trim().parse().expect("Not Integer")
                        }
                    }
                    log::info!("Matrix 2: {:?}", arr2);

                    let result = matrix_a_b(arr1, arr2);
                    log::info!("{:?}", result);
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
