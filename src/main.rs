use std::io::{self, Write};

use codecrafters_shell::CmdLine;

fn main() {
    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        initial_display();
        input.clear();
        stdin.read_line(&mut input).unwrap();

        CmdLine::build(input.as_str()).and_then(|cmd_line| {
            cmd_line.execute();
            Some(())
        });
    }
}

fn initial_display() {
    print!("$ ");
    io::stdout().flush().unwrap();
}
