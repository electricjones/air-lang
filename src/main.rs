extern crate core;

use std::{env, io};
use std::io::Write;
use air_lang::interpreter::Interpreter;
use std::fs;
use crate::application::Application;

mod application;

fn main() {
    let args: Vec<String> = env::args().collect();
    let application = Application::from(args);
    application.run();
}

