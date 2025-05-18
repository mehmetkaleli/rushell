use std::io::{self, Write};
use std::env;
use std::path::{PathBuf};

fn is_builtin(input: &str) {
    match env::var("PATH") {
        Ok(path) => {
            let mut found = false;
            let directory = env::split_paths(&path);
            
            for dir in directory {
                let path = dir.join(input).to_string_lossy().trim().to_string();
                let path = PathBuf::from(path);
                if path.is_file(){
                    println!("{} is {}", input, path.display());
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
            command if command.starts_with("echo") => println!("{:?}", &args.get(0).unwrap()), 
            command if command.starts_with("type") => is_builtin(&input[5..]), // refactor to search PATH
            _ => println!("{}: command not found", input.trim())
        }
    }
} 
