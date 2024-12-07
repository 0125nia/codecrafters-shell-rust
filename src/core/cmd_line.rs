#[allow(unused)]
pub struct CmdLine {
    command: String,
    options: Vec<String>,
    arguments: Vec<String>,
}
impl CmdLine {
    pub fn command(&self) -> &str {
        &self.command
    }

    pub fn arguments(&self) -> &[String] {
        &self.arguments
    }

    pub fn options(&self) -> &[String] {
        &self.options
    }
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
        if let Some(command) = super::commands::Commands::from_str(cmd.as_str()) {
            command.execute(self);
        } else {
            println!("{}: command not found", cmd);
        }
    }
}
