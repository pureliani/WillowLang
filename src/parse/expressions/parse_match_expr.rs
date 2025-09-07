use crate::{
    ast::expr::{Expr, ExprKind, MatchArm},
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

                let binding_name = if p.match_token(0, TokenKind::Punctuation(PunctuationKind::LParen)) {
                    p.advance();
                    let id = p.consume_identifier()?;
                    p.consume_punctuation(PunctuationKind::RParen)?;
                    Some(id)
                } else {
                    None
                };

                p.consume_punctuation(PunctuationKind::Eq)?;
                p.consume_punctuation(PunctuationKind::Gt)?;

                let expr = p.parse_expr(0)?;

                Ok(MatchArm {
                    variant_name,
                    binding_name,
                    evaluate: expr,
                })
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
