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
    Assignment,
    Addition,
    Subtraction,
    Multiplication,
    Division,
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

fn delimit(
    input: &str,
    from: &mut usize,
    i: usize,
    current: Option<&TrieNode>,
    tokens: &mut Vec<Token>,
) {
    let mut new_token: Option<Token> = None;

    if let Some(token) = current.and_then(|node| node.token.as_ref()) {
        new_token = Some(token.clone());
    } else if (i > *from) {
        let sub = String::from(&input[*from..i]);
        if let Ok(integer) = sub.parse::<i32>() {
            new_token = Some(Token::Integer(integer));
        } else {
            new_token = Some(Token::Identifier(sub));
        }
    }

    if let Some(token) = new_token {
        println!("token {token:?}");
        tokens.push(token);
    }

    *from = i + 1;
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut trie = Trie::new();
    trie.insert("local", Token::Keyword(Keyword::Local));
    trie.insert("function", Token::Keyword(Keyword::Function));
    trie.insert("if", Token::Keyword(Keyword::If));
    trie.insert("else", Token::Keyword(Keyword::Else));
    trie.insert("elseif", Token::Keyword(Keyword::ElseIf));
    trie.insert("while", Token::Keyword(Keyword::While));
    trie.insert("do", Token::Keyword(Keyword::Do));
    trie.insert("return", Token::Keyword(Keyword::Return));
    trie.insert("nil", Token::Keyword(Keyword::Nil));
    trie.insert("=", Token::Operation(Operation::Assignment));
    trie.insert("+", Token::Operation(Operation::Addition));
    trie.insert("-", Token::Operation(Operation::Subtraction));
    trie.insert("*", Token::Operation(Operation::Multiplication));
    trie.insert("/", Token::Operation(Operation::Division));
    trie.insert(",", Token::Operation(Operation::Comma));
    trie.insert("(", Token::ParenthesisOpen);
    trie.insert(")", Token::ParenthesisClose);
    let root = &trie.root;

    let mut tokens: Vec<Token> = Vec::new();
    let mut from: usize = 0;
    let mut current: Option<&TrieNode> = None;
    for (i, c) in input.chars().enumerate() {
        let mut delimiter: Option<Token> = None;
        if c.is_whitespace() {
            delimiter = Some(Token::Whitespace);
        } else {
            if let Some(token) = root.children.get(&c).and_then(|node| node.token.as_ref()) {
                delimiter = Some(token.clone());
            } else {
                match current {
                    Some(_) => (),
                    None => current = Some(root),
                }
            }
        }

        // either a whitespace or a single character token will indicate the end of an identifier/keyword/integer/string literal
        if let Some(token) = delimiter {
            delimit(input, &mut from, i, current, &mut tokens);
            current = None;
            match token {
                Token::Whitespace => (),
                _ => {
                    println!("token {token:?}");
                    tokens.push(token);
                }
            }
        } else {
            // walk down the trie if possible
            current = match current {
                Some(current) => current.children.get(&c),
                None => None,
            }
        }
    }

    tokens
}
