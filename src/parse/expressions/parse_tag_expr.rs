use crate::{
    ast::expr::{Expr, ExprKind},
    parse::{Parser, ParsingError},
    tokenize::PunctuationKind,
};

impl<'a, 'b> Parser<'a, 'b> {
    pub fn parse_tag_expr(&mut self) -> Result<Expr, ParsingError<'a>> {
        let start_offset = self.offset;
        self.consume_punctuation(PunctuationKind::Hashtag)?;
        let identifier = self.consume_identifier()?;
        let value = if self.match_token(0, crate::tokenize::TokenKind::Punctuation(PunctuationKind::LParen)) {
            self.consume_punctuation(PunctuationKind::LParen)?;
            let value_type = self.parse_expr(0)?;
            self.consume_punctuation(PunctuationKind::RParen)?;
            Some(Box::new(value_type))
        } else {
            None
        };
        let span = self.get_span(start_offset, self.offset - 1)?;

        Ok(Expr {
            kind: ExprKind::Tag { identifier, value },
            span,
        })
    }
}
