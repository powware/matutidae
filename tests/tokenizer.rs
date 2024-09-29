use compiler::tokenizer::{tokenize, Keyword, Symbol, Token, TokenKind};

#[test]
fn hello_world() {
    let input = r#"/*
  Hello world
 */
print("Hello, World!\n");"#;

    let tokens = tokenize(String::from(input));
    assert_eq!(
        tokens,
        vec![
            Token {
                line: 4,
                column: 1,
                kind: TokenKind::Keyword(Keyword::Print)
            },
            Token {
                line: 4,
                column: 6,
                kind: TokenKind::Symbol(Symbol::LeftParenthesis)
            },
            Token {
                line: 4,
                column: 7,
                kind: TokenKind::StringLiteral(String::from("Hello, World!\n"))
            },
            Token {
                line: 4,
                column: 24,
                kind: TokenKind::Symbol(Symbol::RightParenthesis)
            },
            Token {
                line: 4,
                column: 25,
                kind: TokenKind::Symbol(Symbol::Semicolon)
            },
            Token {
                line: 5,
                column: 1,
                kind: TokenKind::EOF
            }
        ]
    )
}
