use std::fs;

use codespan_reporting::files::SimpleFiles;
use willow::{
    compile::{compile_file, string_interner::StringInterner},
    hir::ProgramBuilder,
};

fn main() {
    let file_path = match std::env::args().nth(1) {
        Some(path) => path,
        None => {
            eprintln!("{}", "\nExpected file path to the program entry\n");
            return;
        }
    };

    let source_code = match fs::read_to_string(&file_path) {
        Ok(source) => source,
        Err(_) => {
            eprintln!("\n{}{}\n", "Could not read file at path:\n", file_path);
            return;
        }
    };
    let mut string_interner = StringInterner::new();
    let program_builder = ProgramBuilder::new(string_interner);
    let mut files = SimpleFiles::new();

    compile_file(&file_path, &source_code, &mut string_interner, &mut files);

    // let mut coll = string_interner.forward.into_iter().collect::<Vec<_>>();
    // coll.sort_by(|a, b| a.1.cmp(&b.1));
    // coll.iter().for_each(|(name, id)| println!("{}: {}", id, name));
}
