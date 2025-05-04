use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::base_expression::{BlockContents, Expr},
        checked::{
            checked_expression::{CheckedBlockContents, CheckedExpr, CheckedExprKind},
            checked_type::{Type, TypeKind, TypeSpan},
        },
        Span,
    },
    check::{
        check_expr::check_expr,
        check_stmts::check_stmts,
        scope::{Scope, ScopeKind},
        utils::union_of::union_of,
        SemanticError, SemanticErrorKind,
    },
};

pub fn check_if_expr(
    condition: Box<Expr>,
    then_branch: BlockContents,
    else_if_branches: Vec<(Box<Expr>, BlockContents)>,
    else_branch: Option<BlockContents>,
    expr_span: Span,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    let mut if_else_expr_type = Type {
        kind: TypeKind::Void,
        span: TypeSpan::Expr(expr_span),
    };

    let checked_condition = check_expr(*condition, errors, scope.clone());
    if checked_condition.expr_type.kind != TypeKind::Bool {
        errors.push(SemanticError::new(
            SemanticErrorKind::TypeMismatch {
                expected: Type {
                    kind: TypeKind::Bool,
                    span: checked_condition.expr_type.span,
                },
                received: checked_condition.expr_type.clone(),
            },
            checked_condition.expr_type.unwrap_expr_span(),
        ));
    }
    let then_branch_scope = scope.borrow().child(ScopeKind::CodeBlock);
    let checked_then_branch_statements =
        check_stmts(then_branch.statements, errors, then_branch_scope.clone());

    let checked_then_branch_final_expr = then_branch.final_expr.map(|fe| {
        let checked_final_expr = check_expr(*fe, errors, then_branch_scope.clone());

        if_else_expr_type = union_of(&[
            if_else_expr_type.clone(),
            checked_final_expr.expr_type.clone(),
        ]);

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

            let else_if_scope = scope.borrow().child(ScopeKind::CodeBlock);
            let checked_codeblock_statements =
                check_stmts(ei.1.statements, errors, else_if_scope.clone());
            let checked_codeblock_final_expr = ei.1.final_expr.map(|fe| {
                let checked_final_expr = check_expr(*fe, errors, else_if_scope.clone());

                if_else_expr_type = union_of(&[
                    if_else_expr_type.clone(),
                    checked_final_expr.expr_type.clone(),
                ]);

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

            if_else_expr_type = union_of(&[
                if_else_expr_type.clone(),
                checked_final_expr.expr_type.clone(),
            ]);

            Box::new(checked_final_expr)
        });

        CheckedBlockContents {
            statements: checked_statements,
            final_expr: checked_final_expr,
        }
    });

    CheckedExpr {
        expr_type: if_else_expr_type,
        kind: CheckedExprKind::If {
            condition: Box::new(checked_condition),
            then_branch: checked_then_branch,
            else_if_branches: checked_else_if_branches,
            else_branch: checked_else_branch,
        },
    }
}
