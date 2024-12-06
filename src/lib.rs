use std::process;

#[allow(unused)]
pub struct CmdLine {
    command: String,
    options: Vec<String>,
    arguments: Vec<String>,
}

impl CmdLine {
    pub fn build(line: &str) -> Option<CmdLine> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            return None;
        }
        let command = parts.get(0)?.to_string();
        // let arguments,options = parts[1..].iter().partition(|&arg|arg.starts_with("-"));
        let (options, arguments) = parts[1..]
            .iter()
            .map(|arg| arg.to_string())
            .partition(|arg| arg.starts_with("-"));
        Some(CmdLine {
            command,
            arguments,
            options,
        })
    }

    pub fn execute(&self) {
        let cmd = self.command.to_lowercase();
        let cmd = cmd.as_str();
        match cmd {
            "exit" => process::exit(0),
            "echo" => handle_echo(self),
            _ => println!("{}: command not found", cmd),
        }
    }
}

fn handle_echo(cmd: &CmdLine) {
    println!("{}", cmd.arguments.join(" "));
}
