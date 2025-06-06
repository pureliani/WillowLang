use crate::{
    ast::{
        base::base_expression::{Expr, ExprKind},
        IdentifierNode,
    },
    parse::{Parser, ParsingError},
    tokenize::{PunctuationKind, TokenKind},
};

impl<'a, 'b> Parser<'a, 'b> {
    pub fn parse_struct_init_fields(&mut self) -> Result<Vec<(IdentifierNode, Expr)>, ParsingError<'a>> {
        self.consume_punctuation(PunctuationKind::LBrace)?;
        let args = self.comma_separated(
            |p| {
                let name = p.consume_identifier()?;
                p.consume_punctuation(PunctuationKind::Col)?;
                let value = p.parse_expr(0, true)?;
                Ok((name, value))
            },
            |p| p.match_token(0, TokenKind::Punctuation(PunctuationKind::RBrace)),
        )?;
        self.consume_punctuation(PunctuationKind::RBrace)?;
        Ok(args)
    }

    pub fn parse_struct_init_expr(&mut self, left: Expr) -> Result<Expr, ParsingError<'a>> {
        let start_offset = self.offset;

        let mut span = left.span;
        let fields = self.parse_struct_init_fields()?;
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
