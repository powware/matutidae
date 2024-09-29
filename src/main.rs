use clap::Parser as ClapParser;

use compiler::parser::Parser;
use compiler::preprocessor::preprocess;
use compiler::tokenizer::tokenize;

#[derive(ClapParser, Debug)]
#[command(name = "rust-compiler")]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(required = true, num_args = 1..)]
    files: Vec<String>,

    #[arg(short, long)]
    output: Option<String>,

    #[arg(short, long)]
    includes: Vec<String>,

    #[arg(short, long)]
    debug: bool,
}

fn main() {
    let args = Args::parse();

    for file in &args.files {
        let mut input = std::fs::read_to_string(file).expect("Couldn't open file.");
        input = preprocess(input, args.debug, &args.includes);
        println!("{input}");
        let tokens = tokenize(input);

        let mut parser = Parser::new(tokens);
        parser.parse();
    }
}
