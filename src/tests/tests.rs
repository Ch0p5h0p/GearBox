use crate::lexer::{lexer::*, tokens::*};
use std::fs;

fn test_suite(code_file: &str, token_file: Option<&str>) {
    let source = fs::read_to_string(code_file).expect("could not read source file");

    println!("TOKEN DATA");
    print_token_table(flatten_toks(read_tokens(&source)));

    //lexer_source_reconstruction(&source);

    match token_file {
        Some(s) => {
            let token_set = fs::read_to_string(s).expect("could not read token file");
            token_accuracy(&source, &token_set);
        }
        None => {}
    }
}

fn token_accuracy(source: &str, token_set: &str) {
    let source_tokens: Vec<Token> = flatten_toks(read_tokens(source));
    let token_set: Vec<String> = token_set.split('\n').map(|s| s.to_string()).collect();

    for i in 0..source_tokens.len() {
        // assert each token type and lexeme is equal to the one in the file
        todo!("token_accuracy equivalence check");
    }
}

#[test]
fn test_basic() {
    test_suite(
        "src/tests/test_files/test_basic.grb",
        Some("src/tests/test_files/test_basic.grb.tok"),
    );
}
