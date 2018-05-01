pub mod lib;
use self::lib::dot_matrix::dot_matrix::DotMatrix;
use std::io::{stdin, stdout, Write};

fn main() {
    // Little terminal interface, change into graphical interface when possible
    let mut input = String::new();
    let mut output = String::new();
    let mut message = String::new();
    let mut password = String::new();

    print!("Please enter input file path : ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut input)
        .expect("Did not enter a correct string");

    print!("Please enter message : ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut message)
        .expect("Did not enter a correct string");

    print!("Please enter password : ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut password)
        .expect("Did not enter a correct string");

    print!("Please enter output file path : ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut output)
        .expect("Did not enter a correct string");

    let mut input_file = DotMatrix::new(input.trim());
    let _ = input_file.encode(message.trim(), password.trim());
    let _ = input_file.write_to_file(output.trim());
    let output_file = DotMatrix::new(output.trim());
    let res = output_file.decode(password.trim());

    match res {
        Ok(res2) => println!("{}", res2),
        Err(error) => println!("{}", error),
    }
}
