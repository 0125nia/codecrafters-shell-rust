#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        initial_display();
        stdin.read_line(&mut input).unwrap();
        match input.trim().to_lowercase().as_str() {
            "exit 0" => break,
            "exit" => break,
            _ => println!("{}: command not found", input.trim()),
        }

        input.clear();
    }
}

fn initial_display() {
    print!("$ ");
    io::stdout().flush().unwrap();
}
