use std::error::Error;
use crate::common::symbol::Symbol;
use crate::common::symbol::Symbol::{EOS};
use crate::common::token::{Token, TokenType};
use crate::common::token::TokenType::*;

pub struct Parser {
    tokens_buffer: Vec<Token>,
    symbol_table: Vec<Symbol>,
    control_type_stack: Vec<String>,
    program_name: String,
    amount: usize,
}

impl Parser {
    pub fn new(tokens: &Vec<Token>) -> Self {
        let mut temp = tokens.to_owned();
        temp.reverse();
        Self {
            tokens_buffer: temp,
            symbol_table: vec![],
            control_type_stack: vec![],
            program_name: "".to_string(),
            amount: 0,
        }
    }

    pub fn init(&mut self) -> Result<bool, Box<dyn Error + Send + Sync + 'static>> {
        self.programa()?;

        
        for item in self.symbol_table.iter().rev() {
            println!("{item}")
        }

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
        self.remove_scope();

        Ok(())
    }
    
    // Aqui começa a produção de program
    fn program(&mut self) -> Result<(), String>{
        self.consume(Keyword, "program")?;
        
        self.symbol_table.push(EOS); // Criação do escopo global
        let token = self.consume_identifiers()?;
        self.program_name = token.get_lexeme().to_string();
        self.add_symbol(Symbol::new(token))?;
        
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
        
        let symbol_type = self.types()?;
        self.update_symbols_type(&symbol_type);

        self.consume(Delimiter, ";")?;
        
        self.list_of_vars_declaration_prime()?;
        
        Ok(())
    }
    
    fn list_of_vars_declaration_prime(&mut self) -> Result<(), String> {
        let next = self.peek()?;
        if next.is_type_of(Identifier) {
            self.list_of_identifiers()?;

            self.consume(Delimiter, ":")?;
            
            let symbol_type = self.types()?;
            self.update_symbols_type(&symbol_type);

            self.consume(Delimiter, ";")?;
            
            self.list_of_vars_declaration_prime()?;
        }
        Ok(())
    }
    
    fn list_of_identifiers(&mut self) -> Result<(), String> {
        let token = self.consume_identifiers()?;
        
        self.amount += 1;
        self.add_symbol(Symbol::new(token))?; // Adiciona o identificador à tabela de símbolos

        self.list_of_identifiers_prime()?;
        
        Ok(())
    }
    
    fn list_of_identifiers_prime(&mut self) -> Result<(), String> {
        let comma = self.peek()?;
        
        if comma.get_lexeme() == "," { 
            self.tokens_buffer.pop();
            let token = self.consume_identifiers()?;
            self.amount += 1;
            self.add_symbol(Symbol::new(token))?; // Adiciona o identificador à tabela de símbolos

            self.list_of_identifiers_prime()?;
        }
        
        Ok(())
    }
    
    fn types(&mut self) -> Result<String, String> {
        match self.tokens_buffer.pop() {
            None => { Err("Syntactic Error. Unexpected end of file.".to_string()) }
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
                Ok(lexeme.to_owned())
            }
        }
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
        
        let token = self.consume_identifiers()?;
        self.add_symbol(Symbol::new(token))?;
        self.symbol_table.push(EOS);
        self.arguments()?;

        self.consume(Delimiter, ";")?;
        
        self.vars_declaration()?;
        
        self.subprograms_declaration()?;
        
        self.compound_command()?;
        self.remove_scope();
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
        let token = self.consume_identifiers()?;
        let temp = self.find_symbol(&Symbol::new(token))?;
        
        self.control_type_stack.push(temp.get_type().unwrap());
        self.command_dual_prime()?;
        Ok(())
    }

    fn command_dual_prime(&mut self) -> Result<(), String> {
        let next = self.peek()?;

        if next.is_type_of(Assignment) {
            self.consume(Assignment, ":=")?;
            self.expression()?;
            self.check_atribuation(next.get_line(), next.get_column())?;
        }
        else if next.get_lexeme() == "(" {
            self.control_type_stack.pop(); // Pra caso não seja um assignment
            self.consume(Delimiter, "(")?;
            self.list_of_expressions()?;
            self.consume(Delimiter, ")")?;
        }
        else if next.get_lexeme() == "=" {
            return Err(format!("Invalid operator. Got '=' at line {} column {}, didn't you mean ':='?"
            , next.get_line(), next.get_column()))
        }
        // self.control_type_stack.pop(); // Pra caso não seja um assignment
        Ok(())
    }

    fn procedure_activation(&mut self) -> Result<(), String> {
        let token = self.consume_identifiers()?;
        self.find_symbol(&Symbol::new(token))?;
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
            self.check_relational(next.get_line(), next.get_column())?; // TODO
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
            self.check_arithmetics(next.get_line(), next.get_column())?;
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
            let operator = self.multiplicative_op()?;
            self.factor()?;
            self.term_prime()?;
            if operator.get_lexeme() == "and" { 
                self.check_logic(next.get_line(), next.get_column())?; 
            }
            else { self.check_arithmetics(next.get_line(), next.get_column())?; }
        }
        Ok(())
    }

    fn factor(&mut self) -> Result<(), String> {
        let next = self.peek()?;
        
        if next.is_type_of(Identifier) { 
            self.procedure_activation()?;
            let temp = self.find_symbol(&Symbol::new(next.to_owned()))?;
            let symbol_type = temp.get_type().unwrap();
            self.control_type_stack.push(symbol_type);
        }
        else if next.is_type_of(Integer) {
            self.consume_by_type(Integer)?;
            self.control_type_stack.push("integer".to_string());
        }
        else if next.is_type_of(Real) {
            self.consume_by_type(Real)?;
            self.control_type_stack.push("real".to_string());
        }
        else if next.is_type_of(Boolean) {
            self.consume_by_type(Boolean)?;
            self.control_type_stack.push("boolean".to_string());
        }
        else if next.get_lexeme() == "(" {
            self.consume(Delimiter, "(")?;
            self.expression()?;
            self.consume(Delimiter, ")")?;
        }
        else if next.is_type_of(Keyword) && next.get_lexeme() == "not" {
            self.consume(Keyword, "not")?;
            self.control_type_stack.push("boolean".to_string());
            self.factor()?;
            self.check_logic(next.get_line(), next.get_column())?;
        }
        Ok(())
    }

    fn relational_op(&mut self) -> Result<(), String> {
        self.consume_by_type(RelationalOperators)?;
        Ok(())
    }

    fn additive_op(&mut self) -> Result<Token, String> {
        self.consume_by_type(AdditiveOperators)
    }

    fn multiplicative_op(&mut self) -> Result<Token, String> {
        self.consume_by_type(MultiplicativeOperators)
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
    fn consume(&mut self, expected_type: TokenType, expected_lexeme: &str) -> Result<Token, String> {
        match self.tokens_buffer.pop() {
            None => { Err("Syntactic error. Unexpected end of file.".to_string()) }
            Some(value) => {
                if !(value.is_type_of(expected_type) && value.get_lexeme() == expected_lexeme) {
                    return Err(format!("Expected {} '{}'. Instead got '{}' of type {} at line {} column {}",
                                       expected_type, expected_lexeme, value.get_lexeme(), value.get_type(), value.get_line(), value.get_column()));
                }
                Ok(value)
            }
        }
    }

    fn consume_identifiers(&mut self) -> Result<Token, String> {
        match self.tokens_buffer.pop() {
            None => { Err("Syntactic error. Unexpected end of file.".to_string()) }
            Some(value) => {
                if !value.is_type_of(Identifier) {
                    return Err(format!("Expected an identifier. Instead got '{}' of type {} at line {} column {}",
                        value.get_lexeme(), value.get_type(), value.get_line(), value.get_column()));
                }
                Ok(value)
            }
        }
    }

    fn consume_by_type(&mut self, expected_type: TokenType) -> Result<Token, String> {
        match self.tokens_buffer.pop() {
            None => { Err("Syntactic error. Unexpected end of file.".to_string()) }
            Some(value) => {
                if !value.is_type_of(expected_type) {
                    return Err(format!("Expected {}. Instead got '{}' of type {} at line {} column {}",
                                       expected_type, value.get_lexeme(), value.get_type(), value.get_line(), value.get_column()));
                }
                Ok(value)
            }
        }
    }

    fn add_symbol(&mut self, symbol: Symbol) -> Result<(), String> {
        if symbol == EOS { panic!("Wrong use of End of Scope"); }
        let mut buffer = self.symbol_table.iter().rev();
        let mut buffer_symbol = buffer.next().unwrap();

        let a = buffer_symbol != &EOS;
        while buffer_symbol != &EOS {
            if buffer_symbol == &symbol {
                return Err(format!("Identifier '{}' already declared in line {} column {}.",
                    buffer_symbol.as_token().unwrap().get_lexeme(), buffer_symbol.as_token().unwrap().get_line(), buffer_symbol.as_token().unwrap().get_column()));
            }
            buffer_symbol = buffer.next().unwrap();
        }

        self.symbol_table.push(symbol);
        Ok(())
    }
    
    fn update_symbols_type(&mut self, symbol_type: &str) {
        let mut buffer = self.symbol_table.iter_mut().rev();
        let mut buffer_item = buffer.next().expect("Something went wrong while updating symbols table type.");
        while self.amount > 0 {
            buffer_item.change_type(symbol_type);
            buffer_item = buffer.next().expect("Something went wrong while updating symbols table type.");
            self.amount -= 1;
        }
    }
    
    fn find_symbol(&self, symbol: &Symbol) -> Result<Symbol, String> {
        if symbol == &EOS { panic!("Wrong use of End of Scope"); }
        let temp = symbol.as_token().unwrap();
        
        if temp.get_lexeme() == self.program_name {
            return Err(format!("Use of the program name at line {} column {}.", temp.get_line(), temp.get_column()))
        }
        
        match self.symbol_table.iter().rfind(|item| item == &symbol) {
            None => {
                Err(format!("Use of the undeclared identifier '{}' at line {} column {}.",
                            temp.get_lexeme(), temp.get_line(), temp.get_column()))
            }
            Some(t) => { 
                Ok(t.to_owned()) 
            }
        }
        
    }

    fn remove_scope(&mut self) {
        loop {
            if self.symbol_table.pop() == Some(EOS) {break;}
        }
    }

    fn check_arithmetics(&mut self, line: usize, column: usize) -> Result<(), String>{
        let first = match self.control_type_stack.pop() {
            None => return Err(format!("Unable to check type at line {} column {}", line, column)),
            Some(t) => t,
        };
        let second = match self.control_type_stack.pop() {
            None => return Err(format!("Unable to check type at line {} column {}", line, column)),
            Some(t) => t,
        };

        if (first == "integer" || first == "real")&& (first == second) { self.control_type_stack.push(first); }
        else if first == "integer" && second == "real" { self.control_type_stack.push(second); }
        else if first == "real" && second == "integer" {self.control_type_stack.push(first); }
        else {
            return Err(format!("Invalid type between operands in arithmetic operation at line {} column {}.\n\
                                Cannot execute arithmetic operations between '{}' and '{}'.",
                               line, column, second, first))
        };

        Ok(())
    }
    
    fn check_relational(&mut self, line: usize, column: usize) -> Result<(), String> {
        let first = match self.control_type_stack.pop() {
            None => return Err(format!("Relational check Failed.\nUnable to check type at line {} column {}", line, column)),
            Some(t) => t,
        };
        let second = match self.control_type_stack.pop() {
            None => return Err(format!("Relational check Failed.\nUnable to check type at line {} column {}", line, column)),
            Some(t) => t,
        };
        
        if (first == "integer" || first == "real") && (second == "integer" || second == "real") { 
            self.control_type_stack.push("boolean".to_string());
            return Ok(());
        }

        Err(format!("Invalid type between operands in relational operation at line {} column {}.\n\
                    Cannot execute relational operations between '{}' and '{}'.",
                    line, column, second, first))
    }
    
    fn check_atribuation(&mut self, line: usize, column: usize) -> Result<(), String> {
        let first = match self.control_type_stack.pop() {
            None => return Err(format!("Assignment check Failed.\nUnable to check type at line {} column {}", line, column)),
            Some(t) => t,
        };
        let second = match self.control_type_stack.pop() {
            None => return Err(format!("Assignment check Failed.\nUnable to check type at line {} column {}", line, column)),
            Some(t) => t,
        };
        if first != second { 
            return Err(format!("Invalid assignment at line {} column {}.\n\
                                Cannot assign value of type '{}' to an variable of type '{}'.",
            line, column, first, second))
        }
        Ok(())
    }
    
    fn check_logic(&mut self, line: usize, column: usize) -> Result<(), String> {
        let first = match self.control_type_stack.pop() {
            None => return Err(format!("Logic check Failed.\nUnable to check type at line {} column {}", line, column)),
            Some(t) => t,
        };
        let second = match self.control_type_stack.pop() {
            None => return Err(format!("Logic check Failed.\nUnable to check type at line {} column {}", line, column)),
            Some(t) => t,
        };
        
        if first != "boolean" || second != "boolean" {
            return Err(format!("Invalid type between operands in logic operation at line {} column {}.\n\
                    Cannot execute logic operations between '{}' and '{}'.",
                        line, column, second, first));
        }
        
        self.control_type_stack.push(first);
        Ok(())
    }
}