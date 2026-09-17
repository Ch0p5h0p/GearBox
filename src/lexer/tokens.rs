#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Yes, I'm aware these aren't idiomatic. Unfortunately, I'm stupid :)

    // Single-char tokens
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    LEFT_BRACKET,
    RIGHT_BRACKET,
    COMMA,
    DOT,
    DASH,
    PLUS,
    SEMICOLON,
    COLON,
    SLASH,
    STAR,

    // One or two char tokens
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,
    AMPERSAND,
    AMP_EQUAL,
    DOUBLE_AMP,
    BAR,
    DOUBLE_BAR,
    BAR_EQUAL,
    CARET,
    CARET_EQUAL,
    AT,
    DOUBLE_AT,

    // Literals
    IDENTIFIER,
    STRING,
    NUMBER,

    // Special
    EOF,
    COMMENT,
    WHITESPACE,
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
