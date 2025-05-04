use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::base_expression::BlockContents,
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
        SemanticError,
    },
};

pub fn check_codeblock_expr(
    block_contents: BlockContents,
    expr_span: Span,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    let block_scope = scope.borrow().child(ScopeKind::CodeBlock);

    let checked_codeblock_statements =
        check_stmts(block_contents.statements, errors, block_scope.clone());
    let checked_codeblock_final_expr = block_contents.final_expr.map(|fe| {
        let checked_final_expr = check_expr(*fe, errors, block_scope.clone());

        Box::new(checked_final_expr)
    });

    let expr_type = checked_codeblock_final_expr
        .clone()
        .map(|fe| fe.expr_type)
        .unwrap_or(Type {
            kind: TypeKind::Void,
            span: TypeSpan::Expr(expr_span),
        });

    CheckedExpr {
        kind: CheckedExprKind::Block(CheckedBlockContents {
            final_expr: checked_codeblock_final_expr,
            statements: checked_codeblock_statements,
        }),
        expr_type,
    }
}
