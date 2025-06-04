use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::base_expression::BlockContents,
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
        SemanticError,
    },
    compile::SpanRegistry,
};
impl<'a> SemanticChecker<'a> {}

pub fn check_codeblock_expr(
    block_contents: BlockContents,
    span: Span,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    let block_scope = scope.borrow().child(ScopeKind::CodeBlock);

    let checked_codeblock_statements = check_stmts(
        block_contents.statements,
        errors,
        block_scope.clone(),
        span_registry,
    );
    let checked_codeblock_final_expr = block_contents.final_expr.map(|fe| {
        let checked_final_expr = check_expr(*fe, errors, block_scope.clone(), span_registry);

        Box::new(checked_final_expr)
    });

    let ty = checked_codeblock_final_expr
        .clone()
        .map(|fe| fe.ty)
        .unwrap_or(CheckedType::Void);

    CheckedExpr {
        span,
        kind: CheckedExprKind::Block(CheckedBlockContents {
            final_expr: checked_codeblock_final_expr,
            statements: checked_codeblock_statements,
        }),
        ty,
    }
}
