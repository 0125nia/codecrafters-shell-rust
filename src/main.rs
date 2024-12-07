use std::io::{self, Write};

use codecrafters_shell::{core::cmd_line::CmdLine, utils::parse::parse_commands};

fn main() {
    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        initial_display();
        input.clear();
        stdin.read_line(&mut input).unwrap();

        let commands = parse_commands(&input);
        if commands.is_empty() {
            continue;
        }
        CmdLine::build(commands).and_then(|cmd_line| {
            cmd_line.execute();
            Some(())
        });
    }
}

fn initial_display() {
    print!("$ ");
    io::stdout().flush().unwrap();
}
