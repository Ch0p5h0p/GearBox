use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let source = fs::read_to_string(filename).expect("could not read file");

    gearbox::interpreter::core::interpret(source);
}
