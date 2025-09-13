use crate::{
    ast::expr::{Expr, ExprKind, MatchArm, MatchPattern},
    parse::{Parser, ParsingError},
    tokenize::{KeywordKind, PunctuationKind, TokenKind},
};

impl<'a, 'b> Parser<'a, 'b> {
    pub fn parse_match_expr(&mut self) -> Result<Expr, ParsingError<'a>> {
        let start_offset = self.offset;

        self.consume_keyword(KeywordKind::Match)?;
        let condition = self.parse_expr(0)?;
        self.consume_punctuation(PunctuationKind::LBrace)?;
        let arms = self.comma_separated(
            |p| {
                let variant_name = p.consume_identifier()?;

                let pattern = if p.match_token(0, TokenKind::Punctuation(PunctuationKind::LParen)) {
                    p.advance();
                    let binding_name = p.consume_identifier()?;
                    p.consume_punctuation(PunctuationKind::RParen)?;
                    MatchPattern::VariantWithValue(variant_name, binding_name)
                } else {
                    MatchPattern::Variant(variant_name)
                };

                p.consume_punctuation(PunctuationKind::Eq)?;
                p.consume_punctuation(PunctuationKind::Gt)?;

                let expression = p.parse_expr(0)?;

                Ok(MatchArm { expression, pattern })
            },
            |p| p.match_token(0, TokenKind::Punctuation(PunctuationKind::RBrace)),
        )?;
        self.consume_punctuation(PunctuationKind::RBrace)?;

        Ok(Expr {
            kind: ExprKind::Match {
                condition: Box::new(condition),
                arms,
            },
            span: self.get_span(start_offset, self.offset - 1)?,
        })
    }
}
