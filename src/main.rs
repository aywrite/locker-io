extern crate clap;
extern crate scytale;

use clap::{Arg, App, SubCommand};

use scytale::rot13::rot13;
use scytale::caesar_shift;
use scytale::vigenere;
use std::io;
use std::io::prelude::*;
use std::num;

// TODO
// - Operate on files
// - Operate on bytes
// - Implement sub cmd analyse
// - Implement sub cmd crack


#[derive(Debug)]
enum KeyError {
    Parse(num::ParseIntError),
}

#[derive(Debug)]
enum CommandError {
    Unknown,
}

enum Command {
    Encrypt,
    Decrypt,
}

fn run_command(command: &Command, matches: clap::ArgMatches) -> Result<String, CommandError> {

    let matches = match *command {
        Command::Encrypt => matches.subcommand_matches("encrypt").unwrap(),
        Command::Decrypt => matches.subcommand_matches("decrypt").unwrap(),
    };

    //TODO seems like too many conversions between
    //String and &str see also input to encrypt below
    let input = if matches.is_present("input") {
        matches.value_of("input").unwrap().to_string()
    } else {
        get_input_from_stdin()
    };

    let key = matches.value_of("key").unwrap();

    //TODO remove nested match, replace with dynamic calls to algos?
    match matches.value_of("algorithm") {
        Some("rot13") => Ok(rot13(input.as_str())),
        Some("caesar") => {
            match *command {
                Command::Encrypt => {
                    Ok(caesar_shift::encrypt(input.as_str(), parse_key(key).unwrap()))
                }
                Command::Decrypt => {
                    Ok(caesar_shift::decrypt(input.as_str(), parse_key(key).unwrap()))
                }
            }
        }
        Some("vigenere") => {
            match *command {
                Command::Encrypt => Ok(vigenere::encrypt(input.as_str(), key)),
                Command::Decrypt => Ok(vigenere::decrypt(input.as_str(), key)),
            }
        }
        _ => Err(CommandError::Unknown),
    }
}



fn main() {

    let matches = get_cli_args();

    let command = if matches.is_present("encrypt") {
        Command::Encrypt
    } else if matches.is_present("decrypt") {
        Command::Decrypt
    } else {
        panic!("Missing or invalid subcommand")
    };

    let result = run_command(&command, matches).expect("Bother");
    println!("{}", result);
}

fn get_cli_args<'a>() -> clap::ArgMatches<'a> {
    App::new("cryptio")
        .version("0.0.1")
        .author("Andrew W. <andrew.wright@gmail.com>")
        .about("Utility for encrypting/decrypting")
        .subcommand(SubCommand::with_name("encrypt")
                        .arg(Arg::with_name("input")
                                 .index(1)
                                 .help("Text to be encrypted"))
                        .arg(Arg::with_name("key")
                                 .help("Key to use for encryption")
                                 .short("k")
                                 .long("key")
                                 .takes_value(true))
                        .arg(Arg::with_name("algorithm")
                                 .help("Algorithm to use for encryption")
                                 .short("a")
                                 .long("algorithm")
                                 .possible_values(&["rot13", "caesar", "vigenere"])
                                 .takes_value(true)))
        .subcommand(SubCommand::with_name("decrypt")
                        .arg(Arg::with_name("input")
                                 .index(1)
                                 .help("Text to be encrypted"))
                        .arg(Arg::with_name("key")
                                 .help("Key to use for decryption")
                                 .short("k")
                                 .long("key")
                                 .takes_value(true))
                        .arg(Arg::with_name("algorithm")
                                 .help("Algorithm to use for decryption")
                                 .short("a")
                                 .long("algorithm")
                                 .possible_values(&["rot13", "caesar", "vigenere"])
                                 .takes_value(true)))
        .get_matches()
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
