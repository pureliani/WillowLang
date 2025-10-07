use crate::{
    ast::expr::{Expr, ExprKind, MatchArm, MatchPattern},
    parse::{Parser, ParsingError},
    tokenize::{KeywordKind, PunctuationKind, TokenKind},
};

impl<'a, 'b> Parser<'a, 'b> {
    pub fn parse_match_expr(&mut self) -> Result<Expr, ParsingError<'a>> {
        let start_offset = self.offset;

        self.consume_keyword(KeywordKind::Match)?;
        let conditions = self.comma_separated(
            |p| p.parse_expr(0),
            |p| p.match_token(0, TokenKind::Punctuation(PunctuationKind::LBrace)),
        )?;
        self.consume_punctuation(PunctuationKind::LBrace)?;

        let arms = self.comma_separated(
            |p| {
                let patterns = p.comma_separated(
                    |pattern_parser| {
                        let variant_name = pattern_parser.consume_identifier()?;
                        if pattern_parser.match_token(
                            0,
                            TokenKind::Punctuation(PunctuationKind::LParen),
                        ) {
                            pattern_parser.advance();
                            let binding_name = pattern_parser.consume_identifier()?;
                            pattern_parser
                                .consume_punctuation(PunctuationKind::RParen)?;
                            Ok(MatchPattern::VariantWithValue(variant_name, binding_name))
                        } else {
                            Ok(MatchPattern::Variant(variant_name))
                        }
                    },
                    |pattern_parser| {
                        pattern_parser
                            .match_token(0, TokenKind::Punctuation(PunctuationKind::Eq))
                    },
                )?;

                p.consume_punctuation(PunctuationKind::Eq)?;
                p.consume_punctuation(PunctuationKind::Gt)?;

                let expression = p.parse_expr(0)?;

                Ok(MatchArm {
                    expression,
                    pattern: patterns,
                })
            },
            |p| p.match_token(0, TokenKind::Punctuation(PunctuationKind::RBrace)),
        )?;
        self.consume_punctuation(PunctuationKind::RBrace)?;

        Ok(Expr {
            kind: ExprKind::Match { conditions, arms },
            span: self.get_span(start_offset, self.offset - 1)?,
        })
    }
}
