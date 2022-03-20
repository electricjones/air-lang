use interpreter::Interpreter;

use crate::interpreter::ExecutionResult;

mod interpreter;
mod parser;
mod application;

fn main() {
    let source = "1 + 1";

    let interpreter = Interpreter::new(); // Default parser and evaluator
    match interpreter.execute(source) {
        ExecutionResult::Valid(value) => println!("Result: {value}"),
        ExecutionResult::Invalid(err) => println!("Error:  {err}")
    }
}
