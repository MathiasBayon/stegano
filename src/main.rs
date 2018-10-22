pub mod lib;

use self::lib::dot_matrix::dot_matrix::DotMatrix;
use std::{env, process};

fn print_usage() {
    println!("Usage stegano <ENCODE / DECODE> <input file path> <output file path> <password> [<message if encoding]");
    process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        print_usage();
    }

    match (&args[1]).as_str() {
        "ENCODE" => {
            let mut input_file = DotMatrix::new(&args[2]);

            let encoding = input_file.encode_file(&args[5], &args[4]);
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
        "DECODE" => {
            let output_file = DotMatrix::new(&args[2]);

            let decoding = output_file.decode_file(&args[3], &args[4]);
            match decoding {
                Ok(_) => println!("Decoding.....SUCCESS"),
                Err(error) => {
                    println!("Decoding.....ERROR : {}", error);
                    process::exit(1);
                }
            }
        }
        _ => {
            print_usage();
        }
    }
}
