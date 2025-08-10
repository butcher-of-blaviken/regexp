use std::rc::{Rc, Weak};

#[derive(Debug, Clone, PartialEq)]
enum Token {
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
    Dash,
    Dot,
    Caret,
    Dollar,
}

impl Token {
    fn is_binary_operator(&self) -> bool {
        matches!(self, Token::VBar | Token::Comma | Token::Dash)
    }

    fn is_unary_operator(&self) -> bool {
        matches!(self, Token::KleeneStar | Token::KleenePlus | Token::Question | Token::Caret)
    }
}

pub struct Regex {
    tokens: Vec<Token>,
}

struct DFA {
}

impl DFA {
    fn new() -> Self {
        DFA {
            // Initialize with an empty structure
        }
    }

    fn add_state(&mut self) {
        // Here you would typically add a state to the DFA
        // For now, this is just a placeholder
    }
}

struct ASTNode {
    parent: Option<Weak<Rc<ASTNode>>>,
    children: Vec<Box<ASTNode>>,
    op: Option<Token>,
}

impl ASTNode {
    fn new(op: Option<Token>) -> Self {
        ASTNode {
            parent: None,
            children: Vec::new(),
            op,
        }
    }

    fn add_child(&mut self, child: ASTNode) {
        self.children.push(Box::new(child));
    }

    fn pop_child(&mut self) -> Option<ASTNode> {
        self.children.pop().map(|child| *child)
    }
}

struct AST {
    root: Option<ASTNode>,
}

impl AST {
    fn new() -> Self {
        AST {
            root: None,
        }
    }

    fn add_node(&mut self, node: ASTNode) {
        match self.root {
            Some(ref mut root) => root.add_child(node),
            None => self.root = Some(node),
        }
    }

    fn visualize(&self) -> String {
        // Visualize the AST as a string with indentation
        fn visualize_node(node: &ASTNode, depth: usize) -> String {
            let indent = "  ".repeat(depth);
            let mut result = String::new();
            if let Some(ref op) = node.op {
                result.push_str(&format!("{}{:?}\n", indent, op));
            }
            for child in &node.children {
                result.push_str(&visualize_node(child, depth + 1));
            }
            result
        }
        if let Some(ref root) = self.root {
            visualize_node(root, 0)
        } else {
            String::from("Empty AST")
        }
    }
}

struct Parser {
    tokens: Vec<Token>,
    current_token_idx: usize,
    current_node: Option<ASTNode>,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current_token_idx: 0,
            current_node: None,
        }
    }

    fn parse(&mut self) -> Result<(), String> {
        self.current_node = Some(ASTNode::new(Some(Token::Program)));
        while self.current_token_idx < self.tokens.len() {
            match self.parse_next() {
                Ok(_) => {},
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    fn parse_next(&mut self) -> Result<(), String> {
        match self.tokens.get(self.current_token_idx) {
            Some(token) => {
                match token {
                    Token::Literal(_) => {
                        self.parse_literal(token.clone())
                    },
                    Token::KleeneStar => {
                        self.parse_kleene_star(token.clone())
                    },
                    _ => Err(format!("Unexpected token: {:?}", token)),
                }
            },
            None => Err("No tokens to parse".into()),
        }
    }

    fn parse_literal(&mut self, token: Token) -> Result<(), String> {
        match token {
            Token::Literal(_) => {
                if let Some(ref mut node) = self.current_node {
                    node.add_child(ASTNode::new(Some(token.clone())));
                }
                self.current_token_idx += 1;
                Ok(())
            },
            _ => Err(format!("Expected a literal, found: {:?}", token)),
        }
    }

    fn parse_kleene_star(&mut self, token: Token) -> Result<(), String> {
        match token {
            Token::KleeneStar => {
                // Kleene star nodes are unary operators and will be applied
                // to the previous node in the AST.
                match self.current_node {
                    None => return Err("No current node to apply Kleene star to".into()),
                    Some(ref mut node) => {
                        let child = node.pop_child().ok_or("No child to apply Kleene star to")?;
                        let mut kleene_node = ASTNode::new(Some(Token::KleeneStar));
                        kleene_node.add_child(child);
                        node.add_child(kleene_node);
                        self.current_token_idx += 1;
                        Ok(())
                    },
                }
            },
            _ => Err(format!("Expected Kleene star, found: {:?}", token)),
        }
    }
}

impl Regex {
    fn lex(expression: &str) -> Result<Vec<Token>, String> {
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

    pub fn new(expression: &str) -> Result<Regex, String> {
        // lex the expression
        let tokens = Regex::lex(expression)?;
        // Here you would typically compile the regex expression
        // For now, we just return an empty Regex instance
        Ok(Regex{
            tokens,
        })
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
    fn regex_lex() {
        let tokens = Regex::lex("a*b").unwrap();
        assert_eq!(tokens.len(), 3);
        assert!(matches!(tokens[0], Token::Literal(ref s) if s == "a"));
        assert!(matches!(tokens[1], Token::KleeneStar));
        assert!(matches!(tokens[2], Token::Literal(ref s) if s == "b"));
    }

    #[test]
    fn regex_lex_everything() {
        let tokens = Regex::lex("a*b+c?d|e(f|g)[h-i]{2,3},j.k^l$m").unwrap();
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
        let result = Regex::lex(r"a*b\");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid escape sequence");
    }

    #[test]
    fn regex_lex_unsupported_character() {
        // '@' should be escaped
        let result = Regex::lex("a*b@");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Unexpected character: '@'");
    }

    #[test]
    fn regex_new() {
        let regex = Regex::new("a*b").unwrap();
        assert!(regex.is_match("aaab"));
        assert!(!regex.is_match("b"));
    }

    #[test]
    fn regex_parse() {
        let tokens = Regex::lex("a*b").unwrap();
        let mut parser = Parser::new(tokens);
        assert!(parser.parse().is_ok());
        if let Some(ast) = parser.current_node {
            assert_eq!(ast.op, Some(Token::Program));
            assert_eq!(ast.children.len(), 2);
            if let Some(child) = ast.children.first() {
                assert_eq!(child.op, Some(Token::KleeneStar));
            }
        } else {
            panic!("Expected a non-empty AST");
        }
    }
}
