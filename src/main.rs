#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    match input.trim() {
        "exit" => {
            println!("Goodbye!");
            std::process::exit(0);
        }
        command => {
            println!("{}: command not found", command);
        }
    }
}
