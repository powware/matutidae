use std::collections::LinkedList;
use std::path::{Path, PathBuf};

use clap::Parser as ClapParser;

use compiler::parser::Parser;
use compiler::preprocessor::preprocess;
use compiler::tokenizer::tokenize;
use compiler::util::read_lines;

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

    let mut includes: LinkedList<String> = LinkedList::new();
    for include in args.includes {
        includes.push_back(include);
    }

    for file in &args.files {
        let lines = read_lines(file).expect("Unable to open file.");
        includes.push_front(String::from(
            Path::new(file).parent().unwrap().to_str().unwrap(),
        ));
        let lines = preprocess(lines, &mut includes, args.debug);
        includes.pop_front();
        for line in lines {
            println!("{line}");
        }
        // let tokens = tokenize(lines);

        // let mut parser = Parser::new(tokens);
        // parser.parse();
    }
}
