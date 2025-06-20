use crate::{
    ast::checked::{
        checked_expression::{CheckedExpr, CheckedExprKind},
        checked_statement::CheckedStmt,
    },
    check::{SemanticChecker, SemanticError},
};

impl<'a> SemanticChecker<'a> {
    pub fn check_returns(&mut self, statements: &[CheckedStmt]) -> Vec<CheckedExpr> {
        let mut returns: Vec<CheckedExpr> = vec![];

        let stmt_count = statements.len();

        for (i, stmt) in statements.iter().enumerate() {
            match &stmt {
                CheckedStmt::Return(expr) => {
                    if i < stmt_count - 1 {
                        self.errors.push(SemanticError::ReturnNotLastStatement { span: expr.ty.span });
                    }
                    returns.push(expr.clone());
                }
                CheckedStmt::While { body, .. } => {
                    returns.extend(self.check_returns(&body.statements));
                }
                CheckedStmt::Expression(expr) => {
                    if let CheckedExprKind::If {
                        then_branch,
                        else_if_branches,
                        else_branch,
                        ..
                    } = &expr.kind
                    {
                        returns.extend(self.check_returns(&then_branch.statements));

                        for (_, block) in else_if_branches {
                            returns.extend(self.check_returns(&block.statements));
                        }
                        if let Some(else_block) = else_branch {
                            returns.extend(self.check_returns(&else_block.statements));
                        }
                    } else if let CheckedExprKind::Block(block) = &expr.kind {
                        returns.extend(self.check_returns(&block.statements));
                        if let Some(final_expr) = &block.final_expr {
                            returns.push(*final_expr.clone());
                        }
                    }
                }
                _ => (),
            }
        }

        returns
    }
}
