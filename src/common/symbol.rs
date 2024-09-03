use std::fmt;
use std::fmt::Formatter;
use crate::common::token::Token;
use crate::common::token::TokenType::Identifier;

#[derive(PartialEq, Debug)]
pub enum Symbol {
    Token(Token),
    ProgramIdentifier(Token),
    EOS // End of Scope
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}",
        match self {
            Symbol::Token(t) => t.get_lexeme(),
            Symbol::ProgramIdentifier(t) => t.get_lexeme(),
            Symbol::EOS => "EOS"
        })
    }
}

impl Symbol {
    pub fn as_token(&self) -> Option<Token> {
        match self {
            Symbol::Token(t) => Some(t.to_owned()),
            Symbol::ProgramIdentifier(t) => Some(t.to_owned()),
            Symbol::EOS => None
        }
    }
    pub fn is_eos(&self) -> bool {
        matches!(self, Symbol::EOS)
    }

    pub fn is_identifier(&self) -> bool {
        match self {
            Symbol::Token(t) => t.is_type_of(Identifier),
            Symbol::ProgramIdentifier(_) => false,
            Symbol::EOS => false
        }
    }

    pub fn is_program_identifier(&self) -> bool {
        match self {
            Symbol::Token(t) => false,
            Symbol::ProgramIdentifier(_) => true,
            Symbol::EOS => false
        }
    }

    /*
    pub fn is_integer(&self) -> bool {
        matches!(self, Symbol::Integer(_))
    }

    pub fn is_real(&self) -> bool {
        matches!(self, Symbol::Real(_))
    }
     */
}


