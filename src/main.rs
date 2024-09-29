use clap::{arg, Command};
use parser::Parser;
use tokenizer::tokenize;

mod parser;
mod preprocessor;
mod tokenizer;

fn main() {
    let matches = Command::new("rust-compiler")
        .author("powware, powwared@gmail.com")
        .version("0.1.0")
        .about("Compiler")
        .args([
            arg!(<FILE> "file to interpret"),
            arg!(-I <DIR> "Include Directory"),
        ])
        .after_help("redo")
        .get_matches();

    let file = matches.get_one::<String>("FILE").unwrap();
    let input = std::fs::read_to_string(file).expect("Couldn't open file.");
    // input = preprocess(input);
    let tokens = tokenize(&input);

    let mut parser = Parser::new(tokens);
    parser.parse();
}
