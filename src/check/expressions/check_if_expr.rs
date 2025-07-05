use crate::{
    ast::{
        base::base_expression::{BlockContents, Expr},
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{CheckedType, CheckedTypeKind},
        },
        Span,
    },
    check::{utils::union_of::union_of, SemanticChecker},
};

impl<'a> SemanticChecker<'a> {
    pub fn check_if_expr(
        &mut self,
        condition: Box<Expr>,
        then_branch: BlockContents,
        else_if_branches: Vec<(Box<Expr>, BlockContents)>,
        else_branch: Option<BlockContents>,
        span: Span,
    ) -> CheckedExpr {
        todo!()
    }
}
