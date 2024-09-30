use std::collections::{HashMap, LinkedList};
use std::path::{Path, PathBuf};

use crate::util::read_lines;
use lazy_regex::regex_captures;

const MAX_INCLUDE_DEPTH: u32 = 5;

fn resolve_include(file: &str, includes: &LinkedList<String>) -> (String, Vec<String>) {
    let cwd = std::env::current_dir().unwrap();

    for include in includes {
        std::env::set_current_dir(include);
        if let Ok(lines) = read_lines(file) {
            return (
                String::from(Path::new(file).parent().unwrap().to_str().unwrap()),
                lines,
            );
        }
    }

    std::env::set_current_dir(cwd);

    panic!("unable to include file {file:?}.");
}

fn strip_comments(line: String, mut multiline: bool) -> (String, bool) {
    if multiline {
        if let Some((_, code_after)) = regex_captures!(r".*?\*/\s*(.*)", &line) {
            return strip_comments(String::from(code_after), false);
        } else {
            return (String::new(), true);
        }
    }

    if let Some((
        _,
        code_before,
        comment,
        single_line,
        multi_line,
        after_multiline,
        multi_line_open,
    )) = regex_captures!(r"^(.*?)\s*((//.*)|(/\*.*\*/\s*(.*))|(/\*.*))$", &line)
    {
        if !single_line.is_empty() {
            return (String::from(code_before), false);
        }
        if !multi_line.is_empty() {
            return strip_comments(String::from(code_before) + after_multiline, false);
        }
        if !multi_line_open.is_empty() {
            return (String::from(code_before), true);
        }
    }

    (line, false)
}

fn resolve_includes_and_strip_comments(
    lines: Vec<String>,
    includes: &mut LinkedList<String>,
    debug: bool,
    depth: u32,
) -> Vec<String> {
    if depth == MAX_INCLUDE_DEPTH {
        panic!("max include depth of {MAX_INCLUDE_DEPTH:?} reached.");
    }

    let mut output: Vec<String> = Vec::new();

    let mut multiline: bool = false;

    for mut line in lines {
        if line.is_empty() {
            output.push(line);
            continue;
        }

        (line, multiline) = strip_comments(line, multiline);

        // TODO change the capture for include file names
        if let Some((_, file)) = regex_captures!(r###"^\s*#\s*include\s+"(.+)"\s*$"###, &line) {
            println!("including \"{file}\"");

            let (directory, lines) = resolve_include(file, &includes);

            includes.push_front(directory);
            let lines = resolve_includes_and_strip_comments(lines, includes, debug, depth + 1);
            for line in lines {
                output.push(line);
            }
            includes.pop_front();
        } else {
            output.push(line);
        }
    }

    output
}

struct Function {}

enum Macro {
    Object(String),
    Function,
}

pub fn preprocess(
    mut lines: Vec<String>,
    includes: &mut LinkedList<String>,
    debug: bool,
) -> Vec<String> {
    lines = resolve_includes_and_strip_comments(lines, includes, debug, 0);

    for line in &mut lines {
        if line.is_empty() {
            continue;
        }

        if let Some((_, name, arg_list, args, expansion)) =
            regex_captures!(r###"^\s*#\s*define\s+(.+?)(\((.*?)\))?\s(.*)\s*$"###, &line)
        {
            print!("defining {name}");
            if !args.is_empty() {
                let args: Vec<String> = args
                    .split(',')
                    .map(|part| part.trim().to_string())
                    .collect();

                print!(" with args {args:?}");

                let splits: Vec<String> = Vec::new();
                splits.push(String::from(expansion));
                for arg in args {
                    for split in splits {
                        let split: Vec<String> = split
                            .split(&arg)
                            .map(|part| part.trim().to_string())
                            .collect();
                    }
                }
            }

            println!(" expands to \"{expansion}\"");

            line.clear();
        }
    }

    lines
}
