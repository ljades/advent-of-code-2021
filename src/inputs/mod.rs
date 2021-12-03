pub mod i1;
use std::fs;

pub fn read_input(filename: &str) -> String {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    contents.trim_end().to_string()
}
