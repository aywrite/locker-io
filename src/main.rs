extern crate scytale;

use scytale::rot13::rot13;
use std::io;

//TOTO
// - Operate on files
// - Operate on bytes
// - Operate on multiline text
// - Implement cli flags/options

fn main() {
    println!("---Please enter some text to encrypt---");
    let mut plaintext = String::new();

    io::stdin().read_line(&mut plaintext)
        .expect("Failed to read line");

    println!("You entered the text:\n {}", plaintext);

    let ciphertext = rot13(plaintext.as_str());

    println!("Output text:\n {}", ciphertext);
}
