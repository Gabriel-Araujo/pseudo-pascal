#[cfg(test)]
mod lexical_tests {
    use crate::common::token::TokenType;
    use crate::lexical::Scanner;
    use std::fs::File;
    use std::io::Read;

    fn consume_file(file_path: String) -> std::io::Result<String> {
        let mut file = File::open(file_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }

    #[test]
    fn test_keywords() {
        let input =
            "program var integer real boolean procedure begin end if then else while do not";
        let mut scanner = Scanner::new(input);
        let tokens = scanner.init().unwrap();
        assert!(tokens
            .iter()
            .all(|token| token.is_type_of(TokenType::Keyword)))
    }

    #[test]
    fn test_valid_identifier() {
        let input = "gabriel souza9 _cruz";
        let mut scanner = Scanner::new(input);
        let tokens = scanner.init().unwrap();
        assert!(tokens
            .iter()
            .all(|token| token.is_type_of(TokenType::Identifier)))
    }

    #[test]
    fn test_integers() {
        let input = "0 1 2 3 4 5 6 7 8 9 10 100 1000";
        let mut scanner = Scanner::new(input);
        let tokens = scanner.init().unwrap();
        tokens
            .iter()
            .for_each(|token| assert!(token.is_type_of(TokenType::Integer)))
    }

    #[test]
    fn test_reals() {
        let input = "1. 2.5";
        let mut scanner = Scanner::new(input);
        let tokens = scanner.init().unwrap();
        tokens
            .iter()
            .for_each(|token| assert!(token.is_type_of(TokenType::Real)))
    }

    #[test]
    fn test_delimiters() {
        let input = "; . : ( ) ,";
        let mut scanner = Scanner::new(input);
        let tokens = scanner.init().unwrap();
        assert!(tokens
            .iter()
            .all(|token| token.is_type_of(TokenType::Delimiter)))
    }

    #[test]
    fn test_assignment() {
        let input = ":=";
        let mut scanner = Scanner::new(input);
        let tokens = scanner.init().unwrap();
        assert!(tokens
            .iter()
            .all(|token| token.is_type_of(TokenType::Assignment)))
    }

    #[test]
    fn test_relational_operators() {
        let input = "= < > <= >= <>";
        let mut scanner = Scanner::new(input);
        let tokens = scanner.init().unwrap();
        assert!(tokens
            .iter()
            .all(|token| token.is_type_of(TokenType::RelationalOperators)))
    }

    #[test]
    fn test_additive_operators() {
        let input = "+ - or";
        let mut scanner = Scanner::new(input);
        let tokens = scanner.init().unwrap();
        assert!(tokens
            .iter()
            .all(|token| token.is_type_of(TokenType::AdditiveOperators)))
    }

    #[test]
    fn test_multiplicative_operators() {
        let input = "* / and";
        let mut scanner = Scanner::new(input);
        let tokens = scanner.init().unwrap();
        assert!(tokens
            .iter()
            .all(|token| token.is_type_of(TokenType::MultiplicativeOperators)))
    }

    #[test]
    fn test1() {
        let input = consume_file("tests/Test1.pas".to_string()).unwrap();
        let mut scanner = Scanner::new(&input);

        let failed = match scanner.init() {
            Ok(_) => false,
            Err(_) => true,
        };

        assert!(failed);
    }

    #[test]
    fn test2() {
        let input = consume_file("tests/Test2.pas".to_string()).unwrap();
        let mut scanner = Scanner::new(&input);

        let failed = match scanner.init() {
            Ok(_) => true,
            Err(_) => false,
        };

        assert!(failed);
    }
}
