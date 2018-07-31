#[macro_use]
extern crate log;
extern crate env_logger;


use std::io::{stdin,stdout,Write};
use std::env;
use std::io::prelude::*;

mod lexical;
use lexical::filereader::FileReader;
use lexical::scanner::Scanner;



fn main() {
    // Initialize logger without a timestamp
    env_logger::Builder::from_default_env()
            .default_format_timestamp(false)
            .init();

    // Get arguments from the command line
    let args: Vec<String> = env::args().collect();

    // First arg is file name, second is path if it is given
    match args.as_slice() {
        [_] => {
            println!("Not yet implemented...");
            run_prompt()
        },
        [_, path] => {
            run_file(path)
        },
        _ => println!("More arguments than expected were given...")
    }
}

fn run_prompt() {
    let stdin = stdin();
    let mut handle = stdin.lock();
    
    loop {
        print!("> ");
        stdout().flush().unwrap();

        // Read input from the stdin
        let mut input = String::new();
        match handle.read_line(&mut input) {
            Ok(_) => {
                let scanner = Scanner::new(&input);

                for t in scanner {
                    println!("{:?}", t);
                }
            },
            Err(error) => error!("Error: {}", error),
        }
    }
}

fn run_file(path: &str) {
    let mut reader = FileReader::new(path);
    let source : &str = reader.get_content();
    let scanner = Scanner::new(source);
    println!("{}", source);

    for t in scanner {
        println!("{:?}", t);
    }
}