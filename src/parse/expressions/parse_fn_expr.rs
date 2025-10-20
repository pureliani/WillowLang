use crate::{
    ast::{
        decl::Param,
        expr::{Expr, ExprKind},
    },
    parse::{Parser, ParsingError},
    tokenize::{KeywordKind, PunctuationKind, TokenKind},
};

impl Parser {
    pub fn parse_fn_expr(&mut self) -> Result<Expr, ParsingError> {
        let start_offset = self.offset;

        self.consume_keyword(KeywordKind::Fn)?;
        let identifier = self.consume_identifier()?;
        self.consume_punctuation(PunctuationKind::LParen)?;
        let params = self.comma_separated(
            |p| {
                let identifier = p.consume_identifier()?;
                p.consume_punctuation(PunctuationKind::Col)?;
                let constraint = p.parse_type_annotation(0)?;

                Ok(Param {
                    constraint,
                    identifier,
                })
            },
            |p| p.match_token(0, TokenKind::Punctuation(PunctuationKind::RParen)),
        )?;
        self.consume_punctuation(PunctuationKind::RParen)?;
        self.consume_punctuation(PunctuationKind::Col)?;
        let return_type = self.parse_type_annotation(0)?;

        let body = self.parse_codeblock_expr()?;

        Ok(Expr {
            kind: ExprKind::Fn {
                identifier,
                params,
                return_type,
                body,
            },
            span: self.get_span(start_offset, self.offset - 1)?,
        })
    }
}
