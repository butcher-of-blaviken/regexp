mod lexer;
mod parser;

use crate::lexer::Token;

pub struct Regex {
    tokens: Vec<Token>,
}

impl Regex {
    pub fn new(expression: &str) -> Result<Regex, String> {
        // lex the expression
        let tokens = lexer::lex(expression)?;
        // Here you would typically compile the regex expression
        // For now, we just return an empty Regex instance
        Ok(Regex { tokens })
    }

    pub fn is_match(&self, _text: &str) -> bool {
        // Here you would typically check if the text matches the regex
        // For now, we just return false
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regex_new() {
        let regex = Regex::new("a*b").unwrap();
        assert!(regex.is_match("aaab"));
        assert!(!regex.is_match("b"));
    }
}
