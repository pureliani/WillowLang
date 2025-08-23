use crate::{
    ast::stmt::{Stmt, StmtKind},
    parse::{Parser, ParsingError},
    tokenize::{KeywordKind, PunctuationKind},
};

impl<'a, 'b> Parser<'a, 'b> {
    pub fn parse_return_stmt(&mut self) -> Result<Stmt, ParsingError<'a>> {
        let start_offset = self.offset;

        self.consume_keyword(KeywordKind::Return)?;
        let value = self.parse_expr(0)?;
        self.consume_punctuation(PunctuationKind::SemiCol)?;

        let span = self.get_span(start_offset, self.offset - 1)?;

        return Ok(Stmt {
            kind: StmtKind::Return { value },
            span,
        });
    }
}
