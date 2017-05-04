extern crate clap;
extern crate scytale;

use clap::{Arg, App, SubCommand};

use scytale::rot13::rot13;
use std::io;
use std::io::prelude::*;

//TOTO
// - Operate on files
// - Operate on bytes
// - Operate on multiline text
// - Implement cli flags/options

fn main() {
    let matches = App::new("cryptio")
                          .version("0.0.1")
                          .author("Andrew W. <andrew.wright@gmail.com>")
                          .about("Utility for encrypting/decrypting")
                          .arg(Arg::with_name("input")
                               .index(1)
                               .help("Text to be encrypted"))
                          .get_matches();

    //TODO seems like too many conversions between
    //String and &str see also input to encrypt below
    let input = if matches.is_present("input") {
        matches.value_of("input").unwrap().to_string()
    } else {
        get_input_from_stdin()
    };

    //print encrypted text
    let ciphertext = rot13(input.as_str());
    println!("{}", ciphertext);
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
