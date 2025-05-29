use crate::{
    ast::base::base_statement::{Stmt, StmtKind},
    parse::{Parser, ParsingError},
    tokenize::KeywordKind,
};

impl<'a, 'b> Parser<'a, 'b> {
    pub fn parse_break_stmt(&mut self) -> Result<Stmt, ParsingError<'a>> {
        let start_offset = self.offset;
        self.consume_keyword(KeywordKind::Break)?;
        let span = self.get_span(start_offset, self.offset - 1)?;
        return Ok(Stmt {
            kind: StmtKind::Break,
            span,
        });
    }
}
