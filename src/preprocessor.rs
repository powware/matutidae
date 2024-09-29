fn include(file: &str, includes: &Vec<String>) -> Option<String> {
    let cwd = std::env::current_dir().unwrap();

    let mut output: Option<String> = None;
    for include in includes {
        std::env::set_current_dir(include);
        if let Ok(input) = std::fs::read_to_string(file) {
            if output != None {
                panic!("two candidates for include");
            }

            output = Some(input);
        }
    }

    std::env::set_current_dir(cwd);

    output
}

pub fn preprocess(input: String, debug: bool, includes: &Vec<String>) -> String {
    let mut output = String::new();

    const PREPROCESSOR: char = '#';

    for line in input.lines() {
        if line.is_empty() {
            output.push_str(line);
            continue;
        }

        if line.chars().nth(0).unwrap() == PREPROCESSOR {
            // regex matches
            if line.starts_with("#include") {
                let include = include();
            } else if line.starts_with("#define") {
            } else {
                panic!("unsupported prepforessor directive");
            }
        } else if line.contains(PREPROCESSOR) {
        } else {
            output.push_str(line);
        }
    }

    output
}
