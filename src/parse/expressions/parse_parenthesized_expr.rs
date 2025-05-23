use crate::{
    ast::base::base_expression::Expr,
    parse::{Parser, ParsingError},
    tokenizer::PunctuationKind,
};

impl Parser {
    pub fn parse_parenthesized_expr(&mut self) -> Result<Expr, ParsingError> {
        let start_offset = self.offset;

        self.consume_punctuation(PunctuationKind::LParen)?;
        let expr = self.parse_expr(0)?;
        self.consume_punctuation(PunctuationKind::RParen)?;

        let span = self.get_span(start_offset, self.offset - 1)?;

        Ok(Expr {
            kind: expr.kind,
            span,
        })
    }
}
