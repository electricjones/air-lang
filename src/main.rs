extern crate core;

use std::{env};
use crate::application::Application;

mod application;

fn main() {
    let args: Vec<String> = env::args().collect();
    let application = Application::from(args);
    application.run();
}

