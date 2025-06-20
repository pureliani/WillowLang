use crate::{
    ast::{
        base::base_expression::{BlockContents, Expr},
        checked::{
            checked_expression::{CheckedBlockContents, CheckedExpr, CheckedExprKind},
            checked_type::{CheckedType, CheckedTypeKind},
        },
        Span,
    },
    check::{
        utils::{scope::ScopeKind, union_of::union_of},
        SemanticChecker, SemanticError,
    },
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
        let mut expr_type = CheckedType {
            kind: CheckedTypeKind::Void,
            span,
        };

        let checked_condition = self.check_expr(*condition);
        let expected = CheckedType {
            kind: CheckedTypeKind::Bool,
            span: checked_condition.ty.span,
        };

        if !self.check_is_assignable(&checked_condition.ty, &expected) {
            self.errors.push(SemanticError::TypeMismatch {
                expected,
                received: checked_condition.ty.clone(),
            });

            expr_type.kind = CheckedTypeKind::Unknown;
        }

        self.enter_scope(ScopeKind::CodeBlock);
        let checked_then_branch_statements = self.check_stmts(then_branch.statements);
        let checked_then_branch_final_expr = then_branch.final_expr.map(|fe| {
            let checked_final_expr = self.check_expr(*fe);

            expr_type = union_of([expr_type.clone(), checked_final_expr.ty.clone()], checked_final_expr.ty.span);

            Box::new(checked_final_expr)
        });
        self.exit_scope();

        let checked_then_branch = CheckedBlockContents {
            final_expr: checked_then_branch_final_expr,
            statements: checked_then_branch_statements,
        };

        let checked_else_if_branches: Vec<(Box<CheckedExpr>, CheckedBlockContents)> = else_if_branches
            .into_iter()
            .map(|(condition, codeblock)| {
                let checked_condition = self.check_expr(*condition);
                let expected = CheckedType {
                    kind: CheckedTypeKind::Bool,
                    span: checked_condition.ty.span,
                };
                if !self.check_is_assignable(&checked_condition.ty, &expected) {
                    self.errors.push(SemanticError::TypeMismatch {
                        expected,
                        received: checked_condition.ty.clone(),
                    });

                    expr_type.kind = CheckedTypeKind::Unknown;
                }

                self.enter_scope(ScopeKind::CodeBlock);
                let checked_codeblock_statements = self.check_stmts(codeblock.statements);
                let checked_codeblock_final_expr = codeblock.final_expr.map(|fe| {
                    let checked_final_expr = self.check_expr(*fe);

                    expr_type = union_of([expr_type.clone(), checked_final_expr.ty.clone()], checked_final_expr.ty.span);

                    Box::new(checked_final_expr)
                });
                self.exit_scope();

                (
                    Box::new(checked_condition),
                    CheckedBlockContents {
                        final_expr: checked_codeblock_final_expr,
                        statements: checked_codeblock_statements,
                    },
                )
            })
            .collect();

        let checked_else_branch = else_branch.map(|br| {
            self.enter_scope(ScopeKind::CodeBlock);
            let checked_statements = self.check_stmts(br.statements);
            let checked_final_expr = br.final_expr.map(|fe| {
                let checked_final_expr = self.check_expr(*fe);

                expr_type = union_of([expr_type.clone(), checked_final_expr.ty.clone()], checked_final_expr.ty.span);

                Box::new(checked_final_expr)
            });
            self.exit_scope();

            CheckedBlockContents {
                statements: checked_statements,
                final_expr: checked_final_expr,
            }
        });

        CheckedExpr {
            ty: expr_type,
            kind: CheckedExprKind::If {
                condition: Box::new(checked_condition),
                then_branch: checked_then_branch,
                else_if_branches: checked_else_if_branches,
                else_branch: checked_else_branch,
            },
        }
    }
}
