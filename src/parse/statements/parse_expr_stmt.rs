use crate::{
    ast::{
        base::{
            base_expression::Expr,
            base_statement::{Stmt, StmtKind},
        },
        Span,
    },
    parse::{Parser, ParsingError},
    tokenizer::PunctuationKind,
};

impl Parser {
    pub fn parse_expr_stmt(&mut self, lhs: Expr) -> Result<Stmt, ParsingError> {
        let start_offset = self.offset;
        self.consume_punctuation(PunctuationKind::SemiCol)?;
        let span_end = self.get_span(start_offset, self.offset - 1)?;
        Ok(Stmt {
            span: Span {
                start: lhs.span.start,
                end: span_end.end,
            },
            kind: StmtKind::Expression(lhs),
        })
    }
}
