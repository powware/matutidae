use std::fmt;
use tokenizer::Token;

use crate::tokenizer;

pub enum ASTNode {
    VariableDefiniton,
    FunctionDefinition,
}

type Result<T> = std::result::Result<T, ParserError>;

#[derive(Debug, Clone)]
struct ParserError;

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "parsing error")
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn parse_function(&mut self) {
        // *i = *i + 1;
        // if *i >= tokens.len() {
        //     // error function identifier expected
        // }
        // let identifier = tokens.get(*i).unwrap();
        // match identifier {
        //     Token::Identifier(_) => (),
        //     _ => return, // error
        // }
        // *i = *i + 1;
        // if *i >= tokens.len() {
        //     // error function identifier expected
        // }
        // let paranthesis_open = tokens.get(*i).unwrap();
        // match paranthesis_open {
        //     Token::ParenthesisOpen => (),
        //     _ => return, // error expected parameter list
        // }

        // while  {

        // }
    }

    pub fn peek(&mut self) -> Option<&Token> {
        return self.tokens.get(self.pos);
    }

    pub fn pop(&mut self) {
        self.pos = self.pos + 1;
    }

    pub fn parse(&mut self) -> ASTNode {
        // for token in tokens {
        //     println!("token {token:?}");
        // }

        // while self.pos < self.tokens.len() {
        //     match self.tokens.get(self.pos).unwrap() {
        //         Token::Function => self.parse_function(),
        //         _ => (),
        //     }

        //     self.pos = self.pos + 1;
        // }

        ASTNode::FunctionDefinition
    }
}
