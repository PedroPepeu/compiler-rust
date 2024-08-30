use std::env;
use std::fs;

fn main() {
    let path_to_file = "./file/sample.c";

    let content = fs::read_to_string(path_to_file).expect("Should have been able to read the file");
}
