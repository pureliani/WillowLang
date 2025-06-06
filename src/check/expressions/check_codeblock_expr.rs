use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::base_expression::BlockContents,
        checked::{
            checked_expression::{CheckedBlockContents, CheckedExpr, CheckedExprKind},
            checked_type::{CheckedType, CheckedTypeKind},
        },
        Span,
    },
    check::{
        scope::{Scope, ScopeKind},
        SemanticChecker,
    },
};

impl<'a> SemanticChecker<'a> {
    pub fn check_codeblock_expr(&mut self, block_contents: BlockContents, span: Span, scope: Rc<RefCell<Scope>>) -> CheckedExpr {
        let block_scope = scope.borrow().child(ScopeKind::CodeBlock);

        let checked_codeblock_statements = self.check_stmts(block_contents.statements, block_scope.clone());
        let checked_codeblock_final_expr = block_contents.final_expr.map(|fe| {
            let checked_final_expr = self.check_expr(*fe, block_scope.clone());

            Box::new(checked_final_expr)
        });

        let ty = checked_codeblock_final_expr.clone().map(|fe| fe.ty).unwrap_or(CheckedType {
            kind: CheckedTypeKind::Void,
            span,
        });

        CheckedExpr {
            kind: CheckedExprKind::Block(CheckedBlockContents {
                final_expr: checked_codeblock_final_expr,
                statements: checked_codeblock_statements,
            }),
            ty,
        }
    }
}
