use std::collections::LinkedList;
use std::path::{Path, PathBuf};

use crate::util::read_lines;
use lazy_regex::regex_captures;

const MAX_INCLUDE_DEPTH: u32 = 5;

fn include(file: &str, includes: &LinkedList<String>) -> (String, Vec<String>) {
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
        if let Some((_, code_after)) = regex_captures!(r".*?\*/(.*)", &line) {
            return strip_comments(String::from(code_after), false);
        } else {
            return (String::new(), true);
        }
    }

    if let Some((
        _,
        code_before,
        is_comment,
        single_line,
        multi_line,
        code_after_multiline,
        multi_line_open,
    )) = regex_captures!(r"^(.*?)\s*((//.*)|(/\*.*\*/(.*))|(/\*.*))$", &line)
    {
        if !single_line.is_empty() {
            return (String::from(code_before), false);
        }
        if !multi_line.is_empty() {
            return strip_comments(String::from(code_before) + code_after_multiline, false);
        }
        if !multi_line_open.is_empty() {
            return (String::from(code_before), true);
        }
    }

    (line, false)
}

fn include_and_strip_comments(
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
        if let Some((_, file)) = regex_captures!(r###"\s*#\s*include\s+"(.+)"\s*"###, &line) {
            println!("including {file:?}.");
            let (directory, included_lines) = include(file, &includes);
            includes.push_front(directory);
            let included_and_stripped_lines =
                include_and_strip_comments(included_lines, includes, debug, depth + 1);
            for preprocessed_line in included_and_stripped_lines {
                output.push(preprocessed_line);
            }
            includes.pop_front();
        } else {
            output.push(line);
        }
    }

    output
}

pub fn preprocess(
    lines: Vec<String>,
    includes: &mut LinkedList<String>,
    debug: bool,
) -> Vec<String> {
    let output = include_and_strip_comments(lines, includes, debug, 0);

    // for line in lines {
    //     if line.is_empty() {
    //         output.push(line);
    //         continue;
    //     }
    // }

    output
}
