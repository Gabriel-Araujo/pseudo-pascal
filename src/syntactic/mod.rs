use std::error::Error;
use crate::common::token::{Token, TokenType};
use crate::common::token::TokenType::*;

pub struct Parser {
    tokens_buffer: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: &Vec<Token>) -> Self {
        let mut temp = tokens.to_owned();
        temp.reverse();
        Self {
            tokens_buffer: temp,
        }
    }

    pub fn init(&mut self) -> Result<bool, Box<dyn Error + Send + Sync + 'static>> {
        self.programa()?;
        Ok(true)
    }
}

impl Parser {
    fn programa(&mut self) -> Result<(), String> {
        self.program()?;
        self.vars_declaration()?;
        self.subprograms_declaration()?;
        self.compound_command()?;

        self.consume(Delimiter, ".")?;

        Ok(())
    }
    
    // Aqui começa a produção de program
    fn program(&mut self) -> Result<(), String>{
        self.consume(Keyword, "program")?;
        self.consume_identifiers()?;
        self.consume(Delimiter, ";")?;

        Ok(())
    }
    
    // Aqui começa a produção de vars_declaration
    fn vars_declaration(&mut self) -> Result<(), String> {
        let var = self.peek()?;

        if var.get_lexeme() == "var" {
            self.tokens_buffer.pop();
            self.list_of_vars_declaration()?;
        }
        
        Ok(())
    }

    fn list_of_vars_declaration(&mut self) -> Result<(), String> {
        self.list_of_identifiers()?;

        self.consume(Delimiter, ":")?;
        
        self.types()?;

        self.consume(Delimiter, ";")?;
        
        self.list_of_vars_declaration_prime()?;
        
        Ok(())
    }
    
    fn list_of_vars_declaration_prime(&mut self) -> Result<(), String> {
        let next = self.peek()?;
        if next.is_type_of(Identifier) {
            self.list_of_identifiers()?;

            self.consume(Delimiter, ":")?;
            
            self.types()?;

            self.consume(Delimiter, ";")?;
            
            self.list_of_vars_declaration_prime()?;
        }
        Ok(())
    }
    
    fn list_of_identifiers(&mut self) -> Result<(), String> {
        self.consume_identifiers()?;
        
        self.list_of_identifiers_prime()?;
        Ok(())
    }
    
    fn list_of_identifiers_prime(&mut self) -> Result<(), String> {
        let comma = self.peek()?;
        
        if comma.get_lexeme() == "," { 
            self.tokens_buffer.pop();
            self.consume_identifiers()?;
            
            self.list_of_identifiers_prime()?;
        }
        
        Ok(())
    }
    
    fn types(&mut self) -> Result<(), String> {
        match self.tokens_buffer.pop() {
            None => { return Err("Syntactic Error. Unexpected end of file.".to_string()); }
            Some(value) => {
                if !value.is_type_of(Keyword) {
                    return Err(format!("Expected an keyword: 'integer', 'real' or 'boolean'.\nInstead got '{}' of type '{}' at line {} column {}.",
                    value.get_lexeme(), value.get_type(), value.get_line(), value.get_column()))
                }
                let lexeme = value.get_lexeme();
                if !(lexeme == "integer" || lexeme == "real" || lexeme == "boolean") {
                    return Err(format!("Expected 'integer', 'real' or 'boolean' got '{}' at line {} column {}.",
                    lexeme, value.get_line(), value.get_column()));
                }
            }
        }
        Ok(())
    }
    
    // Aqui começa a produção de subprograms_declaration 
    fn subprograms_declaration(&mut self) -> Result<(), String> {
        let next = self.peek()?;
        
        if next.is_type_of(Keyword) && next.get_lexeme() == "procedure" {
            self.subprogram_declaration()?;

            self.consume(Delimiter, ";")?;
            
            self.subprograms_declaration()?;
        }
        Ok(())
    }
    
    fn subprogram_declaration(&mut self) -> Result<(), String> {
        self.consume(Keyword, "procedure")?;
        
        self.consume_identifiers()?;

        self.arguments()?;

        self.consume(Delimiter, ";")?;
        
        self.vars_declaration()?;
        
        self.subprograms_declaration()?;
        
        self.compound_command()?;
        Ok(())
    }
    
    fn arguments(&mut self) -> Result<(), String> {
        let next = self.peek()?;

        if next.is_type_of(Delimiter) && next.get_lexeme() == "(" {
            self.consume(Delimiter, "(")?;

            self.list_of_parameters()?;

            self.consume(Delimiter, ")")?;
        }
        Ok(())
    }
    
    fn list_of_parameters(&mut self) -> Result<(), String> {
        self.list_of_identifiers()?;

        self.consume(Delimiter, ":")?;
        
        self.types()?;
        
        self.list_of_parameters_prime()?;
        
        Ok(())
    }
    
    fn list_of_parameters_prime(&mut self) -> Result<(), String> {
        let next = self.peek()?;
        if next.is_type_of(Delimiter) && next.get_lexeme() == ";" {
            self.consume(Delimiter, ";")?;
            self.list_of_identifiers()?;

            self.consume(Delimiter, ":")?;

            self.types()?;

            self.list_of_parameters_prime()?;
        }
        else if next.get_lexeme() != ")" {
            return Err(format!("Expected ';'. Instead got {} of type {} at line {} column {}.",
            next.get_lexeme(), next.get_type(), next.get_line(), next.get_column()))
        }
        Ok(())
    }
    
    fn compound_command(&mut self) -> Result<(), String> {
        self.consume(Keyword, "begin")?;

        self.optional_commands()?;

        self.consume(Keyword, "end")?;

        Ok(())
    }

    fn optional_commands(&mut self) -> Result<(), String> {
        let value = self.peek()?;
        let lexeme = value.get_lexeme();
        if (value.is_type_of(Keyword) && (lexeme == "if" || lexeme == "var" || lexeme == "while" || lexeme == "begin")) || value.is_type_of(Identifier) {
            self.list_of_commands()?;
        }
        Ok(())
    }

    fn list_of_commands(&mut self) -> Result<(), String> {
        self.commands()?;
        self.list_of_commands_prime()?;
        Ok(())
    }

    fn list_of_commands_prime(&mut self) -> Result<(), String> {
        let next = self.peek()?;

        if next.get_lexeme() == ";" {
            self.consume(Delimiter, ";")?;
            self.commands()?;
            self.list_of_commands_prime()?;
        }
        Ok(())
    }

    fn commands(&mut self) -> Result<(), String> {
        let next = self.peek()?;

        if next.is_type_of(Identifier) {
            self.command_prime()?;
        }
        else if next.get_lexeme() == "begin" {
            self.compound_command()?;
        }
        else if next.get_lexeme() == "if" {
            self.consume(Keyword, "if")?;
            self.expression()?;
            self.consume(Keyword, "then")?;
            self.commands()?;
            self.else_part()?;
        }
        else if next.get_lexeme() == "while" {
            self.consume(Keyword, "while")?;
            self.expression()?;
            self.consume(Keyword, "do")?;
            self.commands()?;
        }
        Ok(())
    }

    fn else_part(&mut self) -> Result<(), String> {
        let next = self.peek()?;
        if next.get_lexeme() == "else" {
            self.consume(Keyword, "else")?;
            self.commands()?;
        }
        Ok(())
    }

    fn command_prime(&mut self) -> Result<(), String> {
        self.consume_identifiers()?;
        self.command_dual_prime()?;
        Ok(())
    }

    fn command_dual_prime(&mut self) -> Result<(), String> {
        let next = self.peek()?;

        if next.is_type_of(Assignment) {
            self.consume(Assignment, ":=")?;
            self.expression()?;
        }
        else if next.get_lexeme() == "(" {
            self.consume(Delimiter, "(")?;
            self.list_of_expressions()?;
            self.consume(Delimiter, ")")?;
        }
        else if next.get_lexeme() == "=" {
            return Err(format!("Invalid operator. Got '=' at line {} column {}, didn't you mean ':='?"
            , next.get_line(), next.get_column()))
        }
        Ok(())
    }

    fn procedure_activation(&mut self) -> Result<(), String> {
        self.consume_identifiers()?;
        self.procedure_activation_prime()?;
        Ok(())
    }

    fn procedure_activation_prime(&mut self) -> Result<(), String> {
        let next = self.peek()?;
        if next.is_type_of(Delimiter) && next.get_lexeme() == "(" {
            self.consume(Delimiter, "(")?;
            self.list_of_expressions()?;
            self.consume(Delimiter, ")")?;
        }
        Ok(())
    }

    fn list_of_expressions(&mut self) -> Result<(), String> {
        self.expression()?;
        self.list_of_expressions_prime()?;
        Ok(())
    }

    fn list_of_expressions_prime(&mut self) -> Result<(), String> {
        let next = self.peek()?;

        if next.is_type_of(Delimiter) && next.get_lexeme() == "," {
            self.consume(Delimiter, ",")?;
            self.expression()?;
            self.list_of_expressions_prime()?;
        }
        Ok(())
    }

    fn expression(&mut self) -> Result<(), String> {
        self.simple_expression()?;
        self.expression_prime()?;
        Ok(())
    }

    fn expression_prime(&mut self) -> Result<(), String> {
        let next = self.peek()?;

        if next.is_type_of(RelationalOperators) {
            self.relational_op()?;
            self.simple_expression()?;
        }

        Ok(())
    }

    fn simple_expression(&mut self) -> Result<(), String> {
        let next = self.peek()?;

        if next.get_lexeme() == "+" || next.get_lexeme() == "-" {
            self.signal()?;
            self.term()?;
            self.simple_expression_prime()?;
        }
        else {
            self.term()?;
            self.simple_expression_prime()?;
        }
        Ok(())
    }

    fn simple_expression_prime(&mut self) -> Result<(), String> {
        let next = self.peek()?;

        if next.is_type_of(AdditiveOperators) {
            self.additive_op()?;
            self.term()?;
            self.simple_expression_prime()?;
        }
        Ok(())
    }

    fn signal(&mut self) -> Result<(), String> {
        match self.tokens_buffer.pop() {
            None => {return Err("Syntactic error. Unexpected end of file.".to_string())}
            Some(value) => {
                if !(value.get_lexeme() == "+" || value.get_lexeme() == "-") {
                    return Err(format!("Expected a signal '+' or '-'. Instead got {} of type {} at line {} column {}.",
                    value.get_lexeme(), value.get_type(), value.get_line(), value.get_column()));
                }
            }
        }
        Ok(())
    }

    fn term(&mut self) -> Result<(), String> {
        self.factor()?;
        self.term_prime()?;
        Ok(())
    }

    fn term_prime(&mut self) -> Result<(), String> {
        let next = self.peek()?;
        if next.is_type_of(MultiplicativeOperators) {
            self.multiplicative_op()?;
            self.factor()?;
            self.term_prime()?;
        }
        Ok(())
    }

    fn factor(&mut self) -> Result<(), String> {
        let next = self.peek()?;
        
        if next.is_type_of(Identifier) { 
            self.procedure_activation()?;
        }
        else if next.is_type_of(Integer) {
            self.consume_by_type(Integer)?;
        }
        else if next.is_type_of(Real) {
            self.consume_by_type(Real)?;
        }
        else if next.is_type_of(Boolean) {
            self.consume_by_type(Boolean)?;
        }
        else if next.get_lexeme() == "(" {
            self.consume(Delimiter, "(")?;
            self.expression()?;
            self.consume(Delimiter, ")")?;
        }
        else if next.is_type_of(Keyword) && next.get_lexeme() == "not" {
            self.consume(Keyword, "not")?;
            self.factor()?;
        }
        Ok(())
    }

    fn relational_op(&mut self) -> Result<(), String> {
        self.consume_by_type(RelationalOperators)?;
        Ok(())
    }

    fn additive_op(&mut self) -> Result<(), String> {
        self.consume_by_type(AdditiveOperators)?;
        Ok(())
    }

    fn multiplicative_op(&mut self) -> Result<(), String> {
        self.consume_by_type(MultiplicativeOperators)?;
        Ok(())
    }

}

impl Parser {
    fn peek(&self) -> Result<Token, String> {
        match self.tokens_buffer.last() {
            None => { Err("Syntactic error. Unexpected end of file".to_string()) }
            Some(value) => { Ok(value.clone()) }
        }
    }

    // Não funciona com identificadores e números
    fn consume(&mut self, expected_type: TokenType, expected_lexeme: &str) -> Result<(), String> {
        match self.tokens_buffer.pop() {
            None => { return Err("Syntactic error. Unexpected end of file.".to_string()); }
            Some(value) => {
                if !(value.is_type_of(expected_type) && value.get_lexeme() == expected_lexeme) {
                    return Err(format!("Expected {} '{}'. Instead got '{}' of type {} at line {} column {}",
                                       expected_type, expected_lexeme, value.get_lexeme(), value.get_type(), value.get_line(), value.get_column()));
                }
            }
        }
        Ok(())
    }

    fn consume_identifiers(&mut self) -> Result<(), String> {
        match self.tokens_buffer.pop() {
            None => { return Err("Syntactic error. Unexpected end of file.".to_string()); }
            Some(value) => {
                if !value.is_type_of(Identifier) {
                    return Err(format!("Expected an identifier. Instead got '{}' of type {} at line {} column {}",
                        value.get_lexeme(), value.get_type(), value.get_line(), value.get_column()));
                }
            }
        }
        Ok(())
    }

    fn consume_by_type(&mut self, expected_type: TokenType) -> Result<(), String> {
        match self.tokens_buffer.pop() {
            None => { return Err("Syntactic error. Unexpected end of file.".to_string()); }
            Some(value) => {
                if !value.is_type_of(expected_type) {
                    return Err(format!("Expected {}. Instead got '{}' of type {} at line {} column {}",
                                       expected_type, value.get_lexeme(), value.get_type(), value.get_line(), value.get_column()));
                }
            }
        }
        Ok(())
    }
}