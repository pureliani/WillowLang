use std::fs;

use willow::compile::{compile_file, string_interner::StringInterner};

fn main() {
    let file_path = std::env::args().nth(1).expect("Expected file path");
    let source_code = fs::read_to_string(&file_path).unwrap();
    let mut string_interner = StringInterner::new();

    compile_file(&file_path, &source_code, &mut string_interner);
}
