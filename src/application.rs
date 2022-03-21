use std::{fs, io};
use std::error::Error;
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
        if self.arguments.len() > 1 {
            match self.arguments[1].as_str() {
                "help" => {
                    println!(include_str!("application/help.txt"));
                }

                _ => {
                    let contents = self.read_file(self.arguments[1].as_str());
                    self.execute(contents.as_str());
                }
            }

            return;
        }

        // No arguments means enter the REPL
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