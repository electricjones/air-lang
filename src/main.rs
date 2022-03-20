use interpreter::{Compile, Interpreter};

mod interpreter;
mod parser;
mod application;

fn main() {
    println!("Hello, world!");
    // let _ast = parse("1 + 2 + 3 + (4 - 5)").unwrap();

    let result = Interpreter::from_source("1 + 1 - 1 + (2 + 3)").unwrap();
    println!("Answer: `{result}`");
    println!("Goodbye!");
}
