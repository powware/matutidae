use clap::{arg, Command};
use tokenizer::tokenize;

mod parser;
mod tokenizer;

fn main() {
    let matches = Command::new("natutidae")
                .author("powware, powwared@gmail.com")
    .version("0.1.0")
    .about("Interpreter of the Matutidae scripting language.")
    .arg(arg!(<FILE> "file to interpret"))
    .after_help("Matutidae is a Lua inspired scripting language written in rust. The name stems from a family of crabs also referred to as moon crabs.\nGiven that Lua means moon and rust programmers are referred to as rustaceans, this name was only a logical conclusion.")
    .get_matches();

    let input = matches.get_one::<String>("FILE").unwrap();
    let content = std::fs::read_to_string(input).expect("Couldn't open file.");
    let tokens = tokenizer::tokenize(&content);

    let mut i: usize = 0;

    let tokens = tokenize(input);
    Parser::parse(tokens);
}
