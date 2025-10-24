use crate::{
    ast::stmt::{Stmt, StmtKind},
    parse::{Parser, ParsingError},
    tokenize::KeywordKind,
};

impl Parser {
    pub fn parse_continue_stmt(&mut self) -> Result<Stmt, ParsingError> {
        let start_offset = self.offset;
        self.consume_keyword(KeywordKind::Continue)?;
        let span = self.get_span(start_offset, self.offset - 1)?;

        Ok(Stmt {
            kind: StmtKind::Continue,
            span,
        })
    }
}
