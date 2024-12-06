#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        initial_display();
        stdin.read_line(&mut input).unwrap();
        input.clear();
        println!("{}: command not found", input.trim());
    }
}

fn initial_display() {
    print!("$ ");
    io::stdout().flush().unwrap();
}
