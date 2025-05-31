use crate::{
    ast::base::base_statement::{Stmt, StmtKind},
    parse::{Parser, ParsingError},
    tokenize::KeywordKind,
};

impl<'a, 'b> Parser<'a, 'b> {
    pub fn parse_while_stmt(&mut self) -> Result<Stmt, ParsingError<'a>> {
        let start_offset = self.offset;

        self.consume_keyword(KeywordKind::While)?;
        let condition = Box::new(self.parse_expr(0, false)?);
        let body = self.parse_codeblock_expr()?;

        let span = self.get_span(start_offset, self.offset - 1)?;

        Ok(Stmt {
            kind: StmtKind::While { condition, body },
            span,
        })
    }
}
