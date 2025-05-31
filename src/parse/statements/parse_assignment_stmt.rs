use crate::{
    ast::{
        base::{
            base_expression::Expr,
            base_statement::{Stmt, StmtKind},
        },
        Span,
    },
    parse::{Parser, ParsingError},
    tokenize::PunctuationKind,
};

impl<'a, 'b> Parser<'a, 'b> {
    pub fn parse_assignment_stmt(&mut self, lhs: Expr) -> Result<Stmt, ParsingError<'a>> {
        let start_offset = self.offset;
        self.consume_punctuation(PunctuationKind::Eq)?;
        let value = self.parse_expr(0, true)?;
        self.consume_punctuation(PunctuationKind::SemiCol)?;
        let span_end = self.get_span(start_offset, self.offset - 1)?;
        Ok(Stmt {
            span: Span {
                start: lhs.span.start,
                end: span_end.end,
            },
            kind: StmtKind::Assignment { target: lhs, value },
        })
    }
}
