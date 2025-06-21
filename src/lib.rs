use std::{env, process};
use std::path::PathBuf;

// the command type
#[derive(Debug)]
pub struct ShellCommand {
    command: String,
    args: Vec<String>
}

impl ShellCommand {
    // parse command into command string and vector of args
    pub fn build(shell_command: &Vec<String>) -> ShellCommand {
        let command = shell_command[0].clone();
        let args = shell_command[1..].to_vec().clone();
        ShellCommand { command, args }
    }
}
// the function that runs the commands
pub fn run_command(command: ShellCommand) {
    match command.command.as_str() {
        "exit" => execute_exit(),
        "echo" => execute_echo(&command),
        "type" => execute_type(&command),
        _ => println!("{}: command not found", command.command.as_str().trim())
    }
}

fn execute_echo(cmd: &ShellCommand) {
    // print argument
    println!("{:?}", cmd.args.join(" ").as_str())
}

fn execute_exit() {
    // exit shell
    process::exit(0)
}

#[cfg(target_os = "linux")]
fn get_path() -> Vec(String) {
    let path = env::var("PATH")
        .unwrap()
        .split(":")
        .map(|x| x.to_string()).collect();
    path
}
#[cfg(not(target_os = "linux"))]
fn get_path() -> Vec<String> {
    let path = env::var("PATH")
        .unwrap()
        .split(";")
        .map(|x| x.to_string()).collect();
    path
}
fn execute_type(cmd: &ShellCommand) {}

