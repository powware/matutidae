use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
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
    Identifier(String),
    Integer(i32),
    StringLiteral(String),
    EqualSign,
    Plus,
    Minus,
    Asterisk,
    ForwardSlash,
    BackSlash,
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
    ParenthesisOpen,
    ParenthesisClose,
    SingleQuote,
    DoubleQuote,
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
    keywords.insert("local", Token::Local);
    keywords.insert("function", Token::Function);
    keywords.insert("if", Token::If);
    keywords.insert("else", Token::Else);
    keywords.insert("elseif", Token::ElseIf);
    keywords.insert("while", Token::While);
    keywords.insert("do", Token::Do);
    keywords.insert("end", Token::End);
    keywords.insert("return", Token::Return);
    keywords.insert("nil", Token::Nil);
    keywords.insert("=", Token::EqualSign);
    keywords.insert("+", Token::Plus);
    keywords.insert("-", Token::Minus);
    keywords.insert("*", Token::Asterisk);
    keywords.insert("/", Token::ForwardSlash);
    keywords.insert("\\", Token::BackSlash);
    keywords.insert(",", Token::Comma);
    keywords.insert("(", Token::ParenthesisOpen);
    keywords.insert(")", Token::ParenthesisClose);
    keywords.insert("'", Token::SingleQuote);
    keywords.insert("\"", Token::DoubleQuote);

    let mut tokens: Vec<Token> = Vec::new();
    let mut from: usize = 0;
    let mut current: Option<&TrieNode> = None;
    let mut i = 0;
    while i < input.len() {
        let c = input.chars().nth(i).unwrap();
        let mut delimiter: Option<Token> = None;
        if c.is_whitespace() {
            delimiter = Some(Token::Whitespace);
        } else {
            if let Some(node) = keywords.root.children.get(&c) {
                if let Some(symbol) = node.token.as_ref() {
                    // check if c is a symbol (child of the trie root with a token)

                    match symbol {
                        Token::SingleQuote | Token::DoubleQuote => {
                            let mut escaped = false;
                            for si in (i + 1)..input.len() {
                                let sc = input.chars().nth(si).unwrap();
                                if let Some(ssymbol) = keywords
                                    .root
                                    .children
                                    .get(&sc)
                                    .and_then(|node| node.token.as_ref())
                                {
                                    if !escaped && ssymbol == symbol {
                                        delimiter = Some(Token::StringLiteral(String::from(
                                            &input[(i + 1)..(si)],
                                        )));
                                        from = si;
                                        i = si;
                                        break;
                                    }

                                    match ssymbol {
                                        Token::BackSlash => escaped = true,
                                        _ => escaped = false,
                                    }
                                }
                            }
                        }
                        _ => delimiter = Some(symbol.clone()),
                    }
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

        i = i + 1;
    }

    tokens
}
