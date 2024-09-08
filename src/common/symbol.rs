use crate::common::token::Token;
use crate::common::token::TokenType::Identifier;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone)]
pub enum Symbol {
    Identifier(SymbolIdentifier),
    EOS, // End of Scope
}

#[derive(PartialEq, Debug, Clone)]
pub struct SymbolIdentifier {
    pub token: Token,
    pub identifier_type: String,
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Symbol::Identifier(t) => t.token.get_lexeme(),
                Symbol::EOS => "EOS",
            }
        )
    }
}

impl PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        if self.is_eos() && other.is_eos() {
            true
        } else if self.is_identifier() && other.is_identifier() {
            self.as_token().unwrap().get_lexeme() == other.as_token().unwrap().get_lexeme()
        } else {
            false
        }
    }
}

impl Symbol {
    pub fn new(token: Token) -> Self {
        Symbol::Identifier(SymbolIdentifier {
            token,
            identifier_type: "".to_string(),
        })
    }

    pub fn as_token(&self) -> Option<Token> {
        match self {
            Symbol::Identifier(t) => Some(t.token.to_owned()),
            Symbol::EOS => None,
        }
    }

    pub fn change_type(&mut self, new_type: &str) {
        match self {
            Symbol::Identifier(t) => {
                t.identifier_type = new_type.to_string();
            }
            Symbol::EOS => {}
        }
    }

    pub fn is_eos(&self) -> bool {
        matches!(self, Symbol::EOS)
    }

    pub fn is_identifier(&self) -> bool {
        match self {
            Symbol::Identifier(t) => t.token.is_type_of(Identifier),
            Symbol::EOS => false,
        }
    }

    pub fn get_type(&self) -> Option<String> {
        match self {
            Symbol::Identifier(t) => Some(t.identifier_type.to_owned()),
            Symbol::EOS => None,
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
