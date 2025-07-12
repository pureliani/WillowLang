use crate::{
    ast::expr::{Expr, ExprKind},
    parse::{Parser, ParsingError},
    tokenize::{PunctuationKind, TokenKind},
};

impl<'a, 'b> Parser<'a, 'b> {
    pub fn parse_struct_init_expr(&mut self) -> Result<Expr, ParsingError<'a>> {
        let start_offset = self.offset;

        self.consume_punctuation(PunctuationKind::LBrace)?;
        let args = self.comma_separated(
            |p| {
                let name = p.consume_identifier()?;
                p.consume_punctuation(PunctuationKind::Col)?;
                let value = p.parse_expr(0)?;
                Ok((name, value))
            },
            |p| p.match_token(0, TokenKind::Punctuation(PunctuationKind::RBrace)),
        )?;
        self.consume_punctuation(PunctuationKind::RBrace)?;

        let span = self.get_span(start_offset, self.offset - 1)?;

        Ok(Expr {
            kind: ExprKind::StructLiteral(args),
            span,
        })
    }
}
