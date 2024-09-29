use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Multiply,
    Divide,
    Modulo,
    Plus,
    Minus,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    Equal,
    NotEqual,
    Not,
    Assign,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Symbol {
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,
    Semicolon,
    Comma,
    SingleQuote,
    DoubleQuote,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    If,
    Else,
    While,
    Print,
    Putc,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Operator(Operator),
    Symbol(Symbol),
    Keyword(Keyword),
    Identifier(String),
    IntegerLiteral(u32),
    CharLiteral(char),
    StringLiteral(String),
    Whitespace,
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub line: u32,
    pub column: u32,
    pub kind: TokenKind,
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

pub fn tokenize(input: String) -> Vec<Token> {
    let mut trie = Trie::new();
    // trie.insert("local", Token::Local);
    // trie.insert("function", Token::Function);
    // trie.insert("if", Token::If);
    // trie.insert("else", Token::Else);
    // trie.insert("elseif", Token::ElseIf);
    // trie.insert("while", Token::While);
    // trie.insert("do", Token::Do);
    // trie.insert("end", Token::End);
    // trie.insert("return", Token::Return);
    // trie.insert("nil", Token::Nil);
    // trie.insert("=", Token::EqualSign);
    // trie.insert("+", Token::Plus);
    // trie.insert("-", Token::Minus);
    // trie.insert("*", Token::Star);
    // trie.insert("/", Token::ForwardSlash);
    // trie.insert("\\", Token::BackSlash);
    // trie.insert(",", Token::Comma);
    // trie.insert("(", Token::ParenthesisOpen);
    // trie.insert(")", Token::ParenthesisClose);
    // trie.insert("'", Token::SingleQuote);
    // trie.insert("\"", Token::DoubleQuote);

    let mut tokens: Vec<Token> = Vec::new();
    // let mut from: usize = 0;
    // let mut current: Option<&TrieNode> = None;
    // let mut i = 0;
    // 'outer: while i < input.len() {
    //     let c = input.chars().nth(i).unwrap();
    //     let mut delimiter: Option<Token> = None;
    //     if c.is_whitespace() {
    //         delimiter = Some(Token::Whitespace);
    //     } else {
    //         if let Some(node) = trie.root.children.get(&c) {
    //             if let Some(symbol) = node.token.as_ref() {
    //                 // check if c is a symbol (child of the trie root with a token)

    //                 // handle string literals
    //                 match symbol {
    //                     Token::SingleQuote | Token::DoubleQuote => {
    //                         let mut escaped = false;
    //                         for si in (i + 1)..input.len() {
    //                             let sc = input.chars().nth(si).unwrap();
    //                             if let Some(ssymbol) = trie
    //                                 .root
    //                                 .children
    //                                 .get(&sc)
    //                                 .and_then(|node| node.token.as_ref())
    //                             {
    //                                 if !escaped && ssymbol == symbol {
    //                                     tokens.push(Token::StringLiteral(String::from(
    //                                         &input[(i + 1)..(si)],
    //                                     )));
    //                                     i = si + 1;
    //                                     from = i;
    //                                     current = None;
    //                                     continue 'outer;
    //                                 }

    //                                 match ssymbol {
    //                                     Token::BackSlash => escaped = true,
    //                                     _ => escaped = false,
    //                                 }
    //                             }
    //                         }
    //                         panic!("string literal does not close");
    //                     }
    //                     _ => delimiter = Some(symbol.clone()),
    //                 }
    //             } else if i == from {
    //                 // otherwise if the previous was a delimiter or its the first beginning of the input, we are starting a keyword
    //                 current = Some(node);
    //             }
    //         }
    //     }

    //     // either a whitespace or a single character token will indicate the end of an identifier/keyword/integer/string literal
    //     if let Some(symbol) = delimiter {
    //         if let Some(keyword) = current.and_then(|node| node.token.as_ref()) {
    //             tokens.push(keyword.clone());
    //         } else if i > from {
    //             let sub = String::from(&input[from..i]);
    //             if sub.chars().nth(0).unwrap().is_digit(10) {
    //                 if let Ok(integer) = sub.parse::<i32>() {
    //                     tokens.push(Token::Integer(integer));
    //                 } else {
    //                     panic!("invalid integer");
    //                 }
    //             } else {
    //                 tokens.push(Token::Identifier(sub));
    //             }
    //         }

    //         from = i + 1;
    //         current = None;

    //         match symbol {
    //             Token::Whitespace => (),
    //             _ => {
    //                 tokens.push(symbol);
    //             }
    //         }
    //     } else if i > from {
    //         // walk down the trie if possible
    //         current = current.and_then(|node| node.children.get(&c));
    //     }

    //     i = i + 1;
    // }

    tokens
}
