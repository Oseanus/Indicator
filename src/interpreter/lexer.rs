use super::token::Token;

pub struct Lexer {
    source: String,
}

impl Lexer {
    pub fn new (source: String) -> Self {
        Lexer {
            source: source
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        tokens
    }
}