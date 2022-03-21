use air_lang::interpreter::Interpreter;

mod application;

fn main() {
    let source = "1 + 1 - (2 + 1 + (3 - 2)) + 12";

    let interpreter = Interpreter::new();
    match interpreter.execute(source) {
        Ok(value) => println!("Result: {value}"),
        Err(error) => println!("Error:  {error}")
    }
}

