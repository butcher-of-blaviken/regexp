use std::rc::{Rc, Weak};

#[derive(Debug, Clone, PartialEq)]
enum Expression {
    /// Program is the root of the regex expression tree.
    /// Its data is simply the raw input regex itself alongside the expressions
    /// that make up the regex.
    /// It should never be referenced by other expressions.
    /// For example, if the regex is "a*b", the Program expression would contain "a*b"
    /// and the expressions would be [UnaryPostfix(KleeneStar, Literal("a")), Literal("b")].
    Program(String, Vec<Box<Expression>>),

    /// Unary expressions are those that apply to a single expression.
    /// Token is either Kleene star, Kleene plus, or question.
    UnaryPostfix(Token, Box<Expression>),

    /// Parens capture subexpressions in regex.
    /// For example, (a|b) captures the subexpression a|b.
    /// This is useful for grouping and precedence.
    Parens(Box<Expression>),

    /// A bracket expression. Matches a single character that is contained within the brackets.
    /// For example, [abc] matches "a", "b", or "c".
    /// [a-z] specifies a range which matches any lowercase letter from "a" to "z".
    /// These forms can be mixed: [abcx-z] matches "a", "b", "c", "x", "y", or "z", as does [a-cx-z].
    /// The - character is treated as a literal character if it is the last or the first
    /// (after the ^, if present) character within the brackets: [abc-], [-abc], [^-abc]
    /// Backslash escapes are not allowed. The ] character can be included in a bracket
    /// expression if it is the first (after the ^, if present) character: []abc], [^]abc].
    /// [abcx-z] parses into something like [a|b|c|x-z].
    ///
    /// Since the | operator is commutative, the parse tree can look something like
    /// this:
    ///
    ///     Bracket
    ///     ├── VBar
    ///     │   ├── Literal "a"
    ///     │   ├── VBar
    ///     │       ├── Literal "b"
    ///     │       ├── VBar
    ///     │           ├── Literal "c"
    ///     │           └── Range "x-z"
    Bracket(Box<Expression>),

    // A range expression matches any character that is within the specified range.
    // For example, Range('a', 'z') matches any lowercase letter from "a" to "z".
    // The range is inclusive, meaning both endpoints are included.
    Range(char, char),

    // A literal expression matches a single character or a sequence of characters.
    // For example, Literal("abc") matches the string "abc".
    Literal(String),

    // A vertical bar (|) is used to separate alternatives in regex.
    // For example, a|b matches either "a" or "b".
    // The parse tree for this would look like:
    //     VBar
    //     ├── Literal "a"
    //     └── Literal "b"
    VBar(Box<Expression>, Box<Expression>),

    // A comma (,) is used in regex to specify a range of repetitions.
    // For example, a{2,5} matches "aa", "aaa", "aaaa", or "aaaaa".
    // The parse tree for this would look like:
    //     Comma
    //     ├── Literal "a"
    //     └── Range(2, 5)
    // This means that the literal "a" can appear between 2 and 5 times
    // in the matched string.
    Comma(Box<Expression>, i32, i32),
}

impl Expression {
    fn eval(&self, input: &str) -> bool {
        // This is a placeholder for the evaluation logic.
        // In a real implementation, this would check if the input matches the expression.
        // For now, we just return false.
        false
    }
}

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
    fn is_binary_operator(&self) -> bool {
        matches!(self, Token::VBar | Token::Comma | Token::Dash)
    }

    fn is_unary_postfix(&self) -> bool {
        matches!(self, Token::KleeneStar | Token::KleenePlus | Token::Question)
    }

    fn is_unary_prefix(&self) -> bool {
        matches!(self, Token::Caret)
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
    expression: String,
    tokens: Vec<Token>,
    current_token_idx: usize,
    current_node: Option<ASTNode>,
}

impl Parser {
    fn new(expression: &str, tokens: Vec<Token>) -> Self {
        Parser {
            expression: expression.to_string(),
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
                    _ if token.is_unary_postfix() => {
                        self.parse_unary_postfix(token.clone())
                    },
                    Token::LParen => {
                        self.parse_parens()
                    }
                    _ => Err(format!("Unexpected token: {:?}", token)),
                }
            },
            None => Err("No tokens to parse".into()),
        }
    }

    fn parse_parens(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn parse_literal(&mut self, token: Token) -> Result<(), String> {
        match token {
            Token::Literal(_) => {
                if let Some(ref mut node) = self.current_node {
                    node.add_child(ASTNode::new(Some(token)));
                }
                self.current_token_idx += 1;
                Ok(())
            },
            _ => Err(format!("Expected a literal, found: {:?}", token)),
        }
    }

    fn parse_unary_postfix(&mut self, token: Token) -> Result<(), String> {
        match token {
            _ if token.is_unary_postfix() => {
                // unary postfix operators like Kleene star, plus, or question
                // apply to the last node in the current AST.
                match self.current_node {
                    None => return Err("No current node to apply Kleene star to".into()),
                    Some(ref mut node) => {
                        let child = node.pop_child().ok_or("No child to apply Kleene star to")?;
                        let mut unary_postfix_node = ASTNode::new(Some(token));
                        unary_postfix_node.add_child(child);
                        node.add_child(unary_postfix_node);
                        self.current_token_idx += 1;
                        Ok(())
                    },
                }
            },
            _ => Err(format!("Expected unary postfix token, found: {:?}", token)),
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
    fn regex_parse_unary_postfix() {
        let mut input = "a*b";
        let kleene_star_tokens = Regex::lex(input.clone()).unwrap();
        let mut kleene_star_parser = Parser::new(input.clone(), kleene_star_tokens);
        assert!(kleene_star_parser.parse().is_ok());
        if let Some(ast) = kleene_star_parser.current_node {
            assert_eq!(ast.op, Some(Token::Program));
            assert_eq!(ast.children.len(), 2);
            if let Some(child) = ast.children.first() {
                assert_eq!(child.op, Some(Token::KleeneStar));
            }
        } else {
            panic!("Expected a non-empty AST");
        }

        input = "a+b";
        let kleene_plus_tokens = Regex::lex(input).unwrap();
        let mut kleene_plus_parser = Parser::new(input, kleene_plus_tokens);
        assert!(kleene_plus_parser.parse().is_ok());
        if let Some(ast2) = kleene_plus_parser.current_node {
            assert_eq!(ast2.op, Some(Token::Program));
            assert_eq!(ast2.children.len(), 2);
            if let Some(child) = ast2.children.first() {
                assert_eq!(child.op, Some(Token::KleenePlus));
            }
        } else {
            panic!("Expected a non-empty AST");
        }

        input = "a?b";
        let question_tokens = Regex::lex(input).unwrap();
        let mut question_parser = Parser::new(input,question_tokens);
        assert!(question_parser.parse().is_ok());
        if let Some(ast3) = question_parser.current_node {
            assert_eq!(ast3.op, Some(Token::Program));
            assert_eq!(ast3.children.len(), 2);
            if let Some(child) = ast3.children.first() {
                assert_eq!(child.op, Some(Token::Question));
            }
        } else {
            panic!("Expected a non-empty AST");
        }
    }
}
