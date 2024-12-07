use std::process::Command;

use crate::core::commands::Commands;
use crate::utils::path::get_path;

#[allow(unused)]
pub struct CmdLine {
    command: String,
    arguments: Vec<String>,
}

/// [CmdLine] build and some methode of member variable gettings
impl CmdLine {
    pub fn build(commands: Vec<String>) -> Option<CmdLine> {
        let command = commands.get(0).unwrap().to_string();

        let arguments: Vec<String> = commands[1..]
            .iter()
            .map(|command| command.to_string())
            .collect();

        Some(CmdLine { command, arguments })
    }

    pub fn command(&self) -> &str {
        &self.command
    }

    pub fn arguments(&self) -> &Vec<String> {
        &self.arguments
    }
}

/// execute the cmd
impl CmdLine {
    pub fn execute(&self) {
        let cmd = self.command.to_lowercase();
        if let Some(command) = Commands::from_str(cmd.as_str()) {
            command.execute(self);
        } else {
            self.execute_external(&cmd);
        }
    }

    fn execute_external(&self, cmd: &String) {
        let path = get_path(&cmd);
        if let Some(path) = path {
            if Command::new(path).args(&self.arguments).status().is_err() {
                println!("{} execute error", cmd)
            };
        } else {
            println!("{}: command not found", cmd);
        }
    }
}
