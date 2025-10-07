use crate::{
    ast::expr::Expr,
    parse::{Parser, ParsingError},
    tokenize::PunctuationKind,
};

impl<'a, 'b> Parser<'a, 'b> {
    pub fn parse_parenthesized_expr(&mut self) -> Result<Expr, ParsingError<'a>> {
        let start_offset = self.offset;

        self.consume_punctuation(PunctuationKind::LParen)?;
        let expr = self.parse_expr(0)?;
        self.consume_punctuation(PunctuationKind::RParen)?;

        let span = self.get_span(start_offset, self.offset - 1)?;

        Ok(Expr {
            kind: expr.kind,
            span,
        })
    }
}
