pub mod lib;

use self::lib::dot_matrix::dot_matrix::DotMatrix;
use std::{env, process};

enum Usage {
    FULL,
    ENCODE,
    DECODE,
}

fn print_usage(mode: Usage) {
    match mode {
        Usage::FULL => println!("Usage stegano <ENCODE / DECODE> <input file path> <output file path> <password> [<message if encoding]"),
        Usage::ENCODE => println!("Usage stegano ENCODE <input file path> <output file path> <password> <message>"),
        Usage::DECODE => println!("Usage stegano DECODE <input file path> <password>"),
    }
    process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        print_usage(Usage::FULL);
    }

    match &args[1].as_str() {
        &"ENCODE" => {
            if args.len() != 6 {
                print_usage(Usage::ENCODE);
            }

            let mut input_file = DotMatrix::new(&args[2]);

            let encoding = input_file.encode(&args[5], &args[4]);
            match encoding {
                Ok(_) => println!("Encoding.....SUCCESS"),
                Err(error) => {
                    println!("Encoding.....ERROR : {}", error);
                    process::exit(1);
                }
            }

            let writing = input_file.write_to_file(&args[3]);
            match writing {
                Ok(_) => println!("Writing......SUCCESS"),
                Err(error) => {
                    println!("Writing......ERROR : {}", error);
                    process::exit(1);
                }
            }
        }
        &"DECODE" => {
            if args.len() != 5 {
                print_usage(Usage::DECODE);
            }

            let output_file = DotMatrix::new(&args[2]);

            let decoding = output_file.decode(&args[3]);
            match decoding {
                Ok(res) => println!("Decoding.....SUCCESS : {}", res),
                Err(error) => {
                    println!("Decoding.....ERROR : {}", error);
                    process::exit(1);
                }
            }
        }
        _ => {
            print_usage(Usage::FULL);
        }
    }
}
