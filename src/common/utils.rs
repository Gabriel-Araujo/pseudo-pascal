const DELIMITERS: [char; 6] = [';', '.', ':', '(', ')', ','];
const KEYWORDS: [&str; 14] = ["program", "var", "integer", "real", "boolean", "procedure", "begin", "end", "if", "then", "else", "while", "do", "not"];

pub fn is_delimiter(input: char) -> bool {
    DELIMITERS.contains(&input)
}

pub fn is_keyword(input: &str) -> bool {
    KEYWORDS.contains(&input)
}