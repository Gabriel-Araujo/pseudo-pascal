use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenType {
    Keyword, // program, var, integer, real, boolean, procedure, begin, end, if, then, else, while, do, not
    Identifier, // [a-z|A-Z]+[0-9]*[_]*
    Integer, // [0-9]+
    Real, // [0-9]+.[0-9]*
    Delimiter, // ; . : ( ) ,
    RelationalOperators, // = < > <= >= <>
    Assignment, // :=
    AdditiveOperators, // + - or
    MultiplicativeOperators, // * / and
    Invalid,
}

impl Default for TokenType {
    fn default() -> Self {
        TokenType::Invalid
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TokenType::Keyword => "Keyword",
                TokenType::Identifier => "Identifier",
                TokenType::Integer => "Integer",
                TokenType::Real => "Real",
                TokenType::Delimiter => "Delimiter",
                TokenType::RelationalOperators => "Relational Operators",
                TokenType::Assignment => "Assignment",
                TokenType::AdditiveOperators => "Additive Operators",
                TokenType::MultiplicativeOperators => "Multiplicative Operators",
                TokenType::Invalid => "Invalid",
            }
        )
    }
}

#[derive(Debug, Clone, Default)]
pub struct Token {
    lexeme: String,
    category: TokenType,
    line: usize,
    column: usize,
}

impl Token {
    pub fn new(lexeme: &str, category: TokenType, line: usize, column: usize) -> Self {
        Token {lexeme: lexeme.trim().to_string(), category, line, column}
    }
    
    pub fn is_type_of(&self, token_type: TokenType) -> bool {
        self.category == token_type
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.lexeme == other.lexeme && self.category == other.category
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "['{}', {}, line: {}, column: {}]", self.lexeme, self.category, self.line, self.column)
    }
}
