use crate::{
    ast::expr::{Expr, ExprKind},
    parse::{Parser, ParsingError},
    tokenize::{PunctuationKind, TokenKind},
};

impl<'a, 'b> Parser<'a, 'b> {
    pub fn parse_struct_init_expr(&mut self, left: Expr) -> Result<Expr, ParsingError<'a>> {
        let start_offset = self.offset;
        let mut span = left.span;
        self.consume_punctuation(PunctuationKind::LBrace)?;
        let fields = self.comma_separated(
            |p| {
                let name = p.consume_identifier()?;
                p.consume_punctuation(PunctuationKind::Col)?;
                let value = p.parse_expr(0)?;
                Ok((name, value))
            },
            |p| p.match_token(0, TokenKind::Punctuation(PunctuationKind::RBrace)),
        )?;
        self.consume_punctuation(PunctuationKind::RBrace)?;

        let span_end = self.get_span(start_offset, self.offset - 1)?;
        span.end = span_end.end;

        Ok(Expr {
            kind: ExprKind::StructInit {
                left: Box::new(left),
                fields,
            },
            span,
        })
    }
}
