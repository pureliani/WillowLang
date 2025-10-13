use crate::{
    ast::stmt::{Stmt, StmtKind},
    parse::{Parser, ParsingError},
    tokenize::KeywordKind,
};

impl<'a> Parser<'a> {
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
