extern crate clap;
extern crate scytale;

use clap::{Arg, App, SubCommand};

use scytale::rot13::rot13;
use scytale::caesar_shift;
use std::io;
use std::io::prelude::*;
use std::num;

//TOTO
// - Operate on files
// - Operate on bytes
// - Operate on multiline text
// - Implement cli flags/options


#[derive(Debug)]
enum KeyError{
    Parse(num::ParseIntError),
}

fn main() {
    let matches = App::new("cryptio")
                          .version("0.0.1")
                          .author("Andrew W. <andrew.wright@gmail.com>")
                          .about("Utility for encrypting/decrypting")
                          .arg(Arg::with_name("input")
                               .index(1)
                               .help("Text to be encrypted"))
                          .arg(Arg::with_name("key")
                               .help("Key to use for encryption/decryption")
                               .short("k")
                               .long("key")
                               .takes_value(true))
                          .arg(Arg::with_name("algorithm")
                               .help("Algorithm to use for encryption/decryption")
                               .short("a")
                               .long("algorithm")
                               .takes_value(true))
                          .get_matches();

    //TODO seems like too many conversions between
    //String and &str see also input to encrypt below
    let input = if matches.is_present("input") {
        matches.value_of("input").unwrap().to_string()
    } else {
        get_input_from_stdin()
    };

    let key = matches.value_of("key").unwrap();

    let result = match matches.value_of("algorithm") {
        Some("rot13") => rot13(input.as_str()),
        Some("caesar") => caesar_shift::encrypt(
            input.as_str(),
            parse_key(key).unwrap(),
        ),
        _ => panic!(),
    };

    println!("{}", result);
}

fn parse_key(key: &str) -> Result<u32, KeyError> {
    key.parse().map_err(KeyError::Parse)
}

fn get_input_from_stdin() -> String {
    //read from stdin until EOF recieved
    let mut input = String::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        input = input + line.unwrap().as_str() + "\n";
    }
    input
}
