extern crate scytale;

use scytale::rot13::rot13;
use std::io;
use std::io::prelude::*;

//TOTO
// - Operate on files
// - Operate on bytes
// - Operate on multiline text
// - Implement cli flags/options

fn main() {
    
    //read from stdin until EOF recived
    let mut plaintext = String::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        plaintext = plaintext + line.unwrap().as_str() + "\n";
    }

    //print encrypted text
    let ciphertext = rot13(plaintext.as_str());
    println!("{}", ciphertext);
}
