use std::collections::hash_map::Entry::Occupied;
use std::collections::hash_map::Entry::Vacant;
use std::collections::HashMap;

#[derive(Clone)]

pub enum Keyword {
    Local,
    Function,
    If,
    Else,
    ElseIf,
    While,
    Do,
    End,
    Nil,
}

#[derive(Clone)]
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
}

#[derive(Clone)]
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
    let mut trie = Trie::new();
    trie.insert("local", Token::Keyword(Keyword::Local));
    trie.insert("=", Token::Operation(Operation::Assignment));
    trie.insert("(", Token::ParenthesisOpen);
    trie.insert(")", Token::ParenthesisClose);
    trie.insert(" ", Token::Whitespace);

    let mut tokens: Vec<Token> = Vec::new();
    let mut start: Option<usize> = None;
    let i: usize = 0;
    while i < input.len() {
        let c = input.chars().nth(i).unwrap();
        trie.root.children.get(&c);
    }

    tokens
}
