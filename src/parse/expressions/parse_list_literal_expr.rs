use crate::{
    ast::expr::{Expr, ExprKind},
    parse::{Parser, ParsingError},
    tokenize::{PunctuationKind, TokenKind},
};

impl Parser {
    pub fn parse_list_literal_expr(&mut self) -> Result<Expr, ParsingError> {
        let start_offset = self.offset;
        self.consume_punctuation(PunctuationKind::LBracket)?;
        let items: Vec<Expr> = self.comma_separated(
            |p| p.parse_expr(0),
            |p| p.match_token(0, TokenKind::Punctuation(PunctuationKind::RBracket)),
        )?;
        self.consume_punctuation(PunctuationKind::RBracket)?;

        let span = self.get_span(start_offset, self.offset - 1)?;

        Ok(Expr {
            kind: ExprKind::List(items),
            span,
        })
    }
}
