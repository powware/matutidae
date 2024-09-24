use std::fmt;
use tokenizer::{Keyword, Token};

use crate::tokenizer;

enum ASTNode {
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

struct Parser {
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

    pub fn parse(&mut self) -> ASTNode {
        while self.pos < self.tokens.len() {
            match self.tokens.get(self.pos).unwrap() {
                Token::Keyword(keyword) => match keyword {
                    Keyword::Function => {
                        self.parse_function();
                    }
                    _ => (),
                },
                _ => (),
            }

            self.pos = self.pos + 1;
        }

        ASTNode::FunctionDefinition
    }
}

pub fn parse(tokens: Vec<Token>) -> ASTNode {
    let mut parser = Parser::new(tokens);
    parser.parse()
}
