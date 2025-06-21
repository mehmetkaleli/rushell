use std::io::{self, Write};
use rushell::ShellCommand;

fn main() {
    // REPL loop
    loop {
        // read command
        print!("$ ");
        io::stdout().flush().unwrap();
        // user input 
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        // if user input is empty then continue and reinter command
        // .trim removes any empty space, otherwise the if statement will cause a crash
        if input.trim().is_empty() {
            continue;
        }
        // split command into [command, arg] vector
        let command_parts: Vec<String> = input.split_whitespace()
                                                .map(|s| s.to_string())
                                                .collect();
        // build command
        let shell_command = ShellCommand::build(&command_parts);

        // run command
        rushell::run_command(shell_command);
    } 
} 

