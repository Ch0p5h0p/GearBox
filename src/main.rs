use std::env;
use std::fs;

mod debug;
mod interpreter;
mod lexer;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let source = fs::read_to_string(filename).expect("could not read file");

    interpreter::core::interpret(source);
}
