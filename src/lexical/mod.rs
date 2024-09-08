pub mod error;
mod test;

use crate::common::token::{Token, TokenType};
use crate::common::utils::is_keyword;
use crate::lexical::error::{InvalidCharError, InvalidStateError};
use std::error::Error;
use std::iter::Peekable;
use std::str::Chars;

pub struct Scanner<'s> {
    input: Peekable<Chars<'s>>,
    tokens: Vec<Token>,
    current_state: usize,
    identifier_buffer: String,
    line: usize,
    column: usize,
}

impl<'s> Scanner<'s> {
    pub fn new(input: &'s str) -> Self {
        Self {
            input: input.chars().peekable(),
            tokens: Vec::new(),
            current_state: 0,
            identifier_buffer: String::new(),
            line: 1,
            column: 1,
        }
    }

    pub fn init(&mut self) -> Result<Vec<Token>, Box<dyn Error + Send + Sync + 'static>> {
        // println!("{:?}", self.input);
        loop {
            self.transition()?;
            if self.input.peek().is_none() {
                break;
            }
        }

        Ok(self.tokens.clone())
    }
}

impl<'s> Scanner<'s> {
    fn transition(&mut self) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        let current = match self.input.next() {
            None => {
                return Err(Box::from("EOF"));
            }
            Some(value) => value,
        };
        let next = match self.input.peek() {
            None => ' ',
            Some(value) => *value,
        };
        match self.current_state {
            0 => match current {
                c if c == '\n' => {
                    self.line += 1;
                    self.column = 0;
                }
                c if c.is_whitespace() => {}
                c if c.is_alphabetic() || c == '_' => {
                    self.identifier_buffer.push(c);
                    if !(next.is_alphanumeric() || next == '_') {
                        self.tokens.push(Token::new(
                            &self.identifier_buffer,
                            TokenType::Identifier,
                            self.line,
                            self.column - self.identifier_buffer.len() + 1,
                        ));
                        self.identifier_buffer = String::new();
                        self.current_state = 0;
                    } else {
                        self.current_state = 1;
                    }
                }
                c if c.is_ascii_digit() => {
                    self.identifier_buffer.push(current);
                    if next == '.' {
                        self.current_state = 3;
                    } else if !next.is_ascii_digit() {
                        self.tokens.push(Token::new(
                            &self.identifier_buffer,
                            TokenType::Integer,
                            self.line,
                            self.column - self.identifier_buffer.len() + 1,
                        ));
                        self.identifier_buffer = String::new();
                        self.current_state = 0;
                    } else {
                        self.current_state = 2;
                    }
                }
                c if c == '+' => {
                    self.tokens.push(Token::new(
                        "+",
                        TokenType::AdditiveOperators,
                        self.line,
                        self.column,
                    ));
                    self.current_state = 0;
                }
                c if c == '-' => {
                    self.tokens.push(Token::new(
                        "-",
                        TokenType::AdditiveOperators,
                        self.line,
                        self.column,
                    ));
                    self.current_state = 0;
                }
                c if c == '*' => {
                    self.tokens.push(Token::new(
                        "*",
                        TokenType::MultiplicativeOperators,
                        self.line,
                        self.column,
                    ));
                    self.current_state = 0;
                }
                c if c == '/' => {
                    self.tokens.push(Token::new(
                        "/",
                        TokenType::MultiplicativeOperators,
                        self.line,
                        self.column,
                    ));
                    self.current_state = 0;
                }
                c if c == '=' => {
                    self.tokens.push(Token::new(
                        "=",
                        TokenType::RelationalOperators,
                        self.line,
                        self.column,
                    ));
                    self.current_state = 0;
                }
                c if c == '>' => {
                    if next == '=' {
                        self.current_state = 9;
                    } else {
                        self.tokens.push(Token::new(
                            ">",
                            TokenType::RelationalOperators,
                            self.line,
                            self.column - 1,
                        ));
                        self.current_state = 0;
                    }
                }
                c if c == '<' => {
                    if next == '=' {
                        self.current_state = 11;
                    } else if next == '>' {
                        self.current_state = 12;
                    } else {
                        self.tokens.push(Token::new(
                            "<",
                            TokenType::RelationalOperators,
                            self.line,
                            self.column - 1,
                        ));
                        self.current_state = 0;
                    }
                }
                c if c == ':' => {
                    if next == '=' {
                        self.current_state = 14;
                    } else {
                        self.tokens.push(Token::new(
                            ":",
                            TokenType::Delimiter,
                            self.line,
                            self.column,
                        ));
                        self.current_state = 0;
                    }
                }
                c if c == ',' => {
                    self.tokens.push(Token::new(
                        ",",
                        TokenType::Delimiter,
                        self.line,
                        self.column,
                    ));
                    self.current_state = 0;
                }
                c if c == '.' => {
                    self.tokens.push(Token::new(
                        ".",
                        TokenType::Delimiter,
                        self.line,
                        self.column,
                    ));
                    self.current_state = 0;
                }
                c if c == ';' => {
                    self.tokens.push(Token::new(
                        ";",
                        TokenType::Delimiter,
                        self.line,
                        self.column,
                    ));
                    self.current_state = 0;
                }
                c if c == '(' => {
                    self.tokens.push(Token::new(
                        "(",
                        TokenType::Delimiter,
                        self.line,
                        self.column,
                    ));
                    self.current_state = 0;
                }
                c if c == ')' => {
                    self.tokens.push(Token::new(
                        ")",
                        TokenType::Delimiter,
                        self.line,
                        self.column,
                    ));
                    self.current_state = 0;
                }
                c if c == '{' => {
                    self.current_state = 15;
                }
                t => return Err(Box::from(InvalidCharError::new(t, self.line, self.column))),
            },
            1 => {
                self.identifier_buffer.push(current);
                if !(current.is_alphanumeric() || current == '_')
                    || !(next.is_alphanumeric() || next == '_')
                {
                    if is_keyword(&self.identifier_buffer) {
                        self.tokens.push(Token::new(
                            &self.identifier_buffer,
                            TokenType::Keyword,
                            self.line,
                            self.column - self.identifier_buffer.len() + 1,
                        ))
                    } else if self.identifier_buffer == "and" {
                        self.tokens.push(Token::new(
                            &self.identifier_buffer,
                            TokenType::MultiplicativeOperators,
                            self.line,
                            self.column - self.identifier_buffer.len() + 1,
                        ))
                    } else if self.identifier_buffer == "or" {
                        self.tokens.push(Token::new(
                            &self.identifier_buffer,
                            TokenType::AdditiveOperators,
                            self.line,
                            self.column - self.identifier_buffer.len() + 1,
                        ))
                    } else if self.identifier_buffer == "true" || self.identifier_buffer == "false"
                    {
                        self.tokens.push(Token::new(
                            &self.identifier_buffer,
                            TokenType::Boolean,
                            self.line,
                            self.column - self.identifier_buffer.len() + 1,
                        ))
                    } else {
                        self.tokens.push(Token::new(
                            &self.identifier_buffer,
                            TokenType::Identifier,
                            self.line,
                            self.column - self.identifier_buffer.len() + 1,
                        ))
                    };
                    self.identifier_buffer = String::new();
                    self.current_state = 0;
                }
            }
            2 => {
                self.identifier_buffer.push(current);
                if current == '.' || next == '.' {
                    self.current_state = 3;
                } else if !next.is_ascii_digit() {
                    self.tokens.push(Token::new(
                        &self.identifier_buffer,
                        TokenType::Integer,
                        self.line,
                        self.column - self.identifier_buffer.len() + 1,
                    ));
                    self.identifier_buffer = String::new();
                    self.current_state = 0;
                }
            }
            3 => {
                self.identifier_buffer.push(current);
                if next.is_ascii_digit() {
                    self.current_state = 4;
                } else {
                    self.tokens.push(Token::new(
                        &self.identifier_buffer,
                        TokenType::Real,
                        self.line,
                        self.column + 1 - self.identifier_buffer.len(),
                    ));
                    self.identifier_buffer = String::new();
                    self.current_state = 0;
                }
            }
            4 => {
                self.identifier_buffer.push(current);
                if !next.is_ascii_digit() {
                    self.tokens.push(Token::new(
                        &self.identifier_buffer,
                        TokenType::Real,
                        self.line,
                        self.column + 1 - self.identifier_buffer.len(),
                    ));
                    self.identifier_buffer = String::new();
                    self.current_state = 0;
                }
            }
            9 => {
                if current == '=' {
                    self.tokens.push(Token::new(
                        ">=",
                        TokenType::RelationalOperators,
                        self.line,
                        self.column - 1,
                    ));
                    self.current_state = 0;
                }
            }
            11 => {
                if current == '=' {
                    self.tokens.push(Token::new(
                        "<=",
                        TokenType::RelationalOperators,
                        self.line,
                        self.column - 1,
                    ));
                    self.current_state = 0;
                }
            }
            12 => {
                if current == '>' {
                    self.tokens.push(Token::new(
                        "<>",
                        TokenType::RelationalOperators,
                        self.line,
                        self.column - 1,
                    ));
                    self.current_state = 0;
                }
            }
            14 => {
                self.tokens.push(Token::new(
                    ":=",
                    TokenType::Assignment,
                    self.line,
                    self.column - 1,
                ));
                self.current_state = 0;
            }
            15 => {
                if current == '}' {
                    self.current_state = 0;
                } else if current == '\n' {
                    return Err(Box::from(format!("Unclosed comment at line {}", self.line)));
                }
            }
            _ => {
                return Err(Box::from(InvalidStateError::new(
                    "Reached an invalid end state on lexial analysis.",
                    self.line,
                    self.column,
                )))
            }
        }
        self.column += 1;

        Ok(())
    }
}
