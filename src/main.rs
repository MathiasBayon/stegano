pub mod lib;

use self::lib::dot_matrix::dot_matrix::DotMatrix;
use std::{io::{stdin, stdout, Write}, env, process};

#[allow(dead_code)]
fn main_prompts() {
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

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage {} <ENCODE / DECODE> <input file path> <output file path> <password> [<message if encoding]", &args[0]);
        process::exit(1);
    }

    match &args[1].as_str() {
        &"ENCODE" => {
            if args.len() != 6 {
                println!("Usage {} ENCODE <input file path> <output file path> <password> <message>", &args[0]);
                process::exit(1);
            }

            let mut input_file = DotMatrix::new(&args[2]);

            let encoding = input_file.encode(&args[5], &args[4]);
            match encoding {
                Ok(_) => println!("Encoding.....SUCCESS"),
                Err(error) => { println!("Encoding.....ERROR : {}", error); process::exit(1); }
            }

            let writing = input_file.write_to_file(&args[3]);
            match writing {
                Ok(_) => println!("Writing......SUCCESS"),
                Err(error) => { println!("Writing......ERROR : {}", error); process::exit(1); }
            }
        },
        &"DECODE" => {
            if args.len() != 5 {
                println!("Usage {} DECODE <input file path> <password>", &args[0]);
                process::exit(1);
            }

            let output_file = DotMatrix::new(&args[2]);

            let decoding = output_file.decode(&args[3]);
                match decoding {
                Ok(res) => println!("Decoding.....SUCCESS : {}", res),
                Err(error) => { println!("Decoding.....ERROR : {}", error); process::exit(1); }
            }
        },
        _ => {
            println!("Usage {} <ENCODE / DECODE> <input file path> <output file path> <password> [<message if encoding]", &args[0]);
            process::exit(1);
        },
    }
}
