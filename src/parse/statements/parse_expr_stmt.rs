use crate::{
    ast::base::{
        base_expression::Expr,
        base_statement::{Stmt, StmtKind},
    },
    parse::{Parser, ParsingError},
};

impl Parser {
    pub fn parse_expr_stmt(&mut self, lhs: Expr) -> Result<Stmt, ParsingError> {
        Ok(Stmt {
            span: lhs.span,
            kind: StmtKind::Expression(lhs),
        })
    }
}
