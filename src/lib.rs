use std::{env, process};
use std::ffi::OsString;
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

fn execute_type(cmd: &ShellCommand) {
    // check if command is builtin
    if is_builtin(&cmd.args[0]) {
        println!("{} is a shell builtin", cmd.args[0]);
    } else {
        // get path env variable
        let paths = get_path();
        let cmd_arg = cmd.args[0].as_str();
        // split paths directories, join directories with cmd_arg
        // find the file in PATH
        match env::split_paths(&paths)
            .map(|path| path.join(cmd_arg))
            .find(|path| path.is_file())
        {
            Some(found_path) => {
                println!("{} is {}", cmd_arg, found_path.parent().unwrap_or(&found_path).display())
            }
            None => {
                println!("{}: not found", cmd_arg);
            }
        }
    }
}

fn execute_program(cmd: &ShellCommand) {}



fn get_path() -> OsString {
    env::var_os("PATH").unwrap()
}

fn is_builtin(cmd: &String) -> bool {
    let builtin_commands = vec!["exit", "echo", "type"];
    if builtin_commands.iter().any(|&c| c==cmd) {
        true
    } else {
        false
    }
}