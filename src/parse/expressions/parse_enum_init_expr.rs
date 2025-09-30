use crate::{
    ast::expr::{Expr, ExprKind},
    parse::{Parser, ParsingError},
    tokenize::PunctuationKind,
};

impl<'a, 'b> Parser<'a, 'b> {
    pub fn parse_enum_init_expr(&mut self) -> Result<Expr, ParsingError<'a>> {
        todo!()
    }
}
