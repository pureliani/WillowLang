use crate::{
    ast::base::base_statement::{Stmt, StmtKind},
    parse::{Parser, ParsingError},
    tokenizer::{KeywordKind, PunctuationKind},
};

impl Parser {
    pub fn parse_return_stmt(&mut self) -> Result<Stmt, ParsingError> {
        let start_offset = self.offset;

        self.consume_keyword(KeywordKind::Return)?;
        let expr = self.parse_expr(0)?;
        self.consume_punctuation(PunctuationKind::SemiCol)?;

        let span = self.get_span(start_offset, self.offset - 1)?;

        return Ok(Stmt {
            kind: StmtKind::Return(expr),
            span,
        });
    }
}
