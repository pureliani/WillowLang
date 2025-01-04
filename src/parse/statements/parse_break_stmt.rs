use crate::{
    ast::base::base_statement::{Stmt, StmtKind},
    parse::{Parser, ParsingError},
    tokenizer::KeywordKind,
};

impl Parser {
    pub fn parse_break_stmt(&mut self) -> Result<Stmt, ParsingError> {
        let start_offset = self.offset;
        self.consume_keyword(KeywordKind::Break)?;
        let span = self.get_span(start_offset, self.offset - 1)?;
        return Ok(Stmt {
            kind: StmtKind::Break,
            span,
        });
    }
}
