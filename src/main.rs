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
        println!("{}: command not found", input.trim());
    }
}

fn main() {
    repl();
}
