use crate::{
    ast::{
        expr::Expr,
        stmt::{Stmt, StmtKind},
    },
    parse::{Parser, ParsingError},
};

impl<'a, 'b> Parser<'a, 'b> {
    pub fn parse_expr_stmt(&mut self, lhs: Expr) -> Result<Stmt, ParsingError<'a>> {
        Ok(Stmt {
            span: lhs.span,
            kind: StmtKind::Expression(lhs),
        })
    }
}
