use std::env;
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
                // match env::var("PATH")
                //     .unwrap()
                //     .split(":")
                //     .map(|path| format!("{}/{}", path, command))
                //     .find(|path| std::fs::metadata(path).is_ok())
                // {
                //     Some(path) => println!("{} is {}", command, path),
                //     _ => println!("{}: not found", command),
                // }
            }
        }
        // command => println!("{}: not found", command.join(" ")),
        command => {
            let cmd_string = command.join(" ");
            match find_in_path(
                env::var("PATH").unwrap_or_else(|_| "".to_string()).as_str(),
                &cmd_string,
            ) {
                Some(path) => {
                    let mut cmd = std::process::Command::new(path);
                    let status = cmd.status().unwrap();
                    if !status.success() {
                        println!("{}: not found", cmd_string);
                    }
                }
                _ => println!("{}: command not found", cmd_string),
            }
        }
    }
}
