const KEYWORDS: [&str; 14] = ["program", "var", "integer", "real", "boolean", "procedure", "begin", "end", "if", "then", "else", "while", "do", "not"];
const CHECK_TYPES: [&str; 3] = ["integer", "real", "boolean"];
pub fn is_keyword(input: &str) -> bool {
    KEYWORDS.contains(&input)
}