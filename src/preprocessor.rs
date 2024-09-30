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

fn strip_comments(lines: &mut Vec<String>, debug: bool) {
    let mut multiline: bool = false;
    for line in lines {
        if multiline {
            // if {

            // }
            // else {
            //     line.clear();
            // }
            line.clear();
        } else {
            if let Some((_, code, is_comment, multiline_start, multiline_end)) =
                regex_captures!(r"(.*?)\s*(//|(/\*)).*(\*/)?", line)
            {
                multiline = true;
            }
        }
    }
}

fn preprocess_includes(
    lines: Vec<String>,
    includes: &mut LinkedList<String>,
    debug: bool,
    depth: u32,
) -> Vec<String> {
    if depth == MAX_INCLUDE_DEPTH {
        panic!("max include depth of {MAX_INCLUDE_DEPTH:?} reached.");
    }

    let mut output: Vec<String> = Vec::new();

    let mut multiline_comment: bool = false;

    for line in lines {
        if line.is_empty() {
            output.push(line);
            continue;
        }

        if multiline_comment {
            if let Some((_, multiline_end, code_after)) = regex_captures!(r".*?(\*/)(.*)", &line) {
                output.push(String::from(code_after));
                multiline_comment = false;
            }
        }

        if let Some((_, code_before, comment_start, multiline_start, multiline_end, code_after)) =
            regex_captures!(r"(.*?)\s*(//|(/\*)).*(\*/(.?))?", &line)
        {
            if !comment_start.is_empty() {
                output.push(String::from(code_before) + code_after);
            }
            if !multiline_start.is_empty() && multiline_end.is_empty() {
                multiline_comment = true;
            }
        }

        // TODO change the capture for include file names
        if let Some((_, file)) = regex_captures!(r###"^#include\s+"(.+)"[ \t]*$"###, &line) {
            println!("including {file:?}.");
            let (directory, included_lines) = include(file, &includes);
            includes.push_front(directory);
            let preprocessed_lines =
                preprocess_includes(included_lines, includes, debug, depth + 1);
            for preprocessed_line in preprocessed_lines {
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
    let output = preprocess_includes(lines, includes, debug, 0);

    // for line in lines {
    //     if line.is_empty() {
    //         output.push(line);
    //         continue;
    //     }
    // }

    output
}
