use std::env;
use std::io::{self, Write};
use std::process::Command;

const COMMANDS: [&str; 4] = ["exit", "echo", "type", "pwd"];

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

fn find_in_path(path_var: &str, filename: &str) -> Option<String> {
    let paths = env::split_paths(&path_var);

    for path in paths {
        let full_path = path.join(filename);
        if full_path.exists() && full_path.is_file() {
            return Some(full_path.to_string_lossy().into_owned());
        }
    }

    None
}

fn handle_input(input: &str) {
    let input: Vec<&str> = input.split_whitespace().collect();

    match input.as_slice() {
        ["exit", code] => std::process::exit(code.parse().unwrap_or(0)),
        ["echo", ..] => println!("{}", input[1..].join(" ")),
        ["pwd"] => println!("{}", env::current_dir().unwrap().to_string_lossy()),
        ["type", command] => {
            if COMMANDS.contains(&command) {
                println!("{} is a shell builtin", command);
            } else {
                match find_in_path(
                    env::var("PATH").unwrap_or_else(|_| "".to_string()).as_str(),
                    command,
                ) {
                    Some(path) => println!("{} is {}", command, path),
                    _ => println!("{}: not found", command),
                }
            }
        }
        command => match command {
            [cmd, rest @ ..] => {
                match find_in_path(
                    env::var("PATH").unwrap_or_else(|_| "".to_string()).as_str(),
                    cmd,
                ) {
                    Some(path) => match Command::new(path).args(rest).output() {
                        Ok(output) => {
                            io::stdout().write_all(&output.stdout).unwrap();
                            io::stderr().write_all(&output.stderr).unwrap();
                        }
                        Err(e) => eprintln!("{}", e),
                    },
                    None => println!("{}: command not found", cmd),
                }
            }
            _ => println!("{}: not found", command.join(" ")),
        },
    }
}
