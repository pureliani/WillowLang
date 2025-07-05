use std::{cell::RefCell, rc::Rc};

use crate::ast::{IdentifierNode, Span, StringNode};

use super::{
    checked_declaration::{CheckedTypeAliasDecl, CheckedVarDecl},
    checked_expression::{CheckedBlockContents, CheckedExpr},
};

#[derive(Clone, Debug)]
pub enum CheckedStmt {
    Expression(CheckedExpr),
    TypeAliasDecl(Rc<RefCell<CheckedTypeAliasDecl>>),
    VarDecl(Rc<RefCell<CheckedVarDecl>>),
    Break {
        span: Span,
    },
    Continue {
        span: Span,
    },
    Return(CheckedExpr),
    Assignment {
        target: CheckedExpr,
        value: CheckedExpr,
    },
    From {
        path: StringNode,
        identifiers: Vec<(IdentifierNode, Option<IdentifierNode>)>, // optional alias
        span: Span,
    },
    While {
        condition: Box<CheckedExpr>,
        body: CheckedBlockContents,
        span: Span,
    },
}
