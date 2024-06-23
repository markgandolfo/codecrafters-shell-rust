#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    loop {
        // Remove the newline character from the input
        input = input.trim().to_string();

        handle_input(&input);

        print!("$ ");
        io::stdout().flush().unwrap();

        input.clear();
        stdin.read_line(&mut input).unwrap();
    }
}

fn handle_input(input: &str) {
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
