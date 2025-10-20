use crate::{
    ast::stmt::{Stmt, StmtKind},
    parse::{Parser, ParsingError},
    tokenize::KeywordKind,
};

impl Parser {
    pub fn parse_while_stmt(&mut self) -> Result<Stmt, ParsingError> {
        let start_offset = self.offset;

        self.consume_keyword(KeywordKind::While)?;
        let condition = Box::new(self.parse_expr(0)?);
        let body = self.parse_codeblock_expr()?;

        let span = self.get_span(start_offset, self.offset - 1)?;

        Ok(Stmt {
            kind: StmtKind::While { condition, body },
            span,
        })
    }
}
