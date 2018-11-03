extern crate clap;
extern crate scytale;

use clap::{App, Arg, SubCommand};

use scytale::caesar_shift::CaesarCipher;
use scytale::frequency::{count_chars, count_ngrams, count_words};
use scytale::substitution::SubstitutionCipher;
use scytale::text_cipher::TextCipher;
use scytale::vigenere::VigenereCipher;
use std::fmt;
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

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CommandError::Unknown => write!(f, "An unknown error occurred"),
        }
    }
}

enum Command {
    Encrypt,
    Decrypt,
}

fn cmd_encrypt(command: &Command, matches: &clap::ArgMatches) -> Result<String, CommandError> {
    let input = if matches.is_present("input") {
        matches.value_of("input").unwrap().to_string()
    } else {
        get_input_from_stdin()
    };

    let key = matches.value_of("key").unwrap();

    match matches.value_of("algorithm") {
        Some("caesar") => match *command {
            Command::Encrypt => {
                Ok(CaesarCipher::new(parse_key(key).unwrap()).encrypt(input.as_str()))
            }
            Command::Decrypt => {
                Ok(CaesarCipher::new(parse_key(key).unwrap()).decrypt(input.as_str()))
            }
        },
        Some("vigenere") => match *command {
            Command::Encrypt => Ok(VigenereCipher::new(key).encrypt(input.as_str())),
            Command::Decrypt => Ok(VigenereCipher::new(key).decrypt(input.as_str())),
        },
        Some("substitution") => match *command {
            Command::Encrypt => Ok(SubstitutionCipher::from_csv(key).encrypt(input.as_str())),
            Command::Decrypt => Ok(SubstitutionCipher::from_csv(key).decrypt(input.as_str())),
        },
        _ => Err(CommandError::Unknown),
    }
}

fn cmd_frequency(matches: &clap::ArgMatches) -> Result<String, CommandError> {
    let input = if matches.is_present("input") {
        matches.value_of("input").unwrap().to_string()
    } else {
        get_input_from_stdin()
    };
    match matches.value_of("token") {
        Some("chars") => Ok(count_chars(input.as_str())),
        Some("words") => Ok(count_words(input.as_str())),
        Some("ngrams") => Ok(count_ngrams(
            input.as_str(),
            matches
                .value_of("ngram_length")
                .unwrap_or("2")
                .parse()
                .unwrap(),
        )),
        _ => Ok(count_chars(input.as_str())),
    }
}

fn main() {
    let matches = get_cli_args();

    let result = match matches.subcommand_name() {
        Some("encrypt") => cmd_encrypt(
            &Command::Encrypt,
            matches.subcommand_matches("encrypt").unwrap(),
        ),
        Some("decrypt") => cmd_encrypt(
            &Command::Decrypt,
            matches.subcommand_matches("decrypt").unwrap(),
        ),
        Some("frequency") => cmd_frequency(matches.subcommand_matches("frequency").unwrap()),
        _ => panic!("Missing or invalid subcommand"),
    };

    match result {
        Ok(r) => println!("{}", r),
        Err(e) => println!("{}", e),
    }
}

fn get_cli_args<'a>() -> clap::ArgMatches<'a> {
    App::new("cryptio")
        .version("0.0.1")
        .author("Andrew W. <andrew.wright@gmail.com>")
        .about("Utility for encrypting/decrypting")
        .subcommand(
            SubCommand::with_name("encrypt")
                .arg(
                    Arg::with_name("input")
                        .index(1)
                        .help("Text to be encrypted"),
                )
                .arg(
                    Arg::with_name("key")
                        .help("Key to use for encryption")
                        .short("k")
                        .long("key")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("algorithm")
                        .help("Algorithm to use for encryption")
                        .short("a")
                        .long("algorithm")
                        .possible_values(&["caesar", "substitution", "vigenere"])
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("decrypt")
                .arg(
                    Arg::with_name("input")
                        .index(1)
                        .help("Text to be encrypted"),
                )
                .arg(
                    Arg::with_name("key")
                        .help("Key to use for decryption")
                        .short("k")
                        .long("key")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("algorithm")
                        .help("Algorithm to use for decryption")
                        .short("a")
                        .long("algorithm")
                        .possible_values(&["caesar", "substitution", "vigenere"])
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("frequency")
                .arg(
                    Arg::with_name("input")
                        .index(1)
                        .help("Text to be frequency analysed"),
                )
                .arg(
                    Arg::with_name("token")
                        .short("t")
                        .long("token")
                        .help("token type to count frequency of")
                        .possible_values(&["chars", "words", "ngrams"])
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("ngram_length")
                        .short("l")
                        .long("ngram_length")
                        .help("Length of ngrams to count frequency of")
                        .takes_value(true),
                ),
        )
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
