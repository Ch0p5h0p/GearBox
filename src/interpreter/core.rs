use crate::lexer::{lexer::*, tokens::*};

pub fn interpret(code: String) {
    let tokens: Vec<Token> = flatten_toks(read_tokens(&code));

    println!("Reconstructed from token lexemes: ");
    for tok in tokens.iter() {
        let lexeme = tok.get_lexeme();
        print!("{lexeme} ");
    }

    println!("\n\nToken type set");

    for tok in tokens.iter() {
        let tok_type = tok.get_type();
        print!("{tok_type} ");
    }
}
