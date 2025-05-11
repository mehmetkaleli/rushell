#[allow(unused_imports)]
use std::io::{self, Write};

fn repl(){
    loop {
        // read command
        print!("$ ");
        io::stdout().flush().unwrap();

        // user input 
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        // match command
        match input.trim() {
            "exit 0" => std::process::exit(0),
            input if input.starts_with("echo") => println!("{}", &input[5..]), 
            _ => println!("{}: command not found", input.trim())
        }
    }
}

fn main() {
    repl();
}
