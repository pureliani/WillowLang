use crate::{
    ast::expr::{Expr, ExprKind},
    parse::{Parser, ParsingError},
    tokenize::{KeywordKind, PunctuationKind},
};

impl Parser {
    pub fn parse_ptr_expr(&mut self, is_mutable: bool) -> Result<Expr, ParsingError> {
        let start_offset = self.offset;

        if is_mutable {
            self.consume_keyword(KeywordKind::Mut)?;
        } else {
            self.consume_keyword(KeywordKind::Ref)?;
        }

        self.consume_punctuation(PunctuationKind::LParen)?;
        let inner_expr = self.parse_expr(0)?;
        self.consume_punctuation(PunctuationKind::RParen)?;

        let span = self.get_span(start_offset, self.offset - 1)?;

        let kind = if is_mutable {
            ExprKind::Mut(Box::new(inner_expr))
        } else {
            ExprKind::Ref(Box::new(inner_expr))
        };

        Ok(Expr { kind, span })
    }
}
