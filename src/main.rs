use std::fs;

use willow::compile::compile_file;

fn main() {
    let file_path = std::env::args().nth(1).expect("Expected file path");
    let source_code = fs::read_to_string(&file_path).unwrap();

    compile_file(&file_path, &source_code);
}
