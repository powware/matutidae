use std::collections::hash_map::Entry::Occupied;
use std::collections::hash_map::Entry::Vacant;
use std::collections::HashMap;

#[derive(Debug, Clone)]

pub enum Keyword {
    Local,
    Function,
    If,
    Else,
    ElseIf,
    While,
    Do,
    End,
    Return,
    Nil,
}

#[derive(Debug, Clone)]
pub enum Operation {
    EqualSign,
    Plus,
    Minus,
    Asterisk,
    ForwardSlash,
    LessThan,
    LessThanEqual,
    Equal,
    GreaterThanEqual,
    GreaterThan,
    Unequal,
    And,
    Or,
    Not,
    Comma,
}

#[derive(Debug, Clone)]
pub enum Token {
    Keyword(Keyword),
    Identifier(String),
    Integer(i32),
    StringLiteral(String),
    Operation(Operation),
    ParenthesisOpen,
    ParenthesisClose,
    Whitespace,
}

#[derive(Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    token: Option<Token>,
}

#[derive(Default)]
struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Trie::default()
    }

    fn insert(&mut self, name: &str, token: Token) {
        let mut current = &mut self.root;

        for c in name.chars() {
            current = current.children.entry(c).or_default();
        }

        current.token = Some(token);
    }
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut keywords = Trie::new();
    keywords.insert("local", Token::Keyword(Keyword::Local));
    keywords.insert("function", Token::Keyword(Keyword::Function));
    keywords.insert("if", Token::Keyword(Keyword::If));
    keywords.insert("else", Token::Keyword(Keyword::Else));
    keywords.insert("elseif", Token::Keyword(Keyword::ElseIf));
    keywords.insert("while", Token::Keyword(Keyword::While));
    keywords.insert("do", Token::Keyword(Keyword::Do));
    keywords.insert("return", Token::Keyword(Keyword::Return));
    keywords.insert("nil", Token::Keyword(Keyword::Nil));
    keywords.insert("=", Token::Operation(Operation::EqualSign));
    keywords.insert("+", Token::Operation(Operation::Plus));
    keywords.insert("-", Token::Operation(Operation::Minus));
    keywords.insert("*", Token::Operation(Operation::Asterisk));
    keywords.insert("/", Token::Operation(Operation::ForwardSlash));
    keywords.insert(",", Token::Operation(Operation::Comma));
    keywords.insert("(", Token::ParenthesisOpen);
    keywords.insert(")", Token::ParenthesisClose);

    let mut tokens: Vec<Token> = Vec::new();
    let mut from: usize = 0;
    let mut current: Option<&TrieNode> = None;
    for (i, c) in input.chars().enumerate() {
        let mut delimiter: Option<Token> = None;
        if c.is_whitespace() {
            delimiter = Some(Token::Whitespace);
        } else {
            if let Some(node) = keywords.root.children.get(&c) {
                if let Some(symbol) = node.token.as_ref() {
                    // check if c is a symbol (child of the trie root with a token)
                    delimiter = Some(symbol.clone());
                } else if i == from {
                    // otherwise if the previous was a delimiter or its the first beginning of the input, we are starting a keyword
                    current = Some(node);
                }
            }
        }

        // either a whitespace or a single character token will indicate the end of an identifier/keyword/integer/string literal
        if let Some(symbol) = delimiter {
            if let Some(keyword) = current.and_then(|node| node.token.as_ref()) {
                tokens.push(keyword.clone());
            } else if i > from {
                let sub = String::from(&input[from..i]);
                if let Ok(integer) = sub.parse::<i32>() {
                    tokens.push(Token::Integer(integer));
                } else {
                    tokens.push(Token::Identifier(sub));
                }
            }

            from = i + 1;
            current = None;

            match symbol {
                Token::Whitespace => (),
                _ => {
                    tokens.push(symbol);
                }
            }
        } else if i > from {
            // walk down the trie if possible
            current = current.and_then(|node| node.children.get(&c));
        }
    }

    tokens
}
