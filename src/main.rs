use std::io::{self, Write};
use std::env;
use std::path::{PathBuf};

const BUILTIN_COMMANDS: &[&str] = &["exit", "echo", "type"];

fn is_builtin(input: &str) {
    match env::var("PATH") {
        Ok(path) => {
            let mut found = false;
            let directory = env::split_paths(&path);
            
            for dir in directory {
                let path = dir.join(input).to_string_lossy().trim().to_string();
                let path = PathBuf::from(path);
                if path.is_file(){
                    println!("{} is {}", input.trim(), path.display());
                    found = true;
                    break;
                }
            }
            if !found {
                println!("{}: not found", input)
            }
        }
        Err(e) => {
            println!("Error accessing PATH environment variable: {}", e);
        }
    }

}

fn main() {
    // REPL loop
    loop {
        // read command
        print!("$ ");
        io::stdout().flush().unwrap();
        // user input 
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.is_empty() {
            continue;
        }

        // split input into command and args
        let command_parts: Vec<&str> = input.trim().split_whitespace().collect();
        let command = command_parts[0];
        let args = &command_parts[1..];
        
        // match command and handel args
        match command {
            command if command.starts_with("exit") => std::process::exit(0),
            command if command.starts_with("echo") => println!("{}", &args.join(" ").as_str()), 
            command if command.starts_with("type") => {
                let type_arg = args[0];
                if BUILTIN_COMMANDS.contains(&type_arg){
                    println!("{} is a shell builtin", type_arg);
                } else {
                    is_builtin(type_arg)
                }
            }, 
            _ => println!("{}: command not found", input.trim())
        }
    }
} 
