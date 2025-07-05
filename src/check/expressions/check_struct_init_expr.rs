use crate::{
    ast::{
        base::base_expression::Expr,
        checked::{
            checked_expression::CheckedExpr,
            checked_type::{CheckedType, CheckedTypeKind},
        },
        IdentifierNode, Span,
    },
    check::SemanticChecker,
};

impl<'a> SemanticChecker<'a> {
    pub fn check_struct_literal_expr(&mut self, fields: Vec<(IdentifierNode, Expr)>, span: Span) -> CheckedExpr {
        let checked_args: Vec<(IdentifierNode, CheckedExpr)> = fields
            .into_iter()
            .map(|(ident, expr)| (ident, self.check_expr(expr)))
            .collect();

        let mut result_struct_type = CheckedType {
            kind: CheckedTypeKind::Unknown,
            span,
        };

        todo!()
    }
}
