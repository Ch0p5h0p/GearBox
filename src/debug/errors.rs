pub enum LexerError {
    UnterminatedString,
    BadToken(char),
}
