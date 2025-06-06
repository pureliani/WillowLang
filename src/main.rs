use std::fs;

use ariadne::{Color, Fmt};
use codespan_reporting::files::SimpleFiles;
use willow::compile::{compile_file, string_interner::StringInterner};

fn main() {
    let file_path = match std::env::args().nth(1) {
        Some(path) => path,
        None => {
            eprintln!("{}", "\nExpected file path to the program entry\n".fg(Color::BrightRed));
            return;
        }
    };

    let source_code = match fs::read_to_string(&file_path) {
        Ok(source) => source,
        Err(_) => {
            eprintln!(
                "\n{}{}\n",
                "Could not read file at path:\n".fg(Color::BrightRed),
                file_path.fg(Color::BrightBlue)
            );
            return;
        }
    };

    let mut string_interner = StringInterner::new();
    let mut files = SimpleFiles::<String, String>::new();

    compile_file(&file_path, &source_code, &mut string_interner, &mut file_source_cache);
}
