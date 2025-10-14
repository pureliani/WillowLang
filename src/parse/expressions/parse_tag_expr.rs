use crate::{
    ast::expr::{Expr, ExprKind},
    parse::{Parser, ParsingError},
    tokenize::{PunctuationKind, TokenKind},
};

impl<'a> Parser<'a> {
    pub fn parse_tag_expr(&mut self) -> Result<Expr, ParsingError> {
        let start_offset = self.offset;
        self.consume_punctuation(PunctuationKind::Hash)?;
        let name = self.consume_identifier()?;
        let value =
            if self.match_token(0, TokenKind::Punctuation(PunctuationKind::LParen)) {
                self.advance();
                let value = self.parse_expr(0)?;
                self.consume_punctuation(PunctuationKind::RParen)?;
                Some(Box::new(value))
            } else {
                None
            };

        Ok(Expr {
            kind: ExprKind::Tag { name, value },
            span: self.get_span(start_offset, self.offset - 1)?,
        })
    }
}
