#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Single-char tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    Dash,
    Plus,
    Semicolon,
    Colon,
    Slash,
    Star,
    Dollar,

    // One or two char tokens
    Bang,
    BangEqual,
    Equal,
    DoubleEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Ampersand,
    AmpEqual,
    DoubleAmp,
    Bar,
    DoubleBar,
    BarEqual,
    Caret,
    CaretEqual,
    At,
    DoubleAt,

    // Literals
    IdentifierLit,
    StringLit,
    NumberLit,

    // Special
    EOF,
}

pub struct Token {
    tok_type: TokenType,
    lexeme: String,
    line: i32,
}

impl Token {
    pub fn new(tok_type: TokenType, lexeme: String, line: i32) -> Token {
        Token {
            tok_type: tok_type,
            lexeme: lexeme,
            line: line,
        }
    }

    pub fn to_string(&self) -> String {
        let tok_type = self.tok_type.clone();
        let lexeme = self.lexeme.clone();
        let line = self.line;
        format!("{lexeme}\t{tok_type:?}, line {line}")
    }

    pub fn get_type(&self) -> String {
        let tok_type = &self.tok_type;
        format!("{tok_type:?}")
    }

    pub fn get_lexeme(&self) -> String {
        self.lexeme.clone()
    }
}

pub fn print_token_table(tokens: Vec<Token>) {
    for (i, token) in tokens.iter().enumerate() {
        print!(
            "{:08}  {:<16} {:<20}",
            i,
            token.get_type(),
            format!("{:?}", token.get_lexeme())
        );

        if (i + 1) % 3 == 0 {
            println!();
        }
    }

    if tokens.len() % 3 != 0 {
        println!();
    }
}
