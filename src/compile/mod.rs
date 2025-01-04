use crate::{
    check::{check_stmts::check_stmts, SemanticError},
    parse::{Parser, ParsingError},
    tokenizer::{TokenizationError, Tokenizer},
};

pub enum CompilationError {
    TokenizerError(TokenizationError),
    ParsingError(ParsingError),
    SemanticError(SemanticError),
    CodegenError(),
}

// pub fn compile(source_code: String) {
//     let tokens = Tokenizer::tokenize(source_code);
//     let parse_tree = Parser::parse(tokens);
//     let analyzed_tree = check_stmts(parse_tree, errors, scope);
// }
