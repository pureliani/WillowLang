use std::path::PathBuf;
use willow::compile::Compiler;

fn main() {
    let file_path = std::env::args()
        .nth(1)
        .expect("\nExpected file path to the program entry\n");

    let mut compiler = Compiler::default();
    compiler.compile(PathBuf::from(file_path));
}
