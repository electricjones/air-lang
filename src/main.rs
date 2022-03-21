use std::{env, io};
use std::io::Write;
use air_lang::interpreter::Interpreter;
use std::fs;

mod application;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // We were given some arguments, so deal with that
        match args[1].as_str() {
            "help" => {
                println!(r"
This is the help page!
            ");
            }

            _ => {
                // Try to load a file
                let file = fs::read_to_string(&args[1]);

                if let Ok(contents) = file {
                    execute(contents.as_str());
                } else {
                    eprintln!("File could not be loaded");
                }
            }
        }

        return;

    }

    // No arguments means enter the REPL
    println!(r"
           _        _
     /\   (_)      | |
    /  \   _ _ __  | |     __ _ _ __   __ _
   / /\ \ | | '__| | |    / _` | '_ \ / _` |
  / ____ \| | |    | |___| (_| | | | | (_| |
 /_/    \_\_|_|    |______\__,_|_| |_|\__, |
                                       __/ |
                                      |___/
    ");
    println!("Welcome to the REPL!");
    println!("Type an expression and hit <Enter> to see it evaluated. When Finished, type `exit`");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut expression = String::new();

        io::stdin().read_line(&mut expression)
            .ok()
            .expect("Failed to read line");

        expression.truncate(expression.trim_end().len());

        // Escape Hatch
        if expression == "exit" {
            println!("Goodbye!");
            break;
        }

        execute(expression.as_str())
    }
}

fn execute(expression: &str) {
    let interpreter = Interpreter::new();
    match interpreter.execute(expression) {
        Ok(value) => println!("Result: {value}"),
        Err(error) => println!("Error:  {error}")
    }
}

