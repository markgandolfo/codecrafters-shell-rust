#[allow(unused_imports)]
use std::io::{self, Write};

const COMMANDS: [&str; 3] = ["exit", "echo", "type"];

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        handle_input(&input);
        input.clear();
    }
}

fn handle_input(input: &str) {
    let input: Vec<&str> = input.split_whitespace().collect();

    match input.as_slice() {
        ["exit", code] => std::process::exit(code.parse().unwrap_or(0)),
        ["echo", ..] => println!("{}", input[1..].join(" ")),
        ["type", command] => {
            if COMMANDS.contains(&command) {
                println!("{} is a shell builtin", command);
            } else {
                println!("{}: not found", command);
            }
            println!("{} is a shell builtin", command)
        }
        command => println!("{}: command not found", command.join(" ")),
    }
}
