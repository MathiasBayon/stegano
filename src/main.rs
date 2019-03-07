//! MAIN
pub mod lib;

use self::lib::dot_matrix::DotMatrix;
use std::{env, process};

// Enum used to display usage depending on first argument entered by user
enum Usage {
    FULL,
    ENCODE,
    DECODE,
}

/// Print stegano usage
/// This function uses first input argument to display a more contextual usage
fn print_usage(mode: Usage) {
    match mode {
        Usage::FULL =>
            println!("Usage stegano <ENCODE / DECODE> <input file path> <output file path> <password> [<ASCII file to encode, if encoding>]"),
        Usage::ENCODE =>
            println!("Usage stegano ENCODE <input file path> <output file path> <password>"),
        Usage::DECODE =>
            println!("Usage stegano DECODE <input file path> <output file path> <password>"),
    }
}

/// Sub main, for encoding mode
fn main_sub_encode(args: &[String]) {
    // Check input arguments number
    if args.len() != 6 {
        print_usage(Usage::ENCODE);
    }

    // Initialize dot matrix from input file
    let mut input_file = DotMatrix::new(&args[2]);

    // Encode input file within matrix
    let encoding = input_file.encode_file(&args[5], &args[4]);

    // Check success!
    match encoding {
        Ok(_) => {
            println!("Encoding.....SUCCESS");

            // In case of success (I hope so!!), write encoded result into new file
            let writing = input_file.write_to_file(&args[3]);
            match writing {
                // Then check success, again
                Ok(_) => {
                    println!("Writing......SUCCESS");
                    process::exit(0);
                }
                Err(error) => {
                    println!("Writing......ERROR : {}", error);
                }
            }

        },
        Err(error) => {
            println!("Encoding.....ERROR : {}", error);
        }
    }
}

/// Sub main, for decoding mode
fn main_sub_decode(args: &[String]) {
    // Check input arguments number
    if args.len() != 5 {
        print_usage(Usage::DECODE);
    }

    // Initialize dot matrix from input file
    let output_file = DotMatrix::new(&args[2]);

    // Decode input file within matrix
    let decoding = output_file.decode_and_write(&args[3], &args[4]);

    // Check success
    match decoding {
        Ok(_) => {
            println!("Decoding.....SUCCESS");
            process::exit(0);
        }
        Err(error) => {
            println!("Decoding.....ERROR : {}", error);
        }
    }
}

/// MAIN
fn main() {
    // Collect input arguments into vector
    let args: Vec<String> = env::args().collect();

    // If user just called stegano without any arguments
    // display full usage message
    if args.len() < 2 {
        print_usage(Usage::FULL);
    }

    // Analyse first argument
    match (&args[1]).as_str() {
        "ENCODE" => {
            main_sub_encode(&args);
        }
        "DECODE" => {
            main_sub_decode(&args);
        }
        _ => {
            print_usage(Usage::FULL);
        }
    }

    // Ciao!
    process::exit(1);
}
