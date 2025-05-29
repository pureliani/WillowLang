use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::base_expression::{BlockContents, Expr},
        checked::{
            checked_expression::{CheckedBlockContents, CheckedExpr, CheckedExprKind},
            checked_type::CheckedType,
        },
        Span,
    },
    check::{
        check_expr::check_expr,
        check_stmts::check_stmts,
        scope::{Scope, ScopeKind},
        utils::{check_is_assignable::check_is_assignable, union_of::union_of},
        SemanticError, SemanticErrorKind,
    },
};

pub fn check_if_expr(
    condition: Box<Expr>,
    then_branch: BlockContents,
    else_if_branches: Vec<(Box<Expr>, BlockContents)>,
    else_branch: Option<BlockContents>,
    span: Span,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    let mut if_else_expr_type = CheckedType::Void;

    let checked_condition = check_expr(*condition, errors, scope.clone());
    if !check_is_assignable(&checked_condition.ty, &CheckedType::Bool) {
        errors.push(SemanticError {
            kind: SemanticErrorKind::TypeMismatch {
                expected: CheckedType::Bool,
                received: checked_condition.ty.clone(),
            },
            span: checked_condition.span,
        });
    }

    let then_branch_scope = scope.borrow().child(ScopeKind::CodeBlock);
    let checked_then_branch_statements =
        check_stmts(then_branch.statements, errors, then_branch_scope.clone());

    let checked_then_branch_final_expr = then_branch.final_expr.map(|fe| {
        let checked_final_expr = check_expr(*fe, errors, then_branch_scope.clone());

        if_else_expr_type = union_of([if_else_expr_type.clone(), checked_final_expr.ty.clone()]);

        Box::new(checked_final_expr)
    });

    let checked_then_branch = CheckedBlockContents {
        final_expr: checked_then_branch_final_expr,
        statements: checked_then_branch_statements,
    };

    let checked_else_if_branches: Vec<(Box<CheckedExpr>, CheckedBlockContents)> = else_if_branches
        .into_iter()
        .map(|ei| {
            let checked_condition = check_expr(*ei.0, errors, scope.clone());
            if !check_is_assignable(&checked_condition.ty, &CheckedType::Bool) {
                errors.push(SemanticError {
                    kind: SemanticErrorKind::TypeMismatch {
                        expected: CheckedType::Bool,
                        received: checked_condition.ty.clone(),
                    },
                    span: checked_condition.span,
                });
            }

            let else_if_scope = scope.borrow().child(ScopeKind::CodeBlock);
            let checked_codeblock_statements =
                check_stmts(ei.1.statements, errors, else_if_scope.clone());
            let checked_codeblock_final_expr = ei.1.final_expr.map(|fe| {
                let checked_final_expr = check_expr(*fe, errors, else_if_scope.clone());

                if_else_expr_type =
                    union_of([if_else_expr_type.clone(), checked_final_expr.ty.clone()]);

                Box::new(checked_final_expr)
            });

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
        let else_scope = scope.borrow().child(ScopeKind::CodeBlock);
        let checked_statements = check_stmts(br.statements, errors, else_scope.clone());
        let checked_final_expr = br.final_expr.map(|fe| {
            let checked_final_expr = check_expr(*fe, errors, else_scope);

            if_else_expr_type =
                union_of([if_else_expr_type.clone(), checked_final_expr.ty.clone()]);

            Box::new(checked_final_expr)
        });

        CheckedBlockContents {
            statements: checked_statements,
            final_expr: checked_final_expr,
        }
    });

    CheckedExpr {
        ty: if_else_expr_type,
        span,
        kind: CheckedExprKind::If {
            condition: Box::new(checked_condition),
            then_branch: checked_then_branch,
            else_if_branches: checked_else_if_branches,
            else_branch: checked_else_branch,
        },
    }
}
