use std::{fs, io};
use std::io::Write;
use air_lang::interpreter::Interpreter;

pub struct Application {
    arguments: Vec<String>,
}

impl Application {
    pub fn from(arguments: Vec<String>) -> Self {
        Application { arguments }
    }

    pub fn run(&self) {
        let arguments: Vec<&str> = self.arguments.iter().map(|s| &**s).collect();
        match arguments.as_slice() {
            // `air help` - Show the help screen
            ["help"] => {
                println!(include_str!("application/help.txt"));
            },

            // `air file.air` - Attempt to execute the file
            [path] => {
                let contents = self.read_file(&path);
                self.execute(contents.as_str())
            }

            // `air` or `air repl` - Drop into the REPL
            [] | _ => {
                println!(include_str!("application/repl.txt"));

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

                    self.execute(expression.as_str())
                }
            }
        }
    }

    pub fn execute(&self, expression: &str) {
        let interpreter = Interpreter::new();
        match interpreter.execute(expression) {
            Ok(value) => println!("Result: {value}"),
            Err(error) => eprintln!("Error:  {error}")
        }
    }

    pub fn read_file(&self, path: &str) -> String {
        let file = fs::read_to_string(path);
        if let Ok(contents) = file {
            return contents;
        } else {
            panic!("File could not be loaded");
        }
    }
}