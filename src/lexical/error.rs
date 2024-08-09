use std::fmt;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct InvalidCharError {
    pub character: char,
    pub line: usize,
    pub column: usize,
}

impl InvalidCharError  {
    pub fn new(character: char, line: usize, column: usize) -> Self {
        Self {character, line, column}
    }
}

impl fmt::Display for InvalidCharError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result <(), fmt::Error> {
        write!(f, "The invalid character '{}' was fount at line {}, column {}",
               self.character, self.line, self.column)
    }
}

impl Error for InvalidCharError {}


#[derive(Debug, Clone)]
pub struct InvalidStateError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}


impl InvalidStateError  {
    pub fn new(message: &str, line: usize, column: usize) -> Self {
        Self {message: message.to_string(), line, column}
    }
}

impl fmt::Display for InvalidStateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result <(), fmt::Error> {
        write!(f, "An error occurred at line {}, column {}.\n{}",
               self.line, self.column, self.message)
    }
}

impl Error for InvalidStateError {}