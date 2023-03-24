use std::io;
// Import the _ascii_to_morse function from another module
use morse_code_converter::{_ascii_to_morse, _morse_to_ascii};

// write a main function
fn main() {
    let mut input = String::new();

    println!("Enter a string to convert:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();
    if !input.is_empty() {
        let output = _ascii_to_morse(input);
        println!("{}", output);
    } else {
        println!("Input is empty or contains only whitespace.");
    }

    let mut input = String::new();

    println!("Enter Morse code to convert:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();
    if !input.is_empty() {
        let output = _morse_to_ascii(input);
        println!("{}", output);
    } else {
        println!("Input is empty or contains only whitespace.");
    }
}
