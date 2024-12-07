use std::process;

use super::cmd_line::CmdLine;

macro_rules! commands_methods {
    ($name:ident{$($variant:ident => $string:expr => $method:expr),*$(,)?}) => {
        pub enum $name {
            $($variant),*
        }

        impl $name {
            pub fn from_str(s:&str) -> Option<Self>{
                match s {
                    $($string => Some(Self::$variant),)*
                    _ => None,
                }
            }
            pub fn execute(&self, cmd:&CmdLine) {
                match self {
                    $(Self::$variant => $method(cmd)),*
                }
            }

            pub fn commands() -> Vec<&'static str> {
                vec![
                    $($string),*
                ]
            }
        }
    };
}

commands_methods! {
    Commands {
        Echo => "echo" => handle_echo,
        Exit => "exit" => handle_exit,
        Type => "type" => handle_type,
    }

}

fn handle_echo(cmd: &CmdLine) {
    println!("{}", cmd.arguments().join(" "));
}

fn handle_exit(_cmd: &CmdLine) {
    process::exit(0);
}

fn handle_type(cmd: &CmdLine) {
    cmd.arguments().get(0).map(|arg| {
        match_type_args(arg);
    });
}

fn match_type_args(arg: &String) {
    if Commands::commands().contains(&arg.as_str()) {
        println!("{} is a shell builtin", arg);
    } else {
        if let Some(path) = get_path(arg) {
            println!("{arg} is {path}/{arg}");
        } else {
            println!("{}: not found", arg);
        }
    }
}

pub fn get_path(arg: &String) -> Option<String> {
    let paths = std::env::var("PATH").unwrap();
    paths
        .split(':')
        .find(|&path| std::fs::metadata(format!("{}/{}", path, arg)).is_ok())
        .map(|path| format!("{}/{}", path, arg))
}
