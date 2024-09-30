use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn read_lines(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}
