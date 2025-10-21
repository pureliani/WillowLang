use std::path::PathBuf;
use willow::compile::Compiler;

fn main() {
    let file_path = match std::env::args().nth(1) {
        Some(path) => path,
        None => {
            eprintln!("\nExpected file path to the program entry\n");
            return;
        }
    };

    let mut compiler = Compiler::new();
    compiler.compile(PathBuf::from(file_path));
}
