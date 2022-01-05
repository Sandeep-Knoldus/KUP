use std::fs::File;
pub use crate::{armstrong_no, automorphic_no, matrix_a_b, neon_check, palindrome_seq};
pub use std::io;
use std::io::{BufRead, BufReader};

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
                    if result == true {
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
                    if result == true {
                        log::info!("Automorphic")
                    } else {
                        log::error!("Not Automorphic")
                    }
                    op();
                    operations_main();
                    break;
                }
                3 => {
                    let mut f = BufReader::new(File::open("src/matrix1").expect("Cannot open file"));

                    let mut num_line = String::new();
                    f.read_line(&mut num_line).expect("Cannot read line");
                    let n: usize = num_line[1..].trim().parse().expect("Not integer");

                    let mut arr = vec![vec![0i32; n]; n];
                    for (i, line) in f.lines().enumerate() {
                        for (j, number) in line.unwrap().split(char::is_whitespace).enumerate() {
                            arr[i][j] = number.trim().parse().unwrap();
                        }
                    }

                    let mut f = BufReader::new(File::open("src/matrix2").expect("Cannot open file"));

                    let mut num_line = String::new();
                    f.read_line(&mut num_line).expect("Cannot read line");
                    let n: usize = num_line[1..].trim().parse().expect("Not integer");

                    let mut arr2 = vec![vec![0i32; n]; n];
                    for (i, line) in f.lines().enumerate() {
                        for (j, number) in line.unwrap().split(char::is_whitespace).enumerate() {
                            arr2[i][j] = number.trim().parse().unwrap();
                        }
                    }
                    let res = matrix_a_b(arr, arr2);
                    log::info!("{:?}", res);
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
                    if result == true {
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
                    if result == true {
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
