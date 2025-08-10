#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Program is a special token that represents the whole regex 'program'
    Program,
    Literal(String),
    // Backslash is used to escape characters in regex
    // For example, \d for digit, \w for word character, etc.
    Backslash(char),
    KleeneStar,
    KleenePlus,
    Question,
    LParen,
    RParen,
    VBar,
    RBracket,
    LBracket,
    LBrace,
    RBrace,
    Comma,

    // Dash is used in character classes, e.g., [a-z]
    // or sometimes as a literal character.
    // If it appears at the start or end of a character class, it is treated as a literal dash.
    // For example, [a-z-] matches any lowercase letter or a dash.
    // If it appears between two characters, it is treated as a range.
    // For example, [a-z] matches any lowercase letter from "a" to "z".
    // If it appears at the start of a character class, it is treated as a literal dash.
    // For example, [-abc] matches a dash, "a", "b", or "c".
    Dash,
    Dot,
    Caret,
    Dollar,
}

impl Token {
    pub fn is_binary_operator(&self) -> bool {
        matches!(self, Token::VBar | Token::Comma | Token::Dash)
    }

    pub fn is_unary_postfix(&self) -> bool {
        matches!(self, Token::KleeneStar | Token::KleenePlus | Token::Question)
    }

    pub fn is_unary_prefix(&self) -> bool {
        matches!(self, Token::Caret)
    }
}

/// lex transforms the given raw regex expression into a vector of tokens
/// that is amenable to parsing.
pub fn lex(expression: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();

    // need to be able to peek at the next character without consuming it.
    let mut iter = expression.chars().peekable();
    loop {
        let c = match iter.next() {
            Some(c) => c,
            None => break,
        };
        match c {
            '*' => tokens.push(Token::KleeneStar),
            '+' => tokens.push(Token::KleenePlus),
            '?' => tokens.push(Token::Question),
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            '|' => tokens.push(Token::VBar),
            '[' => tokens.push(Token::LBracket),
            ']' => tokens.push(Token::RBracket),
            '{' => tokens.push(Token::LBrace),
            '}' => tokens.push(Token::RBrace),
            ',' => tokens.push(Token::Comma),
            '.' => tokens.push(Token::Dot),
            '^' => tokens.push(Token::Caret),
            '$' => tokens.push(Token::Dollar),
            '-' => tokens.push(Token::Dash),
            '\\' => {
                match iter.next() {
                    Some(next) => tokens.push(Token::Backslash(next)),
                    None => return Err("Invalid escape sequence".into()),
                }
            },
            _ if c.is_alphanumeric() || c.is_whitespace() => {
                let mut literal = String::new();
                literal.push(c);
                while let Some(next) = iter.peek() {
                    if next.is_alphanumeric() || next.is_whitespace() {
                        literal.push(iter.next().unwrap());
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Literal(literal));
            }
            _ => {
                return Err(format!("Unexpected character: '{}'", c));
            }
        }
    }

    Ok(tokens)
}

mod tests {
    use crate::lexer::{Token, lex};

    #[test]
    fn test_token_is_binary_operator() {
        assert!(Token::VBar.is_binary_operator());
        assert!(Token::Comma.is_binary_operator());
        assert!(Token::Dash.is_binary_operator());
        assert!(!Token::Literal("a".to_string()).is_binary_operator());
    }

    #[test]
    fn test_token_is_unary_postfix() {
        assert!(Token::KleeneStar.is_unary_postfix());
        assert!(Token::KleenePlus.is_unary_postfix());
        assert!(Token::Question.is_unary_postfix());
        assert!(!Token::Literal("a".to_string()).is_unary_postfix());
    }

    #[test]
    fn test_token_is_unary_prefix() {
        assert!(Token::Caret.is_unary_prefix());
        assert!(!Token::Literal("a".to_string()).is_unary_prefix());
    }

    #[test]
    fn regex_lex() {
        let tokens = lex("a*b").unwrap();
        assert_eq!(tokens.len(), 3);
        assert!(matches!(tokens[0], Token::Literal(ref s) if s == "a"));
        assert!(matches!(tokens[1], Token::KleeneStar));
        assert!(matches!(tokens[2], Token::Literal(ref s) if s == "b"));
    }

    #[test]
    fn regex_lex_everything() {
        let tokens = lex("a*b+c?d|e(f|g)[h-i]{2,3},j.k^l$m").unwrap();
        assert_eq!(tokens.len(), 32);
        assert!(matches!(tokens[0], Token::Literal(ref s) if s == "a"));
        assert!(matches!(tokens[1], Token::KleeneStar));
        assert!(matches!(tokens[2], Token::Literal(ref s) if s == "b"));
        assert!(matches!(tokens[3], Token::KleenePlus));
        assert!(matches!(tokens[4], Token::Literal(ref s) if s == "c"));
        assert!(matches!(tokens[5], Token::Question));
        assert!(matches!(tokens[6], Token::Literal(ref s) if s == "d"));
        assert!(matches!(tokens[7], Token::VBar));
        assert!(matches!(tokens[8], Token::Literal(ref s) if s == "e"));
        assert!(matches!(tokens[9], Token::LParen));
        assert!(matches!(tokens[10], Token::Literal(ref s) if s == "f"));
        assert!(matches!(tokens[11], Token::VBar));
        assert!(matches!(tokens[12], Token::Literal(ref s) if s == "g"));
        assert!(matches!(tokens[13], Token::RParen));
        assert!(matches!(tokens[14], Token::LBracket));
        assert!(matches!(tokens[15], Token::Literal(ref s) if s == "h"));
        assert!(matches!(tokens[16], Token::Dash));
        assert!(matches!(tokens[17], Token::Literal(ref s) if s == "i"));
        assert!(matches!(tokens[18], Token::RBracket));
        assert!(matches!(tokens[19], Token::LBrace));
        assert!(matches!(tokens[20], Token::Literal(ref s) if s == "2"));
        assert!(matches!(tokens[21], Token::Comma));
        assert!(matches!(tokens[22], Token::Literal(ref s) if s == "3"));
        assert!(matches!(tokens[23], Token::RBrace));
        assert!(matches!(tokens[24], Token::Comma));
        assert!(matches!(tokens[25], Token::Literal(ref s) if s == "j"));
        assert!(matches!(tokens[26], Token::Dot));
        assert!(matches!(tokens[27], Token::Literal(ref s) if s == "k"));
        assert!(matches!(tokens[28], Token::Caret));
        assert!(matches!(tokens[29], Token::Literal(ref s) if s == "l"));
        assert!(matches!(tokens[30], Token::Dollar));
        assert!(matches!(tokens[31], Token::Literal(ref s) if s == "m"));
    }

    #[test]
    fn regex_lex_invalid() {
        let result = lex(r"a*b\");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid escape sequence");
    }

    #[test]
    fn regex_lex_unsupported_character() {
        // '@' should be escaped
        let result = lex("a*b@");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Unexpected character: '@'");
    }
}
