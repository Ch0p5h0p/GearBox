// from https://craftinginterpreters.com/scanning.html

use super::tokens::{Token, TokenType::*};
use crate::debug::errors::*;

// Convert code into a token stream
pub fn read_tokens(code: &str) -> Vec<Result<Token, LexerError>> {
    let mut toks: Vec<Result<Token, LexerError>> = Vec::new();

    let mut line = 1;
    let mut code_chars = code.chars().peekable();

    while let Some(c) = code_chars.next() {
        if c == '\n' {
            line += 1;
            continue;
        }

        toks.push(match c {
            // Single-char tokens
            '(' => Ok(Token::new(LEFT_PAREN, "(".to_string(), line)),
            ')' => Ok(Token::new(RIGHT_PAREN, ")".to_string(), line)),
            '{' => Ok(Token::new(LEFT_BRACE, "{".to_string(), line)),
            '}' => Ok(Token::new(RIGHT_BRACE, "}".to_string(), line)),
            ',' => Ok(Token::new(COMMA, ",".to_string(), line)),
            '.' => Ok(Token::new(DOT, ".".to_string(), line)),
            '-' => Ok(Token::new(DASH, "-".to_string(), line)),
            '+' => Ok(Token::new(PLUS, "+".to_string(), line)),
            ';' => Ok(Token::new(SEMICOLON, ";".to_string(), line)),
            ':' => Ok(Token::new(COLON, ":".to_string(), line)),
            '*' => Ok(Token::new(STAR, "*".to_string(), line)),

            // Single- or double-char tokens
            '!' => {
                if code_chars.peek() == Some(&'=') {
                    code_chars.next();
                    Ok(Token::new(BANG_EQUAL, "!=".to_string(), line))
                } else {
                    Ok(Token::new(BANG, "!".to_string(), line))
                }
            }
            '=' => {
                if code_chars.peek() == Some(&'=') {
                    code_chars.next();
                    Ok(Token::new(EQUAL_EQUAL, "==".to_string(), line))
                } else {
                    Ok(Token::new(EQUAL, "=".to_string(), line))
                }
            }
            '>' => {
                if code_chars.peek() == Some(&'=') {
                    code_chars.next();
                    Ok(Token::new(GREATER_EQUAL, ">=".to_string(), line))
                } else {
                    Ok(Token::new(GREATER, ">".to_string(), line))
                }
            }
            '<' => {
                if code_chars.peek() == Some(&'=') {
                    code_chars.next();
                    Ok(Token::new(LESS, "<=".to_string(), line))
                } else {
                    Ok(Token::new(LESS_EQUAL, "<".to_string(), line))
                }
            }
            '&' => {
                if code_chars.peek() == Some(&'=') {
                    code_chars.next();
                    Ok(Token::new(AMP_EQUAL, "&=".to_string(), line))
                } else if code_chars.peek() == Some(&'&') {
                    code_chars.next();
                    Ok(Token::new(DOUBLE_AMP, "&&".to_string(), line))
                } else {
                    Ok(Token::new(AMPERSAND, "&".to_string(), line))
                }
            }
            '|' => {
                if code_chars.peek() == Some(&'=') {
                    code_chars.next();
                    Ok(Token::new(BAR_EQUAL, "|=".to_string(), line))
                } else if code_chars.peek() == Some(&'|') {
                    code_chars.next();
                    Ok(Token::new(DOUBLE_BAR, "||".to_string(), line))
                } else {
                    Ok(Token::new(BAR, "|".to_string(), line))
                }
            }
            '^' => {
                if code_chars.peek() == Some(&'=') {
                    code_chars.next();
                    Ok(Token::new(CARET_EQUAL, "^=".to_string(), line))
                } else {
                    Ok(Token::new(CARET, "^".to_string(), line))
                }
            }
            '@' => {
                if code_chars.peek() == Some(&'@') {
                    code_chars.next();
                    Ok(Token::new(DOUBLE_AT, "@@".to_string(), line))
                } else {
                    Ok(Token::new(AT, "@".to_string(), line))
                }
            }

            // Comments, whitespaces
            '/' => {
                if code_chars.peek() == Some(&'/') {
                    /*while code_chars.peek() != Some(&'\n') {
                        code_chars.next();
                    }*/

                    while let Some(&c) = code_chars.peek() {
                        if c == '\n' {
                            break;
                        }

                        code_chars.next();
                    }
                    continue;
                } else {
                    Ok(Token::new(SLASH, "/".to_string(), line))
                }
            }
            '\n' => {
                line += 1;
                continue; //Ok(Token::new(WHITESPACE, '\n'.to_string(), line))
            }
            '\r' | '\t' | ' ' => continue, //Ok(Token::new(WHITESPACE, ' '.to_string(), line)),

            // Strings, numbers, and identifiers
            '"' => {
                let mut string_chars: Vec<char> = Vec::new();
                string_chars.push(c);

                let result = loop {
                    let Some(c) = code_chars.next() else {
                        break Err(LexerError::UnterminatedString);
                    };

                    if c == '"' {
                        string_chars.push('"');
                        break Ok(Token::new(
                            STRING,
                            string_chars.iter().collect::<String>(),
                            line,
                        ));
                    }

                    string_chars.push(c);
                };

                result
            }
            c if c.is_ascii_digit() => {
                let mut num_chars: Vec<char> = Vec::new();
                num_chars.push(c);

                let result = loop {
                    if code_chars.peek().is_none() || !code_chars.peek().unwrap().is_ascii_digit() {
                        break Ok(Token::new(
                            NUMBER,
                            num_chars.iter().collect::<String>(),
                            line,
                        ));
                    }

                    let c = code_chars.next();
                    num_chars.push(c.unwrap());
                };

                result
            }
            c if c.is_ascii_alphabetic() => {
                let mut ident_chars: Vec<char> = Vec::new();
                ident_chars.push(c);

                let result = loop {
                    if code_chars.peek().is_none()
                        || !code_chars.peek().unwrap().is_ascii_alphanumeric()
                    {
                        break Ok(Token::new(
                            IDENTIFIER,
                            ident_chars.iter().collect::<String>(),
                            line,
                        ));
                    }
                    let c = code_chars.next();
                    ident_chars.push(c.unwrap());
                };

                result
            }

            // Error case
            _ => Err(LexerError::BadToken(c)),
        })
    }

    toks.push(Ok(Token::new(EOF, "".to_string(), line)));
    toks
}

pub fn flatten_toks(tok_set: Vec<Result<Token, LexerError>>) -> Vec<Token> {
    tok_set
        .into_iter()
        .map(|tok| match tok {
            Ok(t) => t,
            Err(LexerError::BadToken(t)) => panic!("Bad token: {t}"),
            Err(LexerError::UnterminatedString) => panic!("Unterminated string"),
        })
        .collect()
}
