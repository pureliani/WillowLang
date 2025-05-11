Project Path: src

Source Tree:

```txt
src
├── ast
│   ├── base
│   │   ├── base_declaration.rs
│   │   ├── base_expression.rs
│   │   ├── base_statement.rs
│   │   ├── base_type.rs
│   │   └── mod.rs
│   ├── checked
│   │   ├── checked_declaration.rs
│   │   ├── checked_expression.rs
│   │   ├── checked_statement.rs
│   │   ├── checked_type.rs
│   │   └── mod.rs
│   ├── mod.rs
│   └── monomorphized
│       ├── mod.rs
│       ├── monomorphized_declaration.rs
│       ├── monomorphized_expression.rs
│       ├── monomorphized_statement.rs
│       └── monomorphized_type.rs
├── check
│   ├── check_expr.rs
│   ├── check_stmt.rs
│   ├── check_stmts.rs
│   ├── expressions
│   │   ├── check_access_expr.rs
│   │   ├── check_addition_expr.rs
│   │   ├── check_and_expr.rs
│   │   ├── check_arithmetic_negation_expr.rs
│   │   ├── check_array_literal_expr.rs
│   │   ├── check_bool_expr.rs
│   │   ├── check_codeblock_expr.rs
│   │   ├── check_division_expr.rs
│   │   ├── check_equality_expr.rs
│   │   ├── check_fn_call_expr.rs
│   │   ├── check_fn_expr.rs
│   │   ├── check_generic_apply_expr.rs
│   │   ├── check_greater_than_expr.rs
│   │   ├── check_greater_than_or_equal_expr.rs
│   │   ├── check_identifier_expr.rs
│   │   ├── check_if_expr.rs
│   │   ├── check_inequality_expr.rs
│   │   ├── check_is_type_expr.rs
│   │   ├── check_less_than_expr.rs
│   │   ├── check_less_than_or_equal_expr.rs
│   │   ├── check_logical_negation_expr.rs
│   │   ├── check_modulo_expr.rs
│   │   ├── check_multiplication_expr.rs
│   │   ├── check_null_expr.rs
│   │   ├── check_numeric_expr.rs
│   │   ├── check_or_expr.rs
│   │   ├── check_static_access_expr.rs
│   │   ├── check_string_expr.rs
│   │   ├── check_struct_init_expr.rs
│   │   ├── check_subtraction_expr.rs
│   │   ├── check_type_cast_expr.rs
│   │   └── mod.rs
│   ├── mod.rs
│   ├── scope.rs
│   ├── type_flow_graph.rs
│   └── utils
│       ├── check_binary_numeric_operation.rs
│       ├── check_is_assignable.rs
│       ├── check_returns.rs
│       ├── get_numeric_type_rank.rs
│       ├── infer_generics.rs
│       ├── is_float.rs
│       ├── is_integer.rs
│       ├── is_signed.rs
│       ├── mod.rs
│       ├── substitute_generics.rs
│       ├── type_annotation_to_semantic.rs
│       └── union_of.rs
├── codegen
│   └── mod.rs
├── compile
│   └── mod.rs
├── lib.rs
├── main.rs
├── parse
│   ├── expressions
│   │   ├── mod.rs
│   │   ├── parse_codeblock_expr.rs
│   │   ├── parse_fn_call_expr.rs
│   │   ├── parse_fn_expr.rs
│   │   ├── parse_if_expr.rs
│   │   ├── parse_parenthesized_expr.rs
│   │   └── parse_struct_init_expr.rs
│   ├── mod.rs
│   ├── parse_generic_args.rs
│   ├── parse_generic_params.rs
│   ├── statements
│   │   ├── mod.rs
│   │   ├── parse_assignment_stmt.rs
│   │   ├── parse_break_stmt.rs
│   │   ├── parse_continue_stmt.rs
│   │   ├── parse_enum_decl.rs
│   │   ├── parse_expr_stmt.rs
│   │   ├── parse_from_stmt.rs
│   │   ├── parse_return_stmt.rs
│   │   ├── parse_struct_decl.rs
│   │   ├── parse_type_alias_decl.rs
│   │   ├── parse_var_decl.rs
│   │   └── parse_while_stmt.rs
│   └── type_annotations
│       ├── mod.rs
│       ├── parse_fn_type_annotation.rs
│       └── parse_parenthesized_type_annotation.rs
└── tokenizer
    ├── mod.rs
    ├── tokenize_documentation.rs
    ├── tokenize_identifier.rs
    ├── tokenize_number.rs
    ├── tokenize_punctuation.rs
    └── tokenize_string.rs

```

`src/ast/base/base_declaration.rs`:

```rs
use std::hash::{Hash, Hasher};

use crate::{ast::IdentifierNode, parse::DocAnnotation};

use super::{base_expression::Expr, base_type::TypeAnnotation};

#[derive(Clone, Debug, PartialEq)]
pub struct Param {
    pub identifier: IdentifierNode,
    pub constraint: TypeAnnotation,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GenericParam {
    pub identifier: IdentifierNode,
    pub constraint: Option<TypeAnnotation>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StructDecl {
    pub identifier: IdentifierNode,
    pub documentation: Option<DocAnnotation>,
    pub generic_params: Vec<GenericParam>,
    pub properties: Vec<Param>,
}

#[derive(Clone, Debug)]
pub struct EnumDecl {
    pub identifier: IdentifierNode,
    pub documentation: Option<DocAnnotation>,
    pub variants: Vec<IdentifierNode>,
}

impl Eq for EnumDecl {}
impl PartialEq for EnumDecl {
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier && self.variants == other.variants
    }
}
impl Hash for EnumDecl {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.identifier.hash(state);
        self.variants.hash(state);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeAliasDecl {
    pub identifier: IdentifierNode,
    pub documentation: Option<DocAnnotation>,
    pub generic_params: Vec<GenericParam>,
    pub value: TypeAnnotation,
}

#[derive(Clone, Debug, PartialEq)]
pub struct VarDecl {
    pub identifier: IdentifierNode,
    pub documentation: Option<DocAnnotation>,
    pub constraint: Option<TypeAnnotation>,
    pub value: Option<Expr>,
}

```

`src/ast/base/base_expression.rs`:

```rs
use crate::{
    ast::{IdentifierNode, Span, StringNode},
    parse::ParsingError,
    tokenizer::NumberKind,
};

use super::{
    base_declaration::{GenericParam, Param},
    base_statement::Stmt,
    base_type::TypeAnnotation,
};

#[derive(Clone, Debug, PartialEq)]
pub struct BlockContents {
    pub statements: Vec<Stmt>,
    pub final_expr: Option<Box<Expr>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExprKind {
    // Prefix expressions
    Not {
        right: Box<Expr>,
    },
    Neg {
        right: Box<Expr>,
    },
    // Infix expressions
    Add {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Subtract {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Multiply {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Divide {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Modulo {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    LessThan {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    LessThanOrEqual {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    GreaterThan {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    GreaterThanOrEqual {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Equal {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    NotEqual {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    And {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Or {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    // Suffix expressions
    Access {
        left: Box<Expr>,
        field: IdentifierNode,
    },
    StaticAccess {
        left: Box<Expr>,
        field: IdentifierNode,
    },
    TypeCast {
        left: Box<Expr>,
        target: TypeAnnotation,
    },
    IsType {
        left: Box<Expr>,
        target: TypeAnnotation,
    },
    GenericApply {
        left: Box<Expr>,
        args: Vec<TypeAnnotation>,
    },
    FnCall {
        left: Box<Expr>,
        args: Vec<Expr>,
    },
    StructInit {
        left: Box<Expr>,
        fields: Vec<(IdentifierNode, Expr)>,
    },
    // Basic/literal expressions
    Null,
    BoolLiteral {
        value: bool,
    },
    Number {
        value: NumberKind,
    },
    String(StringNode),
    Identifier(IdentifierNode),
    // Complex expressions
    Fn {
        params: Vec<Param>,
        body: BlockContents,
        return_type: Option<TypeAnnotation>,
        generic_params: Vec<GenericParam>,
    },
    If {
        condition: Box<Expr>,
        then_branch: BlockContents,
        else_if_branches: Vec<(Box<Expr>, BlockContents)>,
        else_branch: Option<BlockContents>,
    },
    ArrayLiteral {
        items: Vec<Expr>,
    },
    Block(BlockContents),
    Error(ParsingError),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

```

`src/ast/base/base_statement.rs`:

```rs
use crate::{
    ast::{IdentifierNode, Span, StringNode},
    parse::ParsingError,
};

use super::{
    base_declaration::{EnumDecl, StructDecl, TypeAliasDecl, VarDecl},
    base_expression::{BlockContents, Expr},
};

#[derive(Clone, Debug, PartialEq)]
pub enum StmtKind {
    Expression(Expr),
    StructDecl(StructDecl),
    EnumDecl(EnumDecl),
    TypeAliasDecl(TypeAliasDecl),
    VarDecl(VarDecl),
    Break,
    Continue,
    Return(Expr),
    Assignment {
        target: Expr,
        value: Expr,
    },
    From {
        path: StringNode,
        identifiers: Vec<(IdentifierNode, Option<IdentifierNode>)>, // optional alias
    },
    While {
        condition: Box<Expr>,
        body: BlockContents,
    },
    Error(ParsingError),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: Span,
}

```

`src/ast/base/base_type.rs`:

```rs
use crate::{
    ast::{IdentifierNode, Span},
    parse::ParsingError,
    tokenizer::NumberKind,
};

use super::base_declaration::{GenericParam, Param};

#[derive(Clone, Debug, PartialEq)]
pub enum TypeAnnotationKind {
    Void,
    Null,
    Bool,
    U8,
    U16,
    U32,
    U64,
    USize,
    ISize,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    Char,
    Identifier(IdentifierNode),
    GenericFnType {
        params: Vec<Param>,
        return_type: Box<TypeAnnotation>,
        generic_params: Vec<GenericParam>,
    },
    FnType {
        params: Vec<Param>,
        return_type: Box<TypeAnnotation>,
    },
    // Infix types
    Union(Vec<TypeAnnotation>),
    // Suffix types
    Array {
        left: Box<TypeAnnotation>,
        size: NumberKind,
    },
    GenericApply {
        left: Box<TypeAnnotation>,
        args: Vec<TypeAnnotation>,
    },
    Error(ParsingError),
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeAnnotation {
    pub kind: TypeAnnotationKind,
    pub span: Span,
}

```

`src/ast/base/mod.rs`:

```rs
pub mod base_declaration;
pub mod base_expression;
pub mod base_statement;
pub mod base_type;

```

`src/ast/checked/checked_declaration.rs`:

```rs
use std::hash::{Hash, Hasher};

use crate::{ast::IdentifierNode, parse::DocAnnotation};

use super::{checked_expression::CheckedExpr, checked_type::CheckedType};

#[derive(Clone, Debug)]
pub struct CheckedParam {
    pub identifier: IdentifierNode,
    pub constraint: CheckedType,
}

impl Eq for CheckedParam {}
impl PartialEq for CheckedParam {
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier && self.constraint == other.constraint
    }
}
impl Hash for CheckedParam {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.identifier.hash(state);
        self.constraint.hash(state);
    }
}

#[derive(Clone, Debug)]
pub struct CheckedGenericParam {
    pub identifier: IdentifierNode,
    pub constraint: Option<Box<CheckedType>>,
}

impl Eq for CheckedGenericParam {}
impl PartialEq for CheckedGenericParam {
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier && self.constraint == other.constraint
    }
}
impl Hash for CheckedGenericParam {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.identifier.hash(state);
        self.constraint.hash(state);
    }
}

#[derive(Clone, Debug)]
pub struct GenericStructDecl {
    pub identifier: IdentifierNode,
    pub documentation: Option<DocAnnotation>,
    pub generic_params: Vec<CheckedGenericParam>,
    pub properties: Vec<CheckedParam>,
}

impl Eq for GenericStructDecl {}
impl PartialEq for GenericStructDecl {
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier
            && self.generic_params == other.generic_params
            && self.properties == other.properties
    }
}
impl Hash for GenericStructDecl {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.identifier.hash(state);
        self.generic_params.hash(state);
        self.properties.hash(state);
    }
}

#[derive(Clone, Debug)]
pub struct StructDecl {
    pub identifier: IdentifierNode,
    pub documentation: Option<DocAnnotation>,
    pub properties: Vec<CheckedParam>,
}

impl Eq for StructDecl {}
impl PartialEq for StructDecl {
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier && self.properties == other.properties
    }
}
impl Hash for StructDecl {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.identifier.hash(state);
        self.properties.hash(state);
    }
}

#[derive(Clone, Debug)]
pub struct GenericTypeAliasDecl {
    pub identifier: IdentifierNode,
    pub documentation: Option<DocAnnotation>,
    pub generic_params: Vec<CheckedGenericParam>,
    pub value: Box<CheckedType>,
}

impl Eq for GenericTypeAliasDecl {}
impl PartialEq for GenericTypeAliasDecl {
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier
            && self.generic_params == other.generic_params
            && self.value == other.value
    }
}
impl Hash for GenericTypeAliasDecl {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.identifier.hash(state);
        self.generic_params.hash(state);
        self.value.hash(state);
    }
}

#[derive(Clone, Debug)]
pub struct TypeAliasDecl {
    pub identifier: IdentifierNode,
    pub documentation: Option<DocAnnotation>,
    pub value: Box<CheckedType>,
}

impl Eq for TypeAliasDecl {}
impl PartialEq for TypeAliasDecl {
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier && self.value == other.value
    }
}
impl Hash for TypeAliasDecl {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.identifier.hash(state);
        self.value.hash(state);
    }
}

#[derive(Clone, Debug)]
pub struct CheckedVarDecl {
    pub identifier: IdentifierNode,
    pub documentation: Option<DocAnnotation>,
    pub constraint: CheckedType,
    pub value: Option<CheckedExpr>,
}

```

`src/ast/checked/checked_expression.rs`:

```rs
use crate::{
    ast::{IdentifierNode, StringNode},
    tokenizer::NumberKind,
};

use super::{
    checked_declaration::{CheckedGenericParam, CheckedParam},
    checked_statement::CheckedStmt,
    checked_type::CheckedType,
};

#[derive(Clone, Debug)]
pub struct CheckedBlockContents {
    pub statements: Vec<CheckedStmt>,
    pub final_expr: Option<Box<CheckedExpr>>,
}

#[derive(Clone, Debug)]
pub enum CheckedExprKind {
    // Prefix expressions
    Not {
        right: Box<CheckedExpr>,
    },
    Neg {
        right: Box<CheckedExpr>,
    },
    // Infix expressions
    Add {
        left: Box<CheckedExpr>,
        right: Box<CheckedExpr>,
    },
    Subtract {
        left: Box<CheckedExpr>,
        right: Box<CheckedExpr>,
    },
    Multiply {
        left: Box<CheckedExpr>,
        right: Box<CheckedExpr>,
    },
    Divide {
        left: Box<CheckedExpr>,
        right: Box<CheckedExpr>,
    },
    Modulo {
        left: Box<CheckedExpr>,
        right: Box<CheckedExpr>,
    },
    LessThan {
        left: Box<CheckedExpr>,
        right: Box<CheckedExpr>,
    },
    LessThanOrEqual {
        left: Box<CheckedExpr>,
        right: Box<CheckedExpr>,
    },
    GreaterThan {
        left: Box<CheckedExpr>,
        right: Box<CheckedExpr>,
    },
    GreaterThanOrEqual {
        left: Box<CheckedExpr>,
        right: Box<CheckedExpr>,
    },
    Equal {
        left: Box<CheckedExpr>,
        right: Box<CheckedExpr>,
    },
    NotEqual {
        left: Box<CheckedExpr>,
        right: Box<CheckedExpr>,
    },
    And {
        left: Box<CheckedExpr>,
        right: Box<CheckedExpr>,
    },
    Or {
        left: Box<CheckedExpr>,
        right: Box<CheckedExpr>,
    },
    // Suffix expressions
    Access {
        left: Box<CheckedExpr>,
        field: IdentifierNode,
    },
    StaticAccess {
        left: Box<CheckedExpr>,
        field: IdentifierNode,
    },
    TypeCast {
        left: Box<CheckedExpr>,
        target: CheckedType,
    },
    IsType {
        left: Box<CheckedExpr>,
        target: CheckedType,
    },
    FnCall {
        left: Box<CheckedExpr>,
        args: Vec<CheckedExpr>,
    },
    StructInit {
        left: Box<CheckedExpr>,
        fields: Vec<(IdentifierNode, CheckedExpr)>,
    },
    // Basic/literal expressions
    Null,
    BoolLiteral {
        value: bool,
    },
    Number {
        value: NumberKind,
    },
    String(StringNode),
    Identifier(IdentifierNode),
    // Complex expressions
    GenericFn {
        params: Vec<CheckedParam>,
        body: CheckedBlockContents,
        return_type: CheckedType,
        generic_params: Vec<CheckedGenericParam>,
    },
    Fn {
        params: Vec<CheckedParam>,
        body: CheckedBlockContents,
        return_type: CheckedType,
    },
    If {
        condition: Box<CheckedExpr>,
        then_branch: CheckedBlockContents,
        else_if_branches: Vec<(Box<CheckedExpr>, CheckedBlockContents)>,
        else_branch: Option<CheckedBlockContents>,
    },
    ArrayLiteral {
        items: Vec<CheckedExpr>,
    },
    Block(CheckedBlockContents),
}

#[derive(Clone, Debug)]
pub struct CheckedExpr {
    pub kind: CheckedExprKind,
    pub expr_type: CheckedType,
}

```

`src/ast/checked/checked_statement.rs`:

```rs
use crate::ast::{base::base_declaration::EnumDecl, IdentifierNode, Span, StringNode};

use super::{
    checked_declaration::{
        CheckedVarDecl, GenericStructDecl, GenericTypeAliasDecl, StructDecl, TypeAliasDecl,
    },
    checked_expression::{CheckedBlockContents, CheckedExpr},
};

#[derive(Clone, Debug)]
pub enum CheckedStmtKind {
    Expression(CheckedExpr),
    GenericStructDecl(GenericStructDecl),
    StructDecl(StructDecl),
    EnumDecl(EnumDecl),
    GenericTypeAliasDecl(GenericTypeAliasDecl),
    TypeAliasDecl(TypeAliasDecl),
    VarDecl(CheckedVarDecl),
    Break,
    Continue,
    Return(CheckedExpr),
    Assignment {
        target: CheckedExpr,
        value: CheckedExpr,
    },
    From {
        path: StringNode,
        identifiers: Vec<(IdentifierNode, Option<IdentifierNode>)>, // optional alias
    },
    While {
        condition: Box<CheckedExpr>,
        body: CheckedBlockContents,
    },
}

#[derive(Clone, Debug)]
pub struct CheckedStmt {
    pub kind: CheckedStmtKind,
    pub span: Span,
}

```

`src/ast/checked/checked_type.rs`:

```rs
use std::{
    collections::HashSet,
    hash::{Hash, Hasher},
};

use crate::ast::{base::base_declaration::EnumDecl, Span};

use super::checked_declaration::{
    CheckedGenericParam, CheckedParam, GenericStructDecl, GenericTypeAliasDecl, StructDecl,
    TypeAliasDecl,
};

#[derive(Clone, Debug)]
pub enum CheckedTypeKind {
    Void,
    Null,
    Bool,
    U8,
    U16,
    U32,
    U64,
    USize,
    ISize,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    Char,
    GenericStructDecl(GenericStructDecl),
    StructDecl(StructDecl),
    Enum(EnumDecl),
    GenericParam(CheckedGenericParam),
    GenericFnType {
        params: Vec<CheckedParam>,
        return_type: Box<CheckedType>,
        generic_params: Vec<CheckedGenericParam>,
    },
    FnType {
        params: Vec<CheckedParam>,
        return_type: Box<CheckedType>,
    },
    GenericTypeAliasDecl(GenericTypeAliasDecl),
    TypeAliasDecl(TypeAliasDecl),
    // Infix types
    Union(HashSet<CheckedType>),
    // Suffix types
    Array {
        item_type: Box<CheckedType>,
        size: usize,
    },
    Unknown,
}

impl Eq for CheckedTypeKind {}
impl PartialEq for CheckedTypeKind {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (CheckedTypeKind::Void, CheckedTypeKind::Void) => true,
            (CheckedTypeKind::Null, CheckedTypeKind::Null) => true,
            (CheckedTypeKind::Bool, CheckedTypeKind::Bool) => true,
            (CheckedTypeKind::U8, CheckedTypeKind::U8) => true,
            (CheckedTypeKind::U16, CheckedTypeKind::U16) => true,
            (CheckedTypeKind::U32, CheckedTypeKind::U32) => true,
            (CheckedTypeKind::U64, CheckedTypeKind::U64) => true,
            (CheckedTypeKind::USize, CheckedTypeKind::USize) => true,
            (CheckedTypeKind::ISize, CheckedTypeKind::ISize) => true,
            (CheckedTypeKind::I8, CheckedTypeKind::I8) => true,
            (CheckedTypeKind::I16, CheckedTypeKind::I16) => true,
            (CheckedTypeKind::I32, CheckedTypeKind::I32) => true,
            (CheckedTypeKind::I64, CheckedTypeKind::I64) => true,
            (CheckedTypeKind::F32, CheckedTypeKind::F32) => true,
            (CheckedTypeKind::F64, CheckedTypeKind::F64) => true,
            (CheckedTypeKind::Char, CheckedTypeKind::Char) => true,
            (CheckedTypeKind::GenericStructDecl(a), CheckedTypeKind::GenericStructDecl(b)) => {
                a == b
            }
            (CheckedTypeKind::StructDecl(a), CheckedTypeKind::StructDecl(b)) => a == b,
            (CheckedTypeKind::Enum(a), CheckedTypeKind::Enum(b)) => a == b,
            (CheckedTypeKind::GenericParam(a), CheckedTypeKind::GenericParam(b)) => a == b,
            (
                CheckedTypeKind::GenericFnType {
                    params: ap,
                    return_type: ar,
                    generic_params: agp,
                },
                CheckedTypeKind::GenericFnType {
                    params: bp,
                    return_type: br,
                    generic_params: bgp,
                },
            ) => ap == bp && ar == br && agp == bgp,
            (
                CheckedTypeKind::FnType {
                    params: ap,
                    return_type: ar,
                },
                CheckedTypeKind::FnType {
                    params: bp,
                    return_type: br,
                },
            ) => ap == bp && ar == br,
            (
                CheckedTypeKind::GenericTypeAliasDecl(a),
                CheckedTypeKind::GenericTypeAliasDecl(b),
            ) => a == b,
            (CheckedTypeKind::TypeAliasDecl(a), CheckedTypeKind::TypeAliasDecl(b)) => a == b,
            (CheckedTypeKind::Union(a_items), CheckedTypeKind::Union(b_items)) => {
                if a_items.len() != b_items.len() {
                    return false;
                }
                // Order-insensitive comparison for unions
                a_items.iter().all(|item_a| b_items.contains(item_a))
                    && b_items.iter().all(|item_b| a_items.contains(item_b))
            }
            (
                CheckedTypeKind::Array {
                    item_type: ai,
                    size: asize,
                },
                CheckedTypeKind::Array {
                    item_type: bi,
                    size: bsize,
                },
            ) => ai == bi && asize == bsize,
            (CheckedTypeKind::Unknown, CheckedTypeKind::Unknown) => true,
            _ => false,
        }
    }
}

impl Hash for CheckedTypeKind {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);

        match self {
            CheckedTypeKind::Void => {}
            CheckedTypeKind::Null => {}
            CheckedTypeKind::Bool => {}
            CheckedTypeKind::U8 => {}
            CheckedTypeKind::U16 => {}
            CheckedTypeKind::U32 => {}
            CheckedTypeKind::U64 => {}
            CheckedTypeKind::USize => {}
            CheckedTypeKind::ISize => {}
            CheckedTypeKind::I8 => {}
            CheckedTypeKind::I16 => {}
            CheckedTypeKind::I32 => {}
            CheckedTypeKind::I64 => {}
            CheckedTypeKind::F32 => {}
            CheckedTypeKind::F64 => {}
            CheckedTypeKind::Char => {}
            CheckedTypeKind::GenericStructDecl(gsd) => gsd.hash(state),
            CheckedTypeKind::StructDecl(sd) => sd.hash(state),
            CheckedTypeKind::Enum(e) => e.hash(state),
            CheckedTypeKind::GenericParam(gp) => gp.hash(state),
            CheckedTypeKind::GenericFnType {
                params,
                return_type,
                generic_params,
            } => {
                params.hash(state);
                return_type.hash(state);
                generic_params.hash(state);
            }
            CheckedTypeKind::FnType {
                params,
                return_type,
            } => {
                params.hash(state);
                return_type.hash(state);
            }
            CheckedTypeKind::GenericTypeAliasDecl(gta) => gta.hash(state),
            CheckedTypeKind::TypeAliasDecl(ta) => ta.hash(state),
            CheckedTypeKind::Union(items) => {
                // For order-insensitive hashing of unions:
                // 1. Hash the length.
                // 2. Hash each item's hash XORed together (or summed, but XOR is common).
                //    This makes the order not matter.
                // A more robust way is to sort a temporary list of hashes.
                state.write_usize(items.len());
                if !items.is_empty() {
                    let mut item_hashes: Vec<u64> = items
                        .iter()
                        .map(|item| {
                            let mut item_hasher = std::collections::hash_map::DefaultHasher::new();
                            item.hash(&mut item_hasher);
                            item_hasher.finish()
                        })
                        .collect();
                    item_hashes.sort_unstable(); // Sort hashes for canonical order
                    for h in item_hashes {
                        h.hash(state);
                    }
                }
            }
            CheckedTypeKind::Array { item_type, size } => {
                item_type.hash(state);
                size.hash(state);
            }
            CheckedTypeKind::Unknown => {}
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TypeSpan {
    Expr(Span),
    Annotation(Span),
    Decl(Span),
    None,
}

#[derive(Clone, Debug)]
pub struct CheckedType {
    pub kind: CheckedTypeKind,
    pub span: TypeSpan,
}

impl Eq for CheckedType {}
impl PartialEq for CheckedType {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}
impl Hash for CheckedType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.kind.hash(state);
    }
}

impl CheckedType {
    pub fn unwrap_decl_span(&self) -> Span {
        match self.span {
            TypeSpan::Decl(s) => s,
            _ => {
                panic!(
                    "Expected the type of span to be TypeSpan::Decl on {:#?}",
                    self
                )
            }
        }
    }

    pub fn unwrap_expr_span(&self) -> Span {
        match self.span {
            TypeSpan::Expr(s) => s,
            _ => {
                panic!(
                    "Expected the type of span to be TypeSpan::Expr on {:#?}",
                    self
                )
            }
        }
    }

    pub fn unwrap_annotation_span(&self) -> Span {
        match self.span {
            TypeSpan::Annotation(s) => s,
            _ => {
                panic!(
                    "Expected the type of span to be TypeSpan::Annotation on {:#?}",
                    self
                )
            }
        }
    }
}

```

`src/ast/checked/mod.rs`:

```rs
pub mod checked_declaration;
pub mod checked_expression;
pub mod checked_statement;
pub mod checked_type;

```

`src/ast/mod.rs`:

```rs
use std::hash::{Hash, Hasher};

pub mod base;
pub mod checked;
pub mod monomorphized;

#[derive(Debug, Clone)]
pub struct IdentifierNode {
    pub name: String,
    pub span: Span,
}

impl Eq for IdentifierNode {}
impl PartialEq for IdentifierNode {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl Hash for IdentifierNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

#[derive(Debug, Clone)]
pub struct StringNode {
    pub value: String,
    pub span: Span,
}

impl Eq for StringNode {}
impl PartialEq for StringNode {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl Hash for StringNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Position {
    pub line: usize,
    pub col: usize,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

```

`src/ast/monomorphized/mod.rs`:

```rs
pub mod monomorphized_declaration;
pub mod monomorphized_expression;
pub mod monomorphized_statement;
pub mod monomorphized_type;

```

`src/ast/monomorphized/monomorphized_declaration.rs`:

```rs
use crate::ast::IdentifierNode;

use super::{monomorphized_expression::MonoExpr, monomorphized_type::MonoType};

#[derive(Clone, Debug)]
pub struct MonoParam {
    pub identifier: IdentifierNode,
    pub constraint: MonoType,
}

#[derive(Clone, Debug)]
pub struct MonoStructDecl {
    pub identifier: IdentifierNode,
    pub properties: Vec<MonoParam>,
}

#[derive(Clone, Debug)]
pub struct MonoTypeAliasDecl {
    pub identifier: IdentifierNode,
    pub value: Box<MonoType>,
}

#[derive(Clone, Debug)]
pub struct MonoVarDecl {
    pub identifier: IdentifierNode,
    pub constraint: MonoType,
    pub value: Option<MonoExpr>,
}

```

`src/ast/monomorphized/monomorphized_expression.rs`:

```rs
use crate::{
    ast::{IdentifierNode, StringNode},
    tokenizer::NumberKind,
};

use super::{
    monomorphized_declaration::MonoParam, monomorphized_statement::MonoStmt,
    monomorphized_type::MonoType,
};

#[derive(Clone, Debug)]
pub struct MonoBlockContents {
    pub statements: Vec<MonoStmt>,
    pub final_expr: Option<Box<MonoExpr>>,
}

#[derive(Clone, Debug)]
pub enum MonoExprKind {
    // Prefix expressions
    Not {
        right: Box<MonoExpr>,
    },
    Neg {
        right: Box<MonoExpr>,
    },
    // Infix expressions
    Add {
        left: Box<MonoExpr>,
        right: Box<MonoExpr>,
    },
    Subtract {
        left: Box<MonoExpr>,
        right: Box<MonoExpr>,
    },
    Multiply {
        left: Box<MonoExpr>,
        right: Box<MonoExpr>,
    },
    Divide {
        left: Box<MonoExpr>,
        right: Box<MonoExpr>,
    },
    Modulo {
        left: Box<MonoExpr>,
        right: Box<MonoExpr>,
    },
    LessThan {
        left: Box<MonoExpr>,
        right: Box<MonoExpr>,
    },
    LessThanOrEqual {
        left: Box<MonoExpr>,
        right: Box<MonoExpr>,
    },
    GreaterThan {
        left: Box<MonoExpr>,
        right: Box<MonoExpr>,
    },
    GreaterThanOrEqual {
        left: Box<MonoExpr>,
        right: Box<MonoExpr>,
    },
    Equal {
        left: Box<MonoExpr>,
        right: Box<MonoExpr>,
    },
    NotEqual {
        left: Box<MonoExpr>,
        right: Box<MonoExpr>,
    },
    And {
        left: Box<MonoExpr>,
        right: Box<MonoExpr>,
    },
    Or {
        left: Box<MonoExpr>,
        right: Box<MonoExpr>,
    },
    // Suffix expressions
    Access {
        left: Box<MonoExpr>,
        field: IdentifierNode,
    },
    StaticAccess {
        left: Box<MonoExpr>,
        field: IdentifierNode,
    },
    TypeCast {
        left: Box<MonoExpr>,
        target: MonoType,
    },
    IsType {
        left: Box<MonoExpr>,
        target: MonoType,
    },
    FnCall {
        left: Box<MonoExpr>,
        args: Vec<MonoExpr>,
    },
    StructInit {
        left: Box<MonoExpr>,
        fields: Vec<(IdentifierNode, MonoExpr)>,
    },
    // Basic/literal expressions
    Null,
    BoolLiteral {
        value: bool,
    },
    Number {
        value: NumberKind,
    },
    String(StringNode),
    Identifier(IdentifierNode),
    // Complex expressions
    Fn {
        params: Vec<MonoParam>,
        body: MonoBlockContents,
        return_type: MonoType,
    },
    If {
        condition: Box<MonoExpr>,
        then_branch: MonoBlockContents,
        else_if_branches: Vec<(Box<MonoExpr>, MonoBlockContents)>,
        else_branch: Option<MonoBlockContents>,
    },
    ArrayLiteral {
        items: Vec<MonoExpr>,
    },
    Block(MonoBlockContents),
}

#[derive(Clone, Debug)]
pub struct MonoExpr {
    pub kind: MonoExprKind,
    pub expr_type: MonoType,
}

```

`src/ast/monomorphized/monomorphized_statement.rs`:

```rs
use crate::ast::{base::base_declaration::EnumDecl, IdentifierNode, Span, StringNode};

use super::{
    monomorphized_declaration::{MonoStructDecl, MonoTypeAliasDecl, MonoVarDecl},
    monomorphized_expression::{MonoBlockContents, MonoExpr},
};

#[derive(Clone, Debug)]
pub enum MonoStmtKind {
    Expression(MonoExpr),
    StructDecl(MonoStructDecl),
    EnumDecl(EnumDecl),
    TypeAliasDecl(MonoTypeAliasDecl),
    VarDecl(MonoVarDecl),
    Break,
    Continue,
    Return(MonoExpr),
    Assignment {
        target: MonoExpr,
        value: MonoExpr,
    },
    From {
        path: StringNode,
        identifiers: Vec<(IdentifierNode, Option<IdentifierNode>)>, // optional alias
    },
    While {
        condition: Box<MonoExpr>,
        body: MonoBlockContents,
    },
}

#[derive(Clone, Debug)]
pub struct MonoStmt {
    pub kind: MonoStmtKind,
    pub span: Span,
}

```

`src/ast/monomorphized/monomorphized_type.rs`:

```rs
use crate::ast::base::base_declaration::EnumDecl;

use super::monomorphized_declaration::{MonoParam, MonoStructDecl, MonoTypeAliasDecl};

#[derive(Clone, Debug)]
pub enum MonoType {
    Void,
    Null,
    Bool,
    U8,
    U16,
    U32,
    U64,
    USize,
    ISize,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    Char,
    Struct(MonoStructDecl),
    Enum(EnumDecl),
    TypeAlias(MonoTypeAliasDecl),
    FnType {
        params: Vec<MonoParam>,
        return_type: Box<MonoType>,
    },
    // Infix types
    Union(Vec<MonoType>),
    // Suffix types
    Array {
        item_type: Box<MonoType>,
        size: usize,
    },
}

impl PartialEq for MonoType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (MonoType::Void, MonoType::Void) => true,
            (MonoType::Null, MonoType::Null) => true,
            (MonoType::Bool, MonoType::Bool) => true,
            (MonoType::U8, MonoType::U8) => true,
            (MonoType::U16, MonoType::U16) => true,
            (MonoType::U32, MonoType::U32) => true,
            (MonoType::U64, MonoType::U64) => true,
            (MonoType::USize, MonoType::USize) => true,
            (MonoType::ISize, MonoType::ISize) => true,
            (MonoType::I8, MonoType::I8) => true,
            (MonoType::I16, MonoType::I16) => true,
            (MonoType::I32, MonoType::I32) => true,
            (MonoType::I64, MonoType::I64) => true,
            (MonoType::F32, MonoType::F32) => true,
            (MonoType::F64, MonoType::F64) => true,
            (MonoType::Char, MonoType::Char) => true,
            (
                MonoType::Struct(MonoStructDecl {
                    identifier: this_identifier,
                    properties: this_properties,
                }),
                MonoType::Struct(MonoStructDecl {
                    identifier: other_identifier,
                    properties: other_properties,
                }),
            ) => {
                let same_name = this_identifier.name == other_identifier.name;

                if this_properties.len() != other_properties.len() {
                    return false;
                }

                let same_props = this_properties.iter().zip(other_properties.iter()).all(
                    |(this_param, other_param)| {
                        this_param.identifier.name == other_param.identifier.name
                            && this_param.constraint == other_param.constraint
                    },
                );

                same_name && same_props
            }
            (
                MonoType::TypeAlias(MonoTypeAliasDecl {
                    identifier: this_identifier,
                    value: this_value,
                }),
                MonoType::TypeAlias(MonoTypeAliasDecl {
                    identifier: other_identifier,
                    value: other_value,
                }),
            ) => {
                let same_name = this_identifier.name == other_identifier.name;

                let same_value = this_value == other_value;

                same_name && same_value
            }
            (
                MonoType::Enum(EnumDecl {
                    identifier: this_identifier,
                    variants: this_variants,
                    documentation: _,
                }),
                MonoType::Enum(EnumDecl {
                    identifier: other_identifier,
                    variants: other_variants,
                    documentation: _,
                }),
            ) => {
                this_identifier.name == other_identifier.name
                    && this_variants.iter().zip(other_variants.iter()).all(
                        |(this_variant, other_variant)| this_variant.name == other_variant.name,
                    )
            }
            (
                MonoType::FnType {
                    params: this_params,
                    return_type: this_return_type,
                },
                MonoType::FnType {
                    params: other_params,
                    return_type: other_return_type,
                },
            ) => {
                if this_params.len() != other_params.len() {
                    return false;
                }

                let same_params =
                    this_params
                        .iter()
                        .zip(other_params.iter())
                        .all(|(this_param, other_param)| {
                            this_param.identifier.name == other_param.identifier.name
                                && this_param.constraint == other_param.constraint
                        });

                let same_return_type = this_return_type == other_return_type;

                same_params && same_return_type
            }
            (MonoType::Union(left_items), MonoType::Union(right_items)) => {
                let same_len = left_items.len() == right_items.len();

                let same_items = left_items.iter().all(|item| right_items.contains(item))
                    && right_items.iter().all(|item| left_items.contains(item));

                same_len && same_items
            }
            (
                MonoType::Array {
                    item_type: this_left,
                    size: this_size,
                },
                MonoType::Array {
                    item_type: other_left,
                    size: other_size,
                },
            ) => this_left == other_left && this_size == other_size,
            _ => false,
        }
    }
}

```

`src/check/check_expr.rs`:

```rs
use std::{cell::RefCell, rc::Rc};

use crate::ast::{
    base::base_expression::{Expr, ExprKind},
    checked::{
        checked_expression::{CheckedExpr, CheckedExprKind},
        checked_type::{CheckedType, CheckedTypeKind, TypeSpan},
    },
};

use super::{
    expressions::{
        check_access_expr::check_access_expr, check_addition_expr::check_addition_expr,
        check_and_expr::check_and_expr,
        check_arithmetic_negation_expr::check_arithmetic_negation_expr,
        check_array_literal_expr::check_array_literal_expr, check_bool_expr::check_bool_expr,
        check_codeblock_expr::check_codeblock_expr, check_division_expr::check_division_expr,
        check_equality_expr::check_equality_expr, check_fn_call_expr::check_fn_call_expr,
        check_fn_expr::check_fn_expr, check_generic_apply_expr::check_generic_apply_expr,
        check_greater_than_expr::check_greater_than_expr,
        check_greater_than_or_equal_expr::check_greater_than_or_equal_expr,
        check_identifier_expr::check_identifier_expr, check_if_expr::check_if_expr,
        check_inequality_expr::check_inequality_expr, check_is_type_expr::check_is_type_expr,
        check_less_than_expr::check_less_than_expr,
        check_less_than_or_equal_expr::check_less_than_or_equal_expr,
        check_logical_negation_expr::check_logical_negation_expr,
        check_modulo_expr::check_modulo_expr, check_multiplication_expr::check_multiplication_expr,
        check_numeric_expr::check_numeric_expr, check_or_expr::check_or_expr,
        check_static_access_expr::check_static_access_expr, check_string_expr::check_string_expr,
        check_struct_init_expr::check_struct_init_expr,
        check_subtraction_expr::check_subtraction_expr, check_type_cast_expr::check_type_cast_expr,
    },
    scope::Scope,
    SemanticError,
};

pub fn check_expr(
    expr: Expr,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    match expr.kind {
        ExprKind::Not { right } => check_logical_negation_expr(right, expr.span, errors, scope),
        ExprKind::Neg { right } => check_arithmetic_negation_expr(right, expr.span, errors, scope),
        ExprKind::Add { left, right } => check_addition_expr(left, right, errors, scope),
        ExprKind::Subtract { left, right } => check_subtraction_expr(left, right, errors, scope),
        ExprKind::Multiply { left, right } => check_multiplication_expr(left, right, errors, scope),
        ExprKind::Divide { left, right } => check_division_expr(left, right, errors, scope),
        ExprKind::Modulo { left, right } => check_modulo_expr(left, right, errors, scope),
        ExprKind::LessThan { left, right } => check_less_than_expr(left, right, errors, scope),
        ExprKind::LessThanOrEqual { left, right } => {
            check_less_than_or_equal_expr(left, right, errors, scope)
        }
        ExprKind::GreaterThan { left, right } => {
            check_greater_than_expr(left, right, errors, scope)
        }
        ExprKind::GreaterThanOrEqual { left, right } => {
            check_greater_than_or_equal_expr(left, right, errors, scope)
        }
        ExprKind::Equal { left, right } => check_equality_expr(left, right, errors, scope),
        ExprKind::NotEqual { left, right } => check_inequality_expr(left, right, errors, scope),
        ExprKind::And { left, right } => check_and_expr(left, right, errors, scope),
        ExprKind::Or { left, right } => check_or_expr(left, right, errors, scope),
        ExprKind::Access { left, field } => check_access_expr(left, field, errors, scope),
        ExprKind::StaticAccess { left, field } => {
            check_static_access_expr(left, field, errors, scope)
        }
        ExprKind::TypeCast { left, target } => check_type_cast_expr(left, target, errors, scope),
        ExprKind::IsType { left, target } => {
            check_is_type_expr(left, target, expr.span, errors, scope)
        }
        ExprKind::GenericApply { left, args } => {
            check_generic_apply_expr(left, args, expr.span, errors, scope)
        }
        ExprKind::FnCall { left, args } => check_fn_call_expr(left, args, expr.span, errors, scope),
        ExprKind::StructInit { left, fields } => {
            check_struct_init_expr(left, fields, errors, scope)
        }
        ExprKind::Null => CheckedExpr {
            kind: CheckedExprKind::Null,
            expr_type: CheckedType {
                kind: CheckedTypeKind::Null,
                span: TypeSpan::Expr(expr.span),
            },
        },
        ExprKind::BoolLiteral { value } => check_bool_expr(value, expr.span),
        ExprKind::Number { value } => check_numeric_expr(value, expr.span),
        ExprKind::String(string_node) => check_string_expr(string_node, expr.span),
        ExprKind::Identifier(id) => check_identifier_expr(id, expr.span, errors, scope),
        ExprKind::Fn {
            params,
            body,
            return_type,
            generic_params,
        } => check_fn_expr(
            params,
            body,
            return_type,
            generic_params,
            expr.span,
            errors,
            scope,
        ),
        ExprKind::If {
            condition,
            then_branch,
            else_if_branches,
            else_branch,
        } => check_if_expr(
            condition,
            then_branch,
            else_if_branches,
            else_branch,
            expr.span,
            errors,
            scope,
        ),
        ExprKind::ArrayLiteral { items } => {
            check_array_literal_expr(items, expr.span, errors, scope)
        }
        ExprKind::Block(block_contents) => {
            check_codeblock_expr(block_contents, expr.span, errors, scope)
        }
        ExprKind::Error(parsing_error) => todo!(),
    }
}

```

`src/check/check_stmt.rs`:

```rs
use std::{cell::RefCell, rc::Rc};

use crate::ast::{
    base::{
        base_declaration::{
            GenericParam, Param, StructDecl as BaseStructDecl, TypeAliasDecl, VarDecl,
        },
        base_statement::{Stmt, StmtKind},
    },
    checked::{
        checked_declaration::{
            CheckedGenericParam, CheckedParam, CheckedVarDecl, GenericStructDecl, StructDecl,
        },
        checked_expression::{CheckedBlockContents, CheckedExprKind},
        checked_statement::{CheckedStmt, CheckedStmtKind},
        checked_type::{CheckedType, CheckedTypeKind, TypeSpan},
    },
};

use super::{
    check_expr::check_expr,
    check_stmts::check_stmts,
    scope::{Scope, ScopeKind, SymbolEntry},
    utils::{
        check_is_assignable::check_is_assignable,
        type_annotation_to_semantic::check_type,
    },
    SemanticError, SemanticErrorKind,
};

pub fn check_generic_params(
    generic_params: &Vec<GenericParam>,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> Vec<CheckedGenericParam> {
    generic_params
        .into_iter()
        .map(|gp| {
            let checked_constraint = gp.constraint.as_ref().map(|constraint| {
                Box::new(check_type(
                    constraint,
                    errors,
                    scope.clone(),
                ))
            });

            let checked_gp = CheckedGenericParam {
                constraint: checked_constraint,
                identifier: gp.identifier.clone(),
            };

            scope.borrow_mut().insert(
                gp.identifier.name.clone(),
                SymbolEntry::GenericParam(checked_gp.clone()),
            );
            checked_gp
        })
        .collect()
}

pub fn check_struct_properties(
    properties: &Vec<Param>,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> Vec<CheckedParam> {
    properties
        .into_iter()
        .map(|p| CheckedParam {
            constraint: check_type(&p.constraint, errors, scope.clone()),
            identifier: p.identifier.to_owned(),
        })
        .collect()
}

pub fn check_stmt(
    stmt: Stmt,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedStmt {
    match stmt.kind {
        StmtKind::Expression(expr) => CheckedStmt {
            kind: CheckedStmtKind::Expression(check_expr(expr, errors, scope)),
            span: stmt.span,
        },
        StmtKind::StructDecl(BaseStructDecl {
            identifier,
            documentation,
            generic_params,
            properties,
        }) => {
            let struct_scope = scope.borrow().child(ScopeKind::Struct);

            let generic_params =
                check_generic_params(&generic_params, errors, struct_scope.clone());

            let checked_properties =
                check_struct_properties(&properties, errors, struct_scope.clone());

            if generic_params.is_empty() {
                let decl = StructDecl {
                    identifier: identifier.to_owned(),
                    documentation,
                    properties: checked_properties,
                };
                scope
                    .borrow_mut()
                    .insert(identifier.name, SymbolEntry::StructDecl(decl.clone()));

                CheckedStmt {
                    kind: CheckedStmtKind::StructDecl(decl),
                    span: stmt.span,
                }
            } else {
                let decl = GenericStructDecl {
                    identifier: identifier.to_owned(),
                    documentation,
                    properties: checked_properties,
                    generic_params,
                };
                scope.borrow_mut().insert(
                    identifier.name,
                    SymbolEntry::GenericStructDecl(decl.clone()),
                );

                CheckedStmt {
                    kind: CheckedStmtKind::GenericStructDecl(decl),
                    span: stmt.span,
                }
            }
        }
        StmtKind::EnumDecl(decl) => {
            scope.borrow_mut().insert(
                decl.identifier.name.clone(),
                SymbolEntry::EnumDecl(decl.clone()),
            );

            CheckedStmt {
                kind: CheckedStmtKind::EnumDecl(decl),
                span: stmt.span,
            }
        }
        StmtKind::VarDecl(VarDecl {
            identifier,
            documentation,
            constraint,
            value,
        }) => {
            let checked_value = value.map(|v| check_expr(v, errors, scope.clone()));

            let checked_constraint =
                constraint.map(|c| check_type(&c, errors, scope.clone()));

            let final_constraint = match (&checked_value, checked_constraint) {
                (None, None) => {
                    errors.push(SemanticError::new(
                        SemanticErrorKind::VarDeclWithNoConstraintOrInitializer,
                        stmt.span,
                    ));

                    CheckedType {
                        kind: CheckedTypeKind::Unknown,
                        span: TypeSpan::Annotation(identifier.span),
                    }
                }
                (Some(value), Some(constraint)) => {
                    let is_assignable = check_is_assignable(&value.expr_type, &constraint);

                    if !is_assignable {
                        errors.push(SemanticError::new(
                            SemanticErrorKind::TypeMismatch {
                                expected: constraint.clone(),
                                received: value.expr_type.clone(),
                            },
                            stmt.span,
                        ));
                    }

                    constraint
                }
                (Some(value), None) => value.expr_type.clone(),
                (None, Some(t)) => t.clone(),
            };

            let checked_declaration = CheckedVarDecl {
                documentation,
                identifier: identifier.to_owned(),
                constraint: final_constraint,
                value: checked_value,
            };

            scope.borrow_mut().insert(
                identifier.name,
                SymbolEntry::VarDecl(checked_declaration.clone()),
            );

            CheckedStmt {
                kind: CheckedStmtKind::VarDecl(checked_declaration),
                span: stmt.span,
            }
        }
        StmtKind::TypeAliasDecl(TypeAliasDecl {
            identifier,
            documentation,
            generic_params,
            value,
        }) => todo!(),
        StmtKind::Break => {
            if !scope.borrow().is_within_loop() {
                errors.push(SemanticError::new(
                    SemanticErrorKind::BreakKeywordOutsideLoop,
                    stmt.span,
                ));
            }

            CheckedStmt {
                kind: CheckedStmtKind::Break,
                span: stmt.span,
            }
        }
        StmtKind::Continue => {
            if !scope.borrow().is_within_loop() {
                errors.push(SemanticError::new(
                    SemanticErrorKind::ContinueKeywordOutsideLoop,
                    stmt.span,
                ));
            }

            CheckedStmt {
                kind: CheckedStmtKind::Continue,
                span: stmt.span,
            }
        }
        StmtKind::Return(expr) => {
            if !scope.borrow().is_within_function() {
                errors.push(SemanticError::new(
                    SemanticErrorKind::ReturnKeywordOutsideFunction,
                    stmt.span,
                ));
            }

            CheckedStmt {
                kind: CheckedStmtKind::Return(check_expr(expr, errors, scope)),
                span: stmt.span,
            }
        }
        StmtKind::Assignment { target, value } => {
            let checked_target = check_expr(target, errors, scope.clone());
            let checked_value = check_expr(value, errors, scope.clone());

            match &checked_target.kind {
                CheckedExprKind::Identifier(id) => {
                    let identifier_expr_type = scope.borrow().lookup(&id.name);

                    if let Some(SymbolEntry::VarDecl(decl)) = identifier_expr_type {
                        let is_assignable =
                            check_is_assignable(&checked_value.expr_type, &decl.constraint);

                        if !is_assignable {
                            errors.push(SemanticError::new(
                                SemanticErrorKind::TypeMismatch {
                                    expected: decl.constraint.clone(),
                                    received: checked_value.expr_type.clone(),
                                },
                                stmt.span,
                            ));
                        }
                    } else {
                        errors.push(SemanticError::new(
                            SemanticErrorKind::UndeclaredIdentifier(id.name.clone()),
                            checked_target.expr_type.unwrap_expr_span(),
                        ));
                    }
                }
                CheckedExprKind::Access { left, field } => {}
                _ => {
                    errors.push(SemanticError::new(
                        SemanticErrorKind::InvalidAssignmentTarget,
                        checked_target.expr_type.unwrap_expr_span(),
                    ));
                }
            }

            CheckedStmt {
                kind: CheckedStmtKind::Assignment {
                    target: checked_target,
                    value: checked_value,
                },
                span: stmt.span,
            }
        }
        StmtKind::From { path, identifiers } => CheckedStmt {
            kind: CheckedStmtKind::From { identifiers, path },
            span: stmt.span,
        },
        StmtKind::While { condition, body } => {
            let while_scope = scope.borrow().child(ScopeKind::While);

            let checked_condition = check_expr(*condition, errors, scope.clone());

            if checked_condition.expr_type.kind != CheckedTypeKind::Bool {
                errors.push(SemanticError::new(
                    SemanticErrorKind::TypeMismatch {
                        expected: CheckedType {
                            kind: CheckedTypeKind::Bool,
                            span: checked_condition.expr_type.span,
                        },
                        received: checked_condition.expr_type.clone(),
                    },
                    checked_condition.expr_type.unwrap_expr_span(),
                ));
            }

            let checked_final_expr = body
                .final_expr
                .map(|expr| Box::new(check_expr(*expr, errors, while_scope.clone())));

            let checked_body_statements = check_stmts(body.statements, errors, while_scope);

            CheckedStmt {
                kind: CheckedStmtKind::While {
                    condition: Box::new(checked_condition),
                    body: CheckedBlockContents {
                        final_expr: checked_final_expr,
                        statements: checked_body_statements,
                    },
                },
                span: stmt.span,
            }
        }
        StmtKind::Error(parsing_error) => todo!(),
    }
}

```

`src/check/check_stmts.rs`:

```rs
use std::{cell::RefCell, rc::Rc};

use crate::ast::{base::base_statement::Stmt, checked::checked_statement::CheckedStmt};

use super::{check_stmt::check_stmt, scope::Scope, SemanticError};

pub fn check_stmts(
    statements: Vec<Stmt>,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> Vec<CheckedStmt> {
    let mut result = vec![];

    for stmt in statements {
        result.push(check_stmt(stmt, errors, scope.clone()));
    }

    result
}

```

`src/check/expressions/check_access_expr.rs`:

```rs
use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::base_expression::Expr,
        checked::{
            checked_declaration::StructDecl,
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{CheckedType, CheckedTypeKind, TypeSpan},
        },
        IdentifierNode, Span,
    },
    check::{check_expr::check_expr, scope::Scope, SemanticError, SemanticErrorKind},
};

pub fn check_access_expr(
    left: Box<Expr>,
    field: IdentifierNode,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    let span = Span {
        start: left.span.start,
        end: field.span.end,
    };
    let checked_left = check_expr(*left, errors, scope);

    let expr_type = match &checked_left.expr_type.kind {
        CheckedTypeKind::StructDecl(StructDecl { properties, .. }) => properties
            .iter()
            .find(|p| p.identifier == field)
            .map(|p| p.constraint.clone())
            .unwrap_or(CheckedType {
                kind: CheckedTypeKind::Unknown,
                span: TypeSpan::Expr(field.span),
            }),
        _ => {
            errors.push(SemanticError::new(
                SemanticErrorKind::UndefinedProperty(field.clone()),
                field.span,
            ));

            CheckedType {
                kind: CheckedTypeKind::Unknown,
                span: TypeSpan::Expr(span),
            }
        }
    };

    CheckedExpr {
        kind: CheckedExprKind::Access {
            left: Box::new(checked_left.clone()),
            field,
        },
        expr_type,
    }
}

```

`src/check/expressions/check_addition_expr.rs`:

```rs
use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::base_expression::Expr,
        checked::checked_expression::{CheckedExpr, CheckedExprKind},
    },
    check::{
        check_expr::check_expr, scope::Scope,
        utils::check_binary_numeric_operation::check_binary_numeric_operation, SemanticError,
    },
};

pub fn check_addition_expr(
    left: Box<Expr>,
    right: Box<Expr>,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    let checked_left = check_expr(*left, errors, scope.clone());
    let checked_right = check_expr(*right, errors, scope);
    let expr_type = check_binary_numeric_operation(&checked_left, &checked_right, errors);

    CheckedExpr {
        kind: CheckedExprKind::Add {
            left: Box::new(checked_left),
            right: Box::new(checked_right),
        },
        expr_type,
    }
}

```

`src/check/expressions/check_and_expr.rs`:

```rs
use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::base_expression::Expr,
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{CheckedType, CheckedTypeKind, TypeSpan},
        },
        Span,
    },
    check::{check_expr::check_expr, scope::Scope, SemanticError, SemanticErrorKind},
};

pub fn check_and_expr(
    left: Box<Expr>,
    right: Box<Expr>,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    let mut expr_type = CheckedType {
        kind: CheckedTypeKind::Bool,
        span: TypeSpan::Expr(Span {
            start: left.span.start,
            end: right.span.end,
        }),
    };

    let checked_left = check_expr(*left, errors, scope.clone());
    let checked_right = check_expr(*right, errors, scope);

    if checked_left.expr_type.kind != CheckedTypeKind::Bool {
        errors.push(SemanticError::new(
            SemanticErrorKind::TypeMismatch {
                expected: CheckedType {
                    kind: CheckedTypeKind::Bool,
                    span: checked_left.expr_type.span,
                },
                received: checked_left.expr_type.clone(),
            },
            checked_left.expr_type.unwrap_expr_span(),
        ));
        expr_type.kind = CheckedTypeKind::Unknown;
    }

    if checked_right.expr_type.kind != CheckedTypeKind::Bool {
        errors.push(SemanticError::new(
            SemanticErrorKind::TypeMismatch {
                expected: CheckedType {
                    kind: CheckedTypeKind::Bool,
                    span: checked_right.expr_type.span,
                },
                received: checked_right.expr_type.clone(),
            },
            checked_right.expr_type.unwrap_expr_span(),
        ));
        expr_type.kind = CheckedTypeKind::Unknown;
    }

    CheckedExpr {
        kind: CheckedExprKind::And {
            left: Box::new(checked_left),
            right: Box::new(checked_right),
        },
        expr_type,
    }
}

```

`src/check/expressions/check_arithmetic_negation_expr.rs`:

```rs
use std::{cell::RefCell, collections::HashSet, rc::Rc};

use crate::{
    ast::{
        base::base_expression::Expr,
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{CheckedType, CheckedTypeKind, TypeSpan},
        },
        Span,
    },
    check::{
        check_expr::check_expr, scope::Scope, utils::is_signed::is_signed, SemanticError,
        SemanticErrorKind,
    },
};

pub fn check_arithmetic_negation_expr(
    right: Box<Expr>,
    span: Span,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    let checked_right = check_expr(*right, errors, scope);

    let expr_type = match &checked_right.expr_type {
        t if is_signed(&t) => t.clone(),
        unexpected_type => {
            let expected = HashSet::from([
                CheckedType {
                    kind: CheckedTypeKind::I8,
                    span: checked_right.expr_type.span,
                },
                CheckedType {
                    kind: CheckedTypeKind::I16,
                    span: checked_right.expr_type.span,
                },
                CheckedType {
                    kind: CheckedTypeKind::I32,
                    span: checked_right.expr_type.span,
                },
                CheckedType {
                    kind: CheckedTypeKind::I64,
                    span: checked_right.expr_type.span,
                },
                CheckedType {
                    kind: CheckedTypeKind::ISize,
                    span: checked_right.expr_type.span,
                },
                CheckedType {
                    kind: CheckedTypeKind::F32,
                    span: checked_right.expr_type.span,
                },
                CheckedType {
                    kind: CheckedTypeKind::F64,
                    span: checked_right.expr_type.span,
                },
            ]);

            errors.push(SemanticError::new(
                SemanticErrorKind::TypeMismatch {
                    expected: CheckedType {
                        kind: CheckedTypeKind::Union(expected),
                        span: checked_right.expr_type.span,
                    },
                    received: unexpected_type.clone(),
                },
                checked_right.expr_type.unwrap_expr_span(),
            ));

            CheckedType {
                kind: CheckedTypeKind::Unknown,
                span: TypeSpan::Expr(span),
            }
        }
    };

    CheckedExpr {
        expr_type,
        kind: CheckedExprKind::Neg {
            right: Box::new(checked_right),
        },
    }
}

```

`src/check/expressions/check_array_literal_expr.rs`:

```rs
use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::base_expression::Expr,
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{CheckedType, CheckedTypeKind, TypeSpan},
        },
        Span,
    },
    check::{check_expr::check_expr, scope::Scope, utils::union_of::union_of, SemanticError},
};

pub fn check_array_literal_expr(
    items: Vec<Expr>,
    expr_span: Span,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    let size = items.len();

    let checked_items: Vec<CheckedExpr> = items
        .into_iter()
        .map(|item| check_expr(item, errors, scope.clone()))
        .collect();

    let unionized_types = union_of(checked_items.iter().map(|item| item.expr_type.clone()));

    CheckedExpr {
        expr_type: CheckedType {
            kind: CheckedTypeKind::Array {
                item_type: Box::new(unionized_types),
                size,
            },
            span: TypeSpan::Expr(expr_span),
        },
        kind: CheckedExprKind::ArrayLiteral {
            items: checked_items,
        },
    }
}

```

`src/check/expressions/check_bool_expr.rs`:

```rs
use crate::ast::{
    checked::{
        checked_expression::{CheckedExpr, CheckedExprKind},
        checked_type::{CheckedType, CheckedTypeKind, TypeSpan},
    },
    Span,
};

pub fn check_bool_expr(value: bool, expr_span: Span) -> CheckedExpr {
    CheckedExpr {
        kind: CheckedExprKind::BoolLiteral { value },

        expr_type: CheckedType {
            kind: CheckedTypeKind::Bool,
            span: TypeSpan::Expr(expr_span),
        },
    }
}

```

`src/check/expressions/check_codeblock_expr.rs`:

```rs
use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::base_expression::BlockContents,
        checked::{
            checked_expression::{CheckedBlockContents, CheckedExpr, CheckedExprKind},
            checked_type::{CheckedType, CheckedTypeKind, TypeSpan},
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
        .unwrap_or(CheckedType {
            kind: CheckedTypeKind::Void,
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

```

`src/check/expressions/check_division_expr.rs`:

```rs
use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::base_expression::Expr,
        checked::checked_expression::{CheckedExpr, CheckedExprKind},
    },
    check::{
        check_expr::check_expr, scope::Scope,
        utils::check_binary_numeric_operation::check_binary_numeric_operation, SemanticError,
    },
};

pub fn check_division_expr(
    left: Box<Expr>,
    right: Box<Expr>,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    let checked_left = check_expr(*left, errors, scope.clone());
    let checked_right = check_expr(*right, errors, scope);
    let expr_type = check_binary_numeric_operation(&checked_left, &checked_right, errors);

    CheckedExpr {
        kind: CheckedExprKind::Divide {
            left: Box::new(checked_left),
            right: Box::new(checked_right),
        },
        expr_type,
    }
}

```

`src/check/expressions/check_equality_expr.rs`:

```rs
use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::base_expression::Expr,
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{CheckedType, CheckedTypeKind, TypeSpan},
        },
        Span,
    },
    check::{
        check_expr::check_expr,
        scope::Scope,
        utils::{get_numeric_type_rank::get_numeric_type_rank, is_integer::is_integer},
        SemanticError, SemanticErrorKind,
    },
};

pub fn check_equality_expr(
    left: Box<Expr>,
    right: Box<Expr>,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    let span = Span {
        start: left.span.start,
        end: right.span.end,
    };
    let mut expr_type = CheckedType {
        kind: CheckedTypeKind::Bool,
        span: TypeSpan::Expr(span.clone()),
    };

    let checked_left = check_expr(*left, errors, scope.clone());
    let checked_right = check_expr(*right, errors, scope);

    let err = SemanticError::new(
        SemanticErrorKind::CannotCompareType {
            of: checked_left.expr_type.clone(),
            to: checked_right.expr_type.clone(),
        },
        span,
    );

    if !is_integer(&checked_left.expr_type)
        || !is_integer(&checked_right.expr_type)
        || get_numeric_type_rank(&checked_left.expr_type)
            < get_numeric_type_rank(&checked_right.expr_type)
    {
        errors.push(err);
        expr_type.kind = CheckedTypeKind::Unknown
    } else {
        match (&checked_left.expr_type.kind, &checked_right.expr_type.kind) {
            (CheckedTypeKind::Bool, CheckedTypeKind::Bool) => {}
            (CheckedTypeKind::Char, CheckedTypeKind::Char) => {}
            (CheckedTypeKind::Null, CheckedTypeKind::Null) => {}
            (CheckedTypeKind::Enum(_), CheckedTypeKind::Enum(_)) => {}
            _ => {
                errors.push(err);
                expr_type.kind = CheckedTypeKind::Unknown
            }
        }
    }

    CheckedExpr {
        kind: CheckedExprKind::Equal {
            left: Box::new(checked_left),
            right: Box::new(checked_right),
        },
        expr_type,
    }
}

```

`src/check/expressions/check_fn_call_expr.rs`:

```rs
use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::base_expression::Expr,
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{CheckedType, CheckedTypeKind, TypeSpan},
        },
        Span,
    },
    check::{
        check_expr::check_expr, scope::Scope, utils::check_is_assignable::check_is_assignable,
        SemanticError, SemanticErrorKind,
    },
};

pub fn check_fn_call_expr(
    left: Box<Expr>,
    args: Vec<Expr>,
    expr_span: Span,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    let checked_left = check_expr(*left, errors, scope.clone());
    let checked_args: Vec<_> = args
        .into_iter()
        .map(|arg| check_expr(arg, errors, scope.clone()))
        .collect();

    let mut call_result_type = CheckedType {
        kind: CheckedTypeKind::Unknown,
        span: TypeSpan::Expr(expr_span),
    };

    match &checked_left.expr_type.kind {
        CheckedTypeKind::FnType {
            params,
            return_type,
        } => {
            call_result_type = *return_type.clone();

            if checked_args.len() != params.len() {
                errors.push(SemanticError::new(
                    SemanticErrorKind::ArgumentCountMismatch {
                        expected: params.len(),
                        received: checked_args.len(),
                    },
                    expr_span,
                ));
            } else {
                for (param, arg) in params.iter().zip(checked_args.iter()) {
                    if !check_is_assignable(&arg.expr_type, &param.constraint) {
                        errors.push(SemanticError::new(
                            SemanticErrorKind::TypeMismatch {
                                expected: param.constraint.clone(),
                                received: arg.expr_type.clone(),
                            },
                            arg.expr_type.unwrap_expr_span(),
                        ));
                    }
                }
            }
        }
        CheckedTypeKind::GenericFnType {
            params,
            return_type,
            generic_params,
        } => {
            // --- Call on a Generic Function without Explicit Type Arguments ---
            // This requires type inference, which is complex.
            // For now, let's require explicit arguments for generic functions.
            todo!("Implement type inference and substitution")
        }
        non_callable_type => {
            errors.push(SemanticError::new(
                SemanticErrorKind::CannotCall(checked_left.expr_type.clone()),
                checked_left.expr_type.unwrap_expr_span(),
            ));
        }
    }

    CheckedExpr {
        expr_type: call_result_type,
        kind: CheckedExprKind::FnCall {
            left: Box::new(checked_left),
            args: checked_args,
        },
    }
}

```

`src/check/expressions/check_fn_expr.rs`:

```rs
use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::{
            base_declaration::{GenericParam, Param},
            base_expression::BlockContents,
            base_type::TypeAnnotation,
        },
        checked::{
            checked_declaration::{CheckedParam, CheckedVarDecl},
            checked_expression::{CheckedBlockContents, CheckedExpr, CheckedExprKind},
            checked_type::{CheckedType, CheckedTypeKind, TypeSpan},
        },
        Span,
    },
    check::{
        check_expr::check_expr,
        check_stmt::check_generic_params,
        check_stmts::check_stmts,
        scope::{Scope, ScopeKind, SymbolEntry},
        utils::{
            check_is_assignable::check_is_assignable, check_returns::check_returns,
            type_annotation_to_semantic::check_type, union_of::union_of,
        },
        SemanticError,
    },
};

pub fn check_fn_expr(
    params: Vec<Param>,
    body: BlockContents,
    return_type: Option<TypeAnnotation>,
    generic_params: Vec<GenericParam>,
    expr_span: Span,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    let fn_scope = scope.borrow().child(ScopeKind::Function);

    let checked_params: Vec<CheckedParam> = params
        .iter()
        .map(|param| {
            let checked_constraint = check_type(&param.constraint, errors, fn_scope.clone());

            fn_scope.borrow_mut().insert(
                param.identifier.name.to_owned(),
                SymbolEntry::VarDecl(CheckedVarDecl {
                    documentation: None,
                    identifier: param.identifier.to_owned(),
                    constraint: checked_constraint.clone(),
                    value: None,
                }),
            );

            CheckedParam {
                constraint: checked_constraint,
                identifier: param.identifier.to_owned(),
            }
        })
        .collect();
    let checked_generic_params = check_generic_params(&generic_params, errors, fn_scope.clone());

    let checked_statements = check_stmts(body.statements, errors, fn_scope.clone());
    let checked_final_expr = body
        .final_expr
        .map(|fe| Box::new(check_expr(*fe, errors, fn_scope.clone())));

    let checked_body = CheckedBlockContents {
        statements: checked_statements.clone(),
        final_expr: checked_final_expr.clone(),
    };

    let mut return_exprs = check_returns(&checked_statements, errors, fn_scope.clone());
    if let Some(final_expr) = checked_final_expr {
        return_exprs.push(*final_expr);
    }
    let inferred_return_type = union_of(return_exprs.iter().map(|e| e.expr_type.clone()));

    let param_types: Vec<CheckedParam> = params
        .into_iter()
        .map(|p| CheckedParam {
            constraint: check_type(&p.constraint, errors, fn_scope.clone()),
            identifier: p.identifier,
        })
        .collect();

    let expected_return_type =
        return_type.map(|return_t| check_type(&return_t, errors, fn_scope.clone()));

    let actual_return_type = if let Some(explicit_return_type) = expected_return_type {
        for return_expr in return_exprs.iter() {
            let is_assignable = check_is_assignable(&return_expr.expr_type, &explicit_return_type);
        }

        explicit_return_type
    } else {
        inferred_return_type
    };

    if generic_params.is_empty() {
        let expr_type = CheckedType {
            kind: CheckedTypeKind::FnType {
                params: param_types,
                return_type: Box::new(actual_return_type.clone()),
            },
            span: TypeSpan::Expr(expr_span),
        };

        CheckedExpr {
            expr_type,
            kind: CheckedExprKind::Fn {
                params: checked_params,
                body: checked_body,
                return_type: actual_return_type,
            },
        }
    } else {
        let expr_type = CheckedType {
            kind: CheckedTypeKind::GenericFnType {
                params: param_types,
                return_type: Box::new(actual_return_type.clone()),
                generic_params: checked_generic_params.clone(),
            },
            span: TypeSpan::Expr(expr_span),
        };

        CheckedExpr {
            expr_type,
            kind: CheckedExprKind::GenericFn {
                params: checked_params,
                body: checked_body,
                return_type: actual_return_type,
                generic_params: checked_generic_params,
            },
        }
    }
}

```

`src/check/expressions/check_generic_apply_expr.rs`:

```rs
use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::{base_expression::Expr, base_type::TypeAnnotation},
        checked::{
            checked_declaration::{
                CheckedGenericParam, CheckedParam, GenericStructDecl, GenericTypeAliasDecl,
                StructDecl,
            },
            checked_expression::{CheckedBlockContents, CheckedExpr, CheckedExprKind},
            checked_type::{CheckedType, CheckedTypeKind, TypeSpan},
        },
        Span,
    },
    check::{
        check_expr::check_expr,
        scope::Scope,
        utils::{
            check_is_assignable::check_is_assignable,
            substitute_generics::{substitute_generics, GenericSubstitutionMap},
            type_annotation_to_semantic::check_type,
        },
        SemanticError, SemanticErrorKind,
    },
};

pub fn check_generic_apply_expr(
    left: Box<Expr>,
    args: Vec<TypeAnnotation>,
    expr_span: Span,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    let checked_left = check_expr(*left, errors, scope.clone());
    let type_args: Vec<_> = args
        .into_iter()
        .map(|type_arg| check_type(&type_arg, errors, scope.clone()))
        .collect();

    let mut check_type_args = |generic_params: Vec<CheckedGenericParam>,
                               type_args: Vec<CheckedType>|
     -> Option<GenericSubstitutionMap> {
        let is_valid_usage = if generic_params.len() != type_args.len() {
            errors.push(SemanticError::new(
                SemanticErrorKind::GenericArgumentCountMismatch {
                    expected: generic_params.len(),
                    received: type_args.len(),
                },
                expr_span,
            ));

            false
        } else {
            let are_arguments_assignable =
                generic_params
                    .iter()
                    .zip(type_args.iter())
                    .all(|(gp, ta)| match &gp.constraint {
                        Some(constraint) => {
                            let is_assignable = check_is_assignable(ta, constraint);

                            if !is_assignable {
                                errors.push(SemanticError::new(
                                    SemanticErrorKind::TypeMismatch {
                                        expected: *constraint.clone(),
                                        received: ta.clone(),
                                    },
                                    ta.unwrap_annotation_span(),
                                ));
                            }

                            is_assignable
                        }
                        None => true,
                    });

            are_arguments_assignable
        };

        if !is_valid_usage {
            None
        } else {
            let substitution: GenericSubstitutionMap = generic_params
                .into_iter()
                .map(|gp| gp.identifier.name.clone())
                .zip(type_args.into_iter())
                .collect();

            Some(substitution)
        }
    };

    match checked_left.expr_type.kind {
        CheckedTypeKind::GenericFnType {
            params,
            return_type,
            generic_params,
        } => {
            if let Some(substitution) = check_type_args(generic_params, type_args) {
                let substituted_params: Vec<_> = params
                    .into_iter()
                    .map(|p| CheckedParam {
                        constraint: substitute_generics(&p.constraint, &substitution, errors),
                        identifier: p.identifier,
                    })
                    .collect();

                let substituted_return_type =
                    substitute_generics(&return_type, &substitution, errors);

                // Expressions that can potentially result in function type
                let substituted_body = match checked_left.kind {
                    CheckedExprKind::Identifier(id) => {
                        todo!()
                    }
                    CheckedExprKind::If {
                        condition,
                        then_branch,
                        else_if_branches,
                        else_branch,
                    } => {
                        todo!()
                    }
                    CheckedExprKind::TypeCast { left, target } => {
                        todo!()
                    }
                    CheckedExprKind::Block(CheckedBlockContents { final_expr, .. }) => {
                        todo!()
                    }
                    CheckedExprKind::GenericFn {
                        params,
                        body,
                        return_type,
                        generic_params,
                    } => {
                        todo!()
                    }
                    CheckedExprKind::FnCall { left, args } => {
                        todo!()
                    }
                    CheckedExprKind::Access { left, field } => {
                        todo!()
                    }
                    _ => {
                        unreachable!()
                    }
                };

                // CheckedExpr {
                //     kind: CheckedExprKind::Fn {
                //         params: substituted_params,
                //         body: (),
                //         return_type: substituted_return_type,
                //     },
                // }
            } else {
                todo!("return default CheckedExpr")
            }
        }
        CheckedTypeKind::GenericStructDecl(GenericStructDecl {
            identifier,
            documentation,
            generic_params,
            properties,
        }) => {
            if let Some(substitution) = check_type_args(generic_params, type_args) {
                let substituted_properties: Vec<_> = properties
                    .into_iter()
                    .map(|p| CheckedParam {
                        constraint: substitute_generics(&p.constraint, &substitution, errors),
                        identifier: p.identifier,
                    })
                    .collect();

                let new_id: String = "placeholder".to_string();

                CheckedExpr {
                    expr_type: CheckedType {
                        kind: CheckedTypeKind::StructDecl(StructDecl {
                            documentation,
                            identifier: new_id,
                            properties: substituted_properties,
                        }),
                        span: TypeSpan::Expr(expr_span),
                    },
                }
            } else {
                todo!("return default CheckedExpr")
            }
        }
        CheckedTypeKind::GenericTypeAliasDecl(GenericTypeAliasDecl {
            identifier,
            documentation,
            generic_params,
            value,
        }) => {
            if let Some(substitution) = check_type_args(generic_params, type_args) {
                let substituted_value = substitute_generics(&value, &substitution, errors);

                let new_id: String = "placeholder".to_string();

                CheckedExpr {
                    expr_type: CheckedType {
                        kind: CheckedTypeKind::StructDecl(StructDecl {
                            documentation,
                            identifier: new_id,
                            properties: substituted_properties,
                        }),
                        span: TypeSpan::Expr(expr_span),
                    },
                }
            } else {
                todo!("return default CheckedExpr")
            }
        }
        _ => {
            errors.push(SemanticError::new(
                SemanticErrorKind::CannotApplyTypeArguments {
                    to: checked_left.expr_type,
                },
                expr_span,
            ));
        }
    }

    todo!()
}

```

`src/check/expressions/check_greater_than_expr.rs`:

```rs
use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::base_expression::Expr,
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{CheckedType, CheckedTypeKind},
        },
    },
    check::{
        check_expr::check_expr, scope::Scope,
        utils::check_binary_numeric_operation::check_binary_numeric_operation, SemanticError,
    },
};

pub fn check_greater_than_expr(
    left: Box<Expr>,
    right: Box<Expr>,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    let checked_left = check_expr(*left, errors, scope.clone());
    let checked_right = check_expr(*right, errors, scope);
    let checked_op = check_binary_numeric_operation(&checked_left, &checked_right, errors);

    let type_kind = if checked_op.kind == CheckedTypeKind::Unknown {
        CheckedTypeKind::Unknown
    } else {
        CheckedTypeKind::Bool
    };

    let expr_type = CheckedType {
        kind: type_kind,
        span: checked_op.span,
    };

    CheckedExpr {
        kind: CheckedExprKind::GreaterThan {
            left: Box::new(checked_left),
            right: Box::new(checked_right),
        },
        expr_type,
    }
}

```

`src/check/expressions/check_greater_than_or_equal_expr.rs`:

```rs
use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::base_expression::Expr,
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{CheckedType, CheckedTypeKind},
        },
    },
    check::{
        check_expr::check_expr, scope::Scope,
        utils::check_binary_numeric_operation::check_binary_numeric_operation, SemanticError,
    },
};

pub fn check_greater_than_or_equal_expr(
    left: Box<Expr>,
    right: Box<Expr>,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    let checked_left = check_expr(*left, errors, scope.clone());
    let checked_right = check_expr(*right, errors, scope);
    let checked_op = check_binary_numeric_operation(&checked_left, &checked_right, errors);

    let type_kind = if checked_op.kind == CheckedTypeKind::Unknown {
        CheckedTypeKind::Unknown
    } else {
        CheckedTypeKind::Bool
    };

    let expr_type = CheckedType {
        kind: type_kind,
        span: checked_op.span,
    };

    CheckedExpr {
        kind: CheckedExprKind::GreaterThanOrEqual {
            left: Box::new(checked_left),
            right: Box::new(checked_right),
        },
        expr_type,
    }
}

```

`src/check/expressions/check_identifier_expr.rs`:

```rs
use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{CheckedType, CheckedTypeKind, TypeSpan},
        },
        IdentifierNode, Span,
    },
    check::{
        scope::{Scope, SymbolEntry},
        SemanticError, SemanticErrorKind,
    },
};

pub fn check_identifier_expr(
    id: IdentifierNode,
    expr_span: Span,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    let type_kind = scope
        .borrow()
        .lookup(&id.name)
        .map(|entry| match entry {
            SymbolEntry::GenericStructDecl(decl) => CheckedTypeKind::GenericStructDecl(decl),
            SymbolEntry::StructDecl(decl) => CheckedTypeKind::StructDecl(decl),
            SymbolEntry::GenericTypeAliasDecl(decl) => CheckedTypeKind::GenericTypeAliasDecl(decl),
            SymbolEntry::TypeAliasDecl(decl) => CheckedTypeKind::TypeAliasDecl(decl),
            SymbolEntry::EnumDecl(decl) => CheckedTypeKind::Enum(decl),
            SymbolEntry::VarDecl(decl) => decl.constraint.kind,
            SymbolEntry::GenericParam(_) => {
                errors.push(SemanticError::new(
                    SemanticErrorKind::CannotUseGenericParameterAsValue,
                    expr_span,
                ));

                CheckedTypeKind::Unknown
            }
        })
        .unwrap_or_else(|| {
            errors.push(SemanticError::new(
                SemanticErrorKind::UndeclaredIdentifier(id.name.clone()),
                expr_span,
            ));

            CheckedTypeKind::Unknown
        });

    CheckedExpr {
        kind: CheckedExprKind::Identifier(id),
        expr_type: CheckedType {
            kind: type_kind,
            span: TypeSpan::Expr(expr_span),
        },
    }
}

```

`src/check/expressions/check_if_expr.rs`:

```rs
use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::base_expression::{BlockContents, Expr},
        checked::{
            checked_expression::{CheckedBlockContents, CheckedExpr, CheckedExprKind},
            checked_type::{CheckedType, CheckedTypeKind, TypeSpan},
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
    let mut if_else_expr_type = CheckedType {
        kind: CheckedTypeKind::Void,
        span: TypeSpan::Expr(expr_span),
    };

    let checked_condition = check_expr(*condition, errors, scope.clone());
    if checked_condition.expr_type.kind != CheckedTypeKind::Bool {
        errors.push(SemanticError::new(
            SemanticErrorKind::TypeMismatch {
                expected: CheckedType {
                    kind: CheckedTypeKind::Bool,
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

        if_else_expr_type = union_of(
            [
                if_else_expr_type.clone(),
                checked_final_expr.expr_type.clone(),
            ]
            .into_iter(),
        );

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

                if_else_expr_type = union_of(
                    [
                        if_else_expr_type.clone(),
                        checked_final_expr.expr_type.clone(),
                    ]
                    .into_iter(),
                );

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

            if_else_expr_type = union_of(
                [
                    if_else_expr_type.clone(),
                    checked_final_expr.expr_type.clone(),
                ]
                .into_iter(),
            );

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

```

`src/check/expressions/check_inequality_expr.rs`:

```rs
use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::base_expression::Expr,
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{CheckedType, CheckedTypeKind, TypeSpan},
        },
        Span,
    },
    check::{check_expr::check_expr, scope::Scope, SemanticError},
};

pub fn check_inequality_expr(
    left: Box<Expr>,
    right: Box<Expr>,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    let mut expr_type = CheckedType {
        kind: CheckedTypeKind::Bool,
        span: TypeSpan::Expr(Span {
            start: left.span.start,
            end: right.span.end,
        }),
    };

    let checked_left = check_expr(*left, errors, scope.clone());
    let checked_right = check_expr(*right, errors, scope);

    // TODO: allow inequality checks for primitives

    CheckedExpr {
        kind: CheckedExprKind::NotEqual {
            left: Box::new(checked_left),
            right: Box::new(checked_right),
        },
        expr_type,
    }
}

```

`src/check/expressions/check_is_type_expr.rs`:

```rs
use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::{base_expression::Expr, base_type::TypeAnnotation},
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{CheckedType, CheckedTypeKind, TypeSpan},
        },
        Span,
    },
    check::{
        check_expr::check_expr, scope::Scope, utils::type_annotation_to_semantic::check_type,
        SemanticError, SemanticErrorKind,
    },
};

pub fn check_is_type_expr(
    left: Box<Expr>,
    target: TypeAnnotation,
    expr_span: Span,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    let checked_left = check_expr(*left, errors, scope.clone());
    let checked_target = check_type(&target, errors, scope);

    if !matches!(checked_left.expr_type.kind, CheckedTypeKind::Union { .. }) {
        errors.push(SemanticError::new(
            SemanticErrorKind::CannotUseIsTypeOnNonUnion,
            expr_span,
        ));
    }

    CheckedExpr {
        kind: CheckedExprKind::IsType {
            left: Box::new(checked_left),
            target: checked_target,
        },
        expr_type: CheckedType {
            kind: CheckedTypeKind::Bool,
            span: TypeSpan::Expr(expr_span),
        },
    }
}

```

`src/check/expressions/check_less_than_expr.rs`:

```rs
use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::base_expression::Expr,
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{CheckedType, CheckedTypeKind},
        },
    },
    check::{
        check_expr::check_expr, scope::Scope,
        utils::check_binary_numeric_operation::check_binary_numeric_operation, SemanticError,
    },
};

pub fn check_less_than_expr(
    left: Box<Expr>,
    right: Box<Expr>,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    let checked_left = check_expr(*left, errors, scope.clone());
    let checked_right = check_expr(*right, errors, scope);
    let checked_op = check_binary_numeric_operation(&checked_left, &checked_right, errors);

    let type_kind = if checked_op.kind == CheckedTypeKind::Unknown {
        CheckedTypeKind::Unknown
    } else {
        CheckedTypeKind::Bool
    };

    let expr_type = CheckedType {
        kind: type_kind,
        span: checked_op.span,
    };

    CheckedExpr {
        kind: CheckedExprKind::LessThan {
            left: Box::new(checked_left),
            right: Box::new(checked_right),
        },
        expr_type,
    }
}

```

`src/check/expressions/check_less_than_or_equal_expr.rs`:

```rs
use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::base_expression::Expr,
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{CheckedType, CheckedTypeKind},
        },
    },
    check::{
        check_expr::check_expr, scope::Scope,
        utils::check_binary_numeric_operation::check_binary_numeric_operation, SemanticError,
    },
};

pub fn check_less_than_or_equal_expr(
    left: Box<Expr>,
    right: Box<Expr>,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    let checked_left = check_expr(*left, errors, scope.clone());
    let checked_right = check_expr(*right, errors, scope);
    let checked_op = check_binary_numeric_operation(&checked_left, &checked_right, errors);

    let type_kind = if checked_op.kind == CheckedTypeKind::Unknown {
        CheckedTypeKind::Unknown
    } else {
        CheckedTypeKind::Bool
    };

    let expr_type = CheckedType {
        kind: type_kind,
        span: checked_op.span,
    };

    CheckedExpr {
        kind: CheckedExprKind::LessThanOrEqual {
            left: Box::new(checked_left),
            right: Box::new(checked_right),
        },
        expr_type,
    }
}

```

`src/check/expressions/check_logical_negation_expr.rs`:

```rs
use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::base_expression::Expr,
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{CheckedType, CheckedTypeKind, TypeSpan},
        },
        Span,
    },
    check::{check_expr::check_expr, scope::Scope, SemanticError, SemanticErrorKind},
};

pub fn check_logical_negation_expr(
    right: Box<Expr>,
    span: Span,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    let right_span = right.span;
    let checked_right = check_expr(*right, errors, scope);

    let mut expr_type = CheckedType {
        kind: CheckedTypeKind::Bool,
        span: TypeSpan::Expr(span),
    };

    if checked_right.expr_type.kind != CheckedTypeKind::Bool {
        errors.push(SemanticError::new(
            SemanticErrorKind::TypeMismatch {
                expected: CheckedType {
                    kind: CheckedTypeKind::Bool,
                    span: TypeSpan::Expr(right_span),
                },
                received: checked_right.expr_type.clone(),
            },
            span,
        ));
        expr_type.kind = CheckedTypeKind::Unknown
    }

    CheckedExpr {
        kind: CheckedExprKind::Not {
            right: Box::new(checked_right),
        },
        expr_type,
    }
}

```

`src/check/expressions/check_modulo_expr.rs`:

```rs
use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::base_expression::Expr,
        checked::checked_expression::{CheckedExpr, CheckedExprKind},
    },
    check::{
        check_expr::check_expr, scope::Scope,
        utils::check_binary_numeric_operation::check_binary_numeric_operation, SemanticError,
    },
};

pub fn check_modulo_expr(
    left: Box<Expr>,
    right: Box<Expr>,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    let checked_left = check_expr(*left, errors, scope.clone());
    let checked_right = check_expr(*right, errors, scope);
    let expr_type = check_binary_numeric_operation(&checked_left, &checked_right, errors);

    CheckedExpr {
        kind: CheckedExprKind::Modulo {
            left: Box::new(checked_left),
            right: Box::new(checked_right),
        },
        expr_type,
    }
}

```

`src/check/expressions/check_multiplication_expr.rs`:

```rs
use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::base_expression::Expr,
        checked::checked_expression::{CheckedExpr, CheckedExprKind},
    },
    check::{
        check_expr::check_expr, scope::Scope,
        utils::check_binary_numeric_operation::check_binary_numeric_operation, SemanticError,
    },
};

pub fn check_multiplication_expr(
    left: Box<Expr>,
    right: Box<Expr>,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    let checked_left = check_expr(*left, errors, scope.clone());
    let checked_right = check_expr(*right, errors, scope);
    let expr_type = check_binary_numeric_operation(&checked_left, &checked_right, errors);

    CheckedExpr {
        kind: CheckedExprKind::Multiply {
            left: Box::new(checked_left),
            right: Box::new(checked_right),
        },
        expr_type,
    }
}

```

`src/check/expressions/check_null_expr.rs`:

```rs
use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{base::base_expression::Expr, checked::checked_expression::CheckedExpr, IdentifierNode},
    check::{scope::Scope, SemanticError},
};

pub fn check_null_expr(
    left: Box<Expr>,
    field: IdentifierNode,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    todo!()
}

```

`src/check/expressions/check_numeric_expr.rs`:

```rs
use crate::{
    ast::{
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{CheckedType, CheckedTypeKind, TypeSpan},
        },
        Span,
    },
    tokenizer::NumberKind,
};

pub fn check_numeric_expr(value: NumberKind, expr_span: Span) -> CheckedExpr {
    let type_kind = match value {
        NumberKind::I64(_) => CheckedTypeKind::I64,
        NumberKind::I32(_) => CheckedTypeKind::I32,
        NumberKind::I16(_) => CheckedTypeKind::I16,
        NumberKind::I8(_) => CheckedTypeKind::I8,
        NumberKind::F32(_) => CheckedTypeKind::F32,
        NumberKind::F64(_) => CheckedTypeKind::F64,
        NumberKind::U64(_) => CheckedTypeKind::U64,
        NumberKind::U32(_) => CheckedTypeKind::U32,
        NumberKind::U16(_) => CheckedTypeKind::U16,
        NumberKind::U8(_) => CheckedTypeKind::U8,
        NumberKind::USize(_) => CheckedTypeKind::USize,
        NumberKind::ISize(_) => CheckedTypeKind::ISize,
    };

    CheckedExpr {
        kind: CheckedExprKind::Number { value },
        expr_type: CheckedType {
            kind: type_kind,
            span: TypeSpan::Expr(expr_span),
        },
    }
}

```

`src/check/expressions/check_or_expr.rs`:

```rs
use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::base_expression::Expr,
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{CheckedType, CheckedTypeKind, TypeSpan},
        },
        Span,
    },
    check::{check_expr::check_expr, scope::Scope, SemanticError, SemanticErrorKind},
};

pub fn check_or_expr(
    left: Box<Expr>,
    right: Box<Expr>,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    let mut expr_type = CheckedType {
        kind: CheckedTypeKind::Bool,
        span: TypeSpan::Expr(Span {
            start: left.span.start,
            end: right.span.end,
        }),
    };

    let checked_left = check_expr(*left, errors, scope.clone());
    let checked_right = check_expr(*right, errors, scope);

    if checked_left.expr_type.kind != CheckedTypeKind::Bool {
        errors.push(SemanticError::new(
            SemanticErrorKind::TypeMismatch {
                expected: CheckedType {
                    kind: CheckedTypeKind::Bool,
                    span: checked_left.expr_type.span,
                },
                received: checked_left.expr_type.clone(),
            },
            checked_left.expr_type.unwrap_expr_span(),
        ));
        expr_type.kind = CheckedTypeKind::Unknown;
    }

    if checked_right.expr_type.kind != CheckedTypeKind::Bool {
        errors.push(SemanticError::new(
            SemanticErrorKind::TypeMismatch {
                expected: CheckedType {
                    kind: CheckedTypeKind::Bool,
                    span: checked_right.expr_type.span,
                },
                received: checked_right.expr_type.clone(),
            },
            checked_right.expr_type.unwrap_expr_span(),
        ));
        expr_type.kind = CheckedTypeKind::Unknown;
    }

    CheckedExpr {
        kind: CheckedExprKind::Or {
            left: Box::new(checked_left),
            right: Box::new(checked_right),
        },
        expr_type,
    }
}

```

`src/check/expressions/check_static_access_expr.rs`:

```rs
use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{base::base_expression::Expr, checked::checked_expression::CheckedExpr, IdentifierNode},
    check::{scope::Scope, SemanticError},
};

pub fn check_static_access_expr(
    left: Box<Expr>,
    field: IdentifierNode,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    todo!()
}

```

`src/check/expressions/check_string_expr.rs`:

```rs
use crate::ast::{
    checked::{
        checked_expression::{CheckedExpr, CheckedExprKind},
        checked_type::{CheckedType, CheckedTypeKind, TypeSpan},
    },
    Span, StringNode,
};

pub fn check_string_expr(string_node: StringNode, expr_span: Span) -> CheckedExpr {
    CheckedExpr {
        expr_type: CheckedType {
            kind: CheckedTypeKind::Array {
                item_type: Box::new(CheckedType {
                    kind: CheckedTypeKind::Char,
                    span: TypeSpan::None,
                }),
                size: string_node.value.len(),
            },
            span: TypeSpan::Expr(expr_span),
        },
        kind: CheckedExprKind::String(string_node),
    }
}

```

`src/check/expressions/check_struct_init_expr.rs`:

```rs
use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{base::base_expression::Expr, checked::checked_expression::CheckedExpr, IdentifierNode},
    check::{scope::Scope, SemanticError},
};

pub fn check_struct_init_expr(
    left: Box<Expr>,
    fields: Vec<(IdentifierNode, Expr)>,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    todo!()
}

```

`src/check/expressions/check_subtraction_expr.rs`:

```rs
use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::base_expression::Expr,
        checked::checked_expression::{CheckedExpr, CheckedExprKind},
    },
    check::{
        check_expr::check_expr, scope::Scope,
        utils::check_binary_numeric_operation::check_binary_numeric_operation, SemanticError,
    },
};

pub fn check_subtraction_expr(
    left: Box<Expr>,
    right: Box<Expr>,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    let checked_left = check_expr(*left, errors, scope.clone());
    let checked_right = check_expr(*right, errors, scope);
    let expr_type = check_binary_numeric_operation(&checked_left, &checked_right, errors);

    CheckedExpr {
        kind: CheckedExprKind::Subtract {
            left: Box::new(checked_left),
            right: Box::new(checked_right),
        },
        expr_type,
    }
}

```

`src/check/expressions/check_type_cast_expr.rs`:

```rs
use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::{base_expression::Expr, base_type::TypeAnnotation},
        checked::checked_expression::CheckedExpr,
    },
    check::{scope::Scope, SemanticError},
};

pub fn check_type_cast_expr(
    left: Box<Expr>,
    target: TypeAnnotation,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedExpr {
    todo!()
}

```

`src/check/expressions/mod.rs`:

```rs
pub mod check_access_expr;
pub mod check_addition_expr;
pub mod check_and_expr;
pub mod check_arithmetic_negation_expr;
pub mod check_array_literal_expr;
pub mod check_bool_expr;
pub mod check_codeblock_expr;
pub mod check_division_expr;
pub mod check_equality_expr;
pub mod check_fn_call_expr;
pub mod check_fn_expr;
pub mod check_generic_apply_expr;
pub mod check_greater_than_expr;
pub mod check_greater_than_or_equal_expr;
pub mod check_identifier_expr;
pub mod check_if_expr;
pub mod check_inequality_expr;
pub mod check_is_type_expr;
pub mod check_less_than_expr;
pub mod check_less_than_or_equal_expr;
pub mod check_logical_negation_expr;
pub mod check_modulo_expr;
pub mod check_multiplication_expr;
pub mod check_null_expr;
pub mod check_numeric_expr;
pub mod check_or_expr;
pub mod check_static_access_expr;
pub mod check_string_expr;
pub mod check_struct_init_expr;
pub mod check_subtraction_expr;
pub mod check_type_cast_expr;

```

`src/check/mod.rs`:

```rs
use crate::{
    ast::{checked::checked_type::CheckedType, IdentifierNode, Span},
    tokenizer::NumberKind,
};

pub mod check_expr;
pub mod check_stmt;
pub mod check_stmts;
pub mod expressions;
pub mod scope;
pub mod type_flow_graph;
pub mod utils;

#[derive(Debug, Clone)]
pub enum SemanticErrorKind {
    NonNumericOperand,
    MixedSignedAndUnsigned,
    MixedFloatAndInteger,
    CannotCompareType { of: CheckedType, to: CheckedType },
    UndeclaredIdentifier(String),
    UndeclaredType(String),
    ReturnKeywordOutsideFunction,
    BreakKeywordOutsideLoop,
    ContinueKeywordOutsideLoop,
    InvalidAssignmentTarget,
    TypeMismatch { expected: CheckedType, received: CheckedType },
    InvalidArraySizeValue(NumberKind),
    ReturnNotLastStatement,
    ReturnTypeMismatch { expected: CheckedType, received: CheckedType },
    CannotAccess(CheckedType),
    CannotCall(CheckedType),
    ArgumentCountMismatch { expected: usize, received: usize },
    GenericArgumentCountMismatch { expected: usize, received: usize },
    CannotUseGenericParameterAsValue,
    CannotUseVariableDeclarationAsType,
    VarDeclWithNoConstraintOrInitializer,
    UndefinedProperty(IdentifierNode),
    UnresolvedGenericParam(String),
    CannotUseIsTypeOnNonUnion,
    ConflictingGenericBinding { existing: CheckedType, new: CheckedType },
    CannotApplyTypeArguments { to: CheckedType },
}

#[derive(Debug, Clone)]
pub struct SemanticError {
    kind: SemanticErrorKind,
    code: usize,
    span: Span,
}

impl SemanticError {
    fn kind_to_code(kind: &SemanticErrorKind) -> usize {
        match kind {
            SemanticErrorKind::NonNumericOperand => 1,
            SemanticErrorKind::MixedSignedAndUnsigned => 2,
            SemanticErrorKind::MixedFloatAndInteger => 3,
            SemanticErrorKind::CannotCompareType { .. } => 4,
            SemanticErrorKind::UndeclaredIdentifier { .. } => 5,
            SemanticErrorKind::ReturnKeywordOutsideFunction => 6,
            SemanticErrorKind::BreakKeywordOutsideLoop => 7,
            SemanticErrorKind::ContinueKeywordOutsideLoop => 8,
            SemanticErrorKind::InvalidAssignmentTarget => 9,
            SemanticErrorKind::TypeMismatch { .. } => 10,
            SemanticErrorKind::ReturnNotLastStatement => 11,
            SemanticErrorKind::ReturnTypeMismatch { .. } => 12,
            SemanticErrorKind::UndeclaredType(..) => 13,
            SemanticErrorKind::CannotAccess(..) => 14,
            SemanticErrorKind::CannotCall(..) => 15,
            SemanticErrorKind::CannotUseGenericParameterAsValue => 16,
            SemanticErrorKind::CannotUseVariableDeclarationAsType => 17,
            SemanticErrorKind::VarDeclWithNoConstraintOrInitializer => 18,
            SemanticErrorKind::UndefinedProperty(..) => 19,
            SemanticErrorKind::CannotUseIsTypeOnNonUnion => 20,
            SemanticErrorKind::InvalidArraySizeValue(..) => 21,
            SemanticErrorKind::ArgumentCountMismatch { .. } => 22,
            SemanticErrorKind::GenericArgumentCountMismatch { .. } => 23,
            SemanticErrorKind::UnresolvedGenericParam(..) => 24,
            SemanticErrorKind::ConflictingGenericBinding { .. } => 25,
        }
    }

    fn new(kind: SemanticErrorKind, span: Span) -> Self {
        let code = Self::kind_to_code(&kind);

        Self { code, kind, span }
    }

    fn get_kind(&self) -> &SemanticErrorKind {
        &self.kind
    }
}

```

`src/check/scope.rs`:

```rs
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::ast::{
    base::base_declaration::EnumDecl,
    checked::checked_declaration::{
        CheckedGenericParam, CheckedVarDecl, GenericStructDecl, GenericTypeAliasDecl, StructDecl,
        TypeAliasDecl,
    },
};

#[derive(Debug, Clone, PartialEq)]
pub enum ScopeKind {
    Function,
    While,
    CodeBlock,
    File,
    TypeAlias,
    Struct,
    FnType,
}

#[derive(Debug, Clone)]
pub enum SymbolEntry {
    GenericStructDecl(GenericStructDecl),
    StructDecl(StructDecl),
    EnumDecl(EnumDecl),
    VarDecl(CheckedVarDecl),
    GenericTypeAliasDecl(GenericTypeAliasDecl),
    TypeAliasDecl(TypeAliasDecl),
    GenericParam(CheckedGenericParam),
}

#[derive(Debug, Clone)]
pub struct Scope {
    parent: Option<Rc<RefCell<Scope>>>,
    symbols: HashMap<String, SymbolEntry>,
    pub kind: ScopeKind,
}

impl Scope {
    pub fn new(kind: ScopeKind) -> Scope {
        Scope {
            parent: None,
            symbols: HashMap::new(),
            kind,
        }
    }

    pub fn new_with_parent(parent: Rc<RefCell<Scope>>, kind: ScopeKind) -> Scope {
        Scope {
            parent: Some(parent),
            symbols: HashMap::new(),
            kind,
        }
    }

    pub fn insert(&mut self, key: String, value: SymbolEntry) {
        self.symbols.insert(key, value);
    }

    pub fn lookup(&self, key: &str) -> Option<SymbolEntry> {
        if let Some(value) = self.symbols.get(key) {
            return Some(value.clone());
        }

        if let Some(parent) = &self.parent {
            return parent.borrow().lookup(key);
        }

        None
    }

    pub fn is_within_function(&self) -> bool {
        if self.kind != ScopeKind::Function {
            self.parent
                .as_ref()
                .map(|p| p.borrow().is_within_function())
                .unwrap_or(false)
        } else {
            true
        }
    }

    pub fn is_within_loop(&self) -> bool {
        if self.kind != ScopeKind::While {
            self.parent
                .as_ref()
                .map(|p| {
                    let p = p.borrow();
                    if p.kind != ScopeKind::Function && p.kind != ScopeKind::File {
                        p.is_within_loop()
                    } else {
                        false
                    }
                })
                .unwrap_or(false)
        } else {
            true
        }
    }

    pub fn child(&self, kind: ScopeKind) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Scope::new_with_parent(
            Rc::new(RefCell::new(self.clone())),
            kind,
        )))
    }
}

```

`src/check/type_flow_graph.rs`:

```rs
use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
    usize,
};

use crate::ast::checked::{
    checked_expression::{CheckedExpr, CheckedExprKind},
    checked_type::{CheckedType, CheckedTypeKind, TypeSpan},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TFGNodeId(usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VariableId(String);

/// Represents the type a specific variable is narrowed to on a given path.
/// This is calculated and stored by the TFG builder.
#[derive(Debug, Clone)]
pub struct NarrowingInfo {
    pub variable: VariableId,
    pub narrowed_type: Rc<CheckedType>,
}

/// Represents the different kinds of nodes in the Type Flow Graph.
/// Edges are implicitly defined by the `next_node` fields.
#[derive(Debug, Clone)]
pub enum TFGNodeKind {
    /// Start of the analyzed scope.
    Entry {
        next_node: Option<TFGNodeId>,
    },
    /// End of the analyzed scope (e.g., return, end of block). Terminal node.
    Exit,

    /// Represents a conditional branch point.
    /// The builder determines the immediate type implications for relevant variables.
    Branch {
        /// Info about the primary variable being narrowed if the condition is false.
        /// If the condition itself doesn't narrow (like `x > 0`), the builder stores
        /// None
        narrowing_if_true: Option<NarrowingInfo>,
        next_node_if_true: Option<TFGNodeId>,

        /// Info about the primary variable being narrowed if the condition is false.
        narrowing_if_false: Option<NarrowingInfo>,
        next_node_if_false: Option<TFGNodeId>,
    },

    /// Represents an assignment `target = <expr>`.
    Assign {
        target: VariableId,
        assigned_type: Rc<CheckedType>,
        next_node: Option<TFGNodeId>,
    },

    NoOp {
        next_node: Option<TFGNodeId>,
    },
}

#[derive(Debug)]
pub struct TFGNode {
    pub id: TFGNodeId,
    pub kind: TFGNodeKind,
    pub predecessors: HashSet<TFGNodeId>,
}

#[derive(Debug)]
pub struct TypeFlowGraph {
    nodes: HashMap<TFGNodeId, TFGNode>,
    pub entry_node_id: TFGNodeId,
    node_counter: usize,
}

impl TypeFlowGraph {
    pub fn new() -> Self {
        let entry_node_id = TFGNodeId(0);

        TypeFlowGraph {
            nodes: HashMap::from([(
                entry_node_id,
                TFGNode {
                    id: entry_node_id,
                    kind: TFGNodeKind::Entry { next_node: None },
                    predecessors: HashSet::new(),
                },
            )]),
            entry_node_id,
            node_counter: 1,
        }
    }

    pub fn create_node(&mut self, kind: TFGNodeKind) -> TFGNodeId {
        let id = TFGNodeId(self.node_counter);
        self.node_counter += 1;

        self.nodes.insert(
            id,
            TFGNode {
                id,
                kind,
                predecessors: HashSet::new(),
            },
        );

        id
    }

    fn link_sequential(&mut self, from_id: TFGNodeId, to_id: TFGNodeId) {
        let from_node = self
            .nodes
            .get_mut(&from_id)
            .expect("Expected node with 'from_id' to exist");

        let next_field = match &mut from_node.kind {
            TFGNodeKind::Entry { next_node } => next_node,
            TFGNodeKind::Assign { next_node, .. } => next_node,
            TFGNodeKind::NoOp { next_node } => next_node,
            TFGNodeKind::Branch { .. } => {
                panic!("Cannot link_sequential from a Branch node")
            }
            TFGNodeKind::Exit => panic!("Cannot link_sequential from an Exit node"),
        };

        *next_field = Some(to_id);

        let to_node = self
            .nodes
            .get_mut(&to_id)
            .expect("Expected node with 'to_id' to exist");

        to_node.predecessors.insert(from_id);
    }

    fn link_branch(
        &mut self,
        branch_id: TFGNodeId,
        target_if_true: TFGNodeId,
        target_if_false: TFGNodeId,
    ) {
        let branch_node = self
            .nodes
            .get_mut(&branch_id)
            .expect("Branch node doesn't exist");

        match &mut branch_node.kind {
            TFGNodeKind::Branch {
                next_node_if_true,
                next_node_if_false,
                ..
            } => {
                *next_node_if_true = Some(target_if_true);
                *next_node_if_false = Some(target_if_false);
            }
            _ => panic!("link_branch called on non-branch node"),
        }

        self.nodes
            .get_mut(&target_if_true)
            .expect("target_if_true node must exist")
            .predecessors
            .insert(branch_id);

        self.nodes
            .get_mut(&target_if_false)
            .expect("target_if_false node must exist")
            .predecessors
            .insert(branch_id);
    }

    pub fn get_node(&self, id: TFGNodeId) -> Option<&TFGNode> {
        self.nodes.get(&id)
    }

    pub fn iter_nodes(&self) -> impl Iterator<Item = &TFGNode> {
        self.nodes.values()
    }
}

#[derive(Debug, Clone)]
pub struct TypeState {
    pub variable_types: HashMap<VariableId, Rc<CheckedType>>,
}

pub fn union_subtract(
    mut from_union: HashSet<CheckedType>,
    type_to_remove: &CheckedType,
) -> HashSet<CheckedType> {
    match &type_to_remove.kind {
        CheckedTypeKind::Union(types_to_remove) => {
            for t in types_to_remove {
                from_union.remove(t);
            }
        }
        _ => {
            from_union.remove(type_to_remove);
        }
    };

    from_union
}

fn analyze_atomic_condition(condition: &CheckedExpr) -> Option<(NarrowingInfo, NarrowingInfo)> {
    match &condition.kind {
        CheckedExprKind::IsType { left, target } => {
            if let CheckedExprKind::Identifier(id) = &left.kind {
                let var_id = VariableId(id.name.clone());

                let false_type = if let CheckedTypeKind::Union(types) = left.expr_type.kind.clone()
                {
                    CheckedType {
                        kind: CheckedTypeKind::Union(union_subtract(types, target)),
                        span: left.expr_type.span,
                    }
                } else {
                    panic!("Cannot subtract from non-union type")
                };

                let narrowing_true = NarrowingInfo {
                    variable: var_id.clone(),
                    narrowed_type: Rc::new(target.clone()),
                };
                let narrowing_false = NarrowingInfo {
                    variable: var_id,
                    narrowed_type: Rc::new(false_type),
                };
                return Some((narrowing_true, narrowing_false));
            }
        }

        CheckedExprKind::NotEqual { left, right } => {
            let ident_expr = if matches!(right.kind, CheckedExprKind::Null) {
                Some(left)
            } else if matches!(left.kind, CheckedExprKind::Null) {
                Some(right)
            } else {
                None
            };

            if let Some(expr) = ident_expr {
                if let CheckedExprKind::Identifier(id) = &expr.kind {
                    let var_id = VariableId(id.name.clone());
                    let original_type = &expr.expr_type;

                    let true_type =
                        if let CheckedTypeKind::Union(types) = left.expr_type.kind.clone() {
                            CheckedType {
                                kind: CheckedTypeKind::Union(union_subtract(
                                    types,
                                    &CheckedType {
                                        kind: CheckedTypeKind::Null,
                                        span: TypeSpan::None,
                                    },
                                )),
                                span: original_type.span,
                            }
                        } else {
                            panic!("Cannot subtract from non-union type")
                        };

                    let narrowing_true = NarrowingInfo {
                        variable: var_id.clone(),
                        narrowed_type: Rc::new(true_type),
                    };
                    let narrowing_false = NarrowingInfo {
                        variable: var_id,
                        narrowed_type: Rc::new(CheckedType {
                            kind: CheckedTypeKind::Null,
                            span: original_type.span,
                        }),
                    };
                    return Some((narrowing_true, narrowing_false));
                }
            }
        }

        CheckedExprKind::Equal { left, right } => {
            let ident_expr = if matches!(right.kind, CheckedExprKind::Null) {
                Some(left)
            } else if matches!(left.kind, CheckedExprKind::Null) {
                Some(right)
            } else {
                None
            };

            if let Some(expr) = ident_expr {
                if let CheckedExprKind::Identifier(id) = &expr.kind {
                    let var_id = VariableId(id.name.clone());
                    let null_type = Rc::new(CheckedType {
                        span: TypeSpan::None,
                        kind: CheckedTypeKind::Null,
                    });

                    let false_type =
                        if let CheckedTypeKind::Union(types) = left.expr_type.kind.clone() {
                            CheckedType {
                                kind: CheckedTypeKind::Union(union_subtract(
                                    types,
                                    &CheckedType {
                                        kind: CheckedTypeKind::Null,
                                        span: TypeSpan::None,
                                    },
                                )),
                                span: left.expr_type.span,
                            }
                        } else {
                            panic!("Cannot subtract from non-union type")
                        };

                    let narrowing_true = NarrowingInfo {
                        variable: var_id.clone(),
                        narrowed_type: null_type,
                    };
                    let narrowing_false = NarrowingInfo {
                        variable: var_id,
                        narrowed_type: Rc::new(false_type),
                    };
                    return Some((narrowing_true, narrowing_false));
                }
            }
        }
        _ => {}
    }

    None
}

fn build_condition_tfg(
    tfg: &mut TypeFlowGraph,
    condition: &CheckedExpr,
    prev_node: TFGNodeId,
    next_node_if_true: TFGNodeId,
    next_node_if_false: TFGNodeId,
) {
    match &condition.kind {
        CheckedExprKind::And { left, right } => {
            let intermediate_node_id = tfg.create_node(TFGNodeKind::NoOp { next_node: None });

            build_condition_tfg(
                tfg,
                &left,
                prev_node,
                intermediate_node_id,
                next_node_if_false,
            );

            build_condition_tfg(
                tfg,
                &right,
                intermediate_node_id,
                next_node_if_true,
                next_node_if_false,
            );
        }
        CheckedExprKind::Or { left, right } => {
            let intermediate_node_id = tfg.create_node(TFGNodeKind::NoOp { next_node: None });

            build_condition_tfg(
                tfg,
                &left,
                prev_node,
                next_node_if_true,
                intermediate_node_id,
            );

            build_condition_tfg(
                tfg,
                &right,
                intermediate_node_id,
                next_node_if_true,
                next_node_if_false,
            );
        }
        CheckedExprKind::BoolLiteral { value } => {
            let target = if *value {
                next_node_if_true
            } else {
                next_node_if_false
            };

            // Need to handle linking from different prev_node types
            // For simplicity, assume prev_node is always linkable sequentially here,
            // but a robust implementation might need checks or different linking.
            tfg.link_sequential(prev_node, target);
        }

        _ => {
            let branch_node_id = tfg.create_node(TFGNodeKind::Branch {
                narrowing_if_true: None,
                next_node_if_true: None,
                narrowing_if_false: None,
                next_node_if_false: None,
            });

            // Link the previous node to this new branch
            // Assume prev_node is suitable for link_sequential. If prev_node could be
            // a Branch itself, this linking needs adjustment. In the current recursive
            // structure, prev_node will be Entry, NoOp, or another Branch's target.
            // Let's assume link_sequential works for Entry/NoOp.
            tfg.link_sequential(prev_node, branch_node_id);

            if let Some((narrowing_true, narrowing_false)) = analyze_atomic_condition(condition) {
                let branch_node = tfg.nodes.get_mut(&branch_node_id).unwrap();
                if let TFGNodeKind::Branch {
                    narrowing_if_true: nit,
                    narrowing_if_false: nif,
                    ..
                } = &mut branch_node.kind
                {
                    *nit = Some(narrowing_true);
                    *nif = Some(narrowing_false);
                }
            }

            tfg.link_branch(branch_node_id, next_node_if_true, next_node_if_false);
        }
    }
}

```

`src/check/utils/check_binary_numeric_operation.rs`:

```rs
use crate::{
    ast::{
        checked::{
            checked_expression::CheckedExpr,
            checked_type::{CheckedType, CheckedTypeKind, TypeSpan},
        },
        Span,
    },
    check::{SemanticError, SemanticErrorKind},
};

use super::{
    get_numeric_type_rank::get_numeric_type_rank, is_float::is_float, is_integer::is_integer,
    is_signed::is_signed,
};

pub fn check_binary_numeric_operation(
    left: &CheckedExpr,
    right: &CheckedExpr,
    errors: &mut Vec<SemanticError>,
) -> CheckedType {
    let combined_span = Span {
        start: left.expr_type.unwrap_expr_span().start,
        end: right.expr_type.unwrap_expr_span().start,
    };

    let left_type = if is_float(&left.expr_type) || is_integer(&left.expr_type) {
        &left.expr_type
    } else {
        errors.push(SemanticError::new(
            SemanticErrorKind::NonNumericOperand,
            left.expr_type.unwrap_expr_span(),
        ));

        return CheckedType {
            kind: CheckedTypeKind::Unknown,
            span: TypeSpan::Expr(combined_span),
        };
    };

    let right_type = if is_float(&right.expr_type) || is_integer(&right.expr_type) {
        &right.expr_type
    } else {
        errors.push(SemanticError::new(
            SemanticErrorKind::NonNumericOperand,
            right.expr_type.unwrap_expr_span(),
        ));

        return CheckedType {
            kind: CheckedTypeKind::Unknown,
            span: TypeSpan::Expr(combined_span),
        };
    };

    if (is_float(&left_type) && is_integer(&right_type))
        || (is_integer(&left_type) && is_float(&right_type))
    {
        errors.push(SemanticError::new(
            SemanticErrorKind::MixedFloatAndInteger,
            combined_span,
        ));
        return CheckedType {
            kind: CheckedTypeKind::Unknown,
            span: TypeSpan::Expr(combined_span),
        };
    }

    if is_signed(&left_type) != is_signed(&right_type) {
        errors.push(SemanticError::new(
            SemanticErrorKind::MixedSignedAndUnsigned,
            combined_span,
        ));
        return CheckedType {
            kind: CheckedTypeKind::Unknown,
            span: TypeSpan::Expr(combined_span),
        };
    }

    if right_type == left_type {
        return left_type.clone();
    }

    let left_rank = get_numeric_type_rank(&left_type);
    let right_rank = get_numeric_type_rank(&right_type);

    if left_rank > right_rank {
        left_type.clone()
    } else {
        right_type.clone()
    }
}

```

`src/check/utils/check_is_assignable.rs`:

```rs
use crate::ast::checked::checked_type::{CheckedType, CheckedTypeKind};

pub fn check_is_assignable(source_type: &CheckedType, target_type: &CheckedType) -> bool {
    use CheckedTypeKind::*;

    match (&source_type.kind, &target_type.kind) {
        (I8, I8)
        | (I16, I16)
        | (I32, I32)
        | (I64, I64)
        | (ISize, ISize)
        | (U8, U8)
        | (U16, U16)
        | (U32, U32)
        | (U64, U64)
        | (USize, USize)
        | (F32, F32)
        | (F64, F64)
        | (Char, Char)
        | (Bool, Bool)
        | (Null, Null)
        | (Void, Void)
        | (Unknown, _) => true,
        (Union(source), Union(target)) => source.iter().all(|source_item| {
            target
                .iter()
                .any(|target_item| check_is_assignable(source_item, target_item))
        }),
        (GenericParam(source), GenericParam(target)) => {
            match (&source.constraint, &target.constraint) {
                (Some(left_constraint), Some(right_constraint)) => {
                    check_is_assignable(&left_constraint, &right_constraint)
                }
                _ => false,
            }
        }
        (GenericStructDecl(source), GenericStructDecl(target)) => todo!(),
        (StructDecl(source), StructDecl(target)) => todo!(),
        (
            Array {
                item_type: source_type,
                size: source_size,
            },
            Array {
                item_type: target_type,
                size: target_size,
            },
        ) => {
            let same_size = source_size == target_size;
            let assignable_types = check_is_assignable(&source_type, &target_type);

            same_size && assignable_types
        }
        (
            FnType {
                params: source_params,
                return_type: source_return_type,
            },
            FnType {
                params: target_params,
                return_type: target_return_type,
            },
        ) => todo!(),
        (
            GenericFnType {
                params: source_params,
                return_type: source_return_type,
                generic_params: source_generic_params,
            },
            GenericFnType {
                params: target_params,
                return_type: target_return_type,
                generic_params: target_generic_params,
            },
        ) => todo!(),
        (Enum(source), Enum(target)) => {
            let same_name = source.identifier.name == target.identifier.name;
            let same_len = source.variants.len() == target.variants.len();

            false
        }

        // TODO: add type alias handling
        _ => false,
    }
}

```

`src/check/utils/check_returns.rs`:

```rs
use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::checked::{
        checked_expression::{CheckedExpr, CheckedExprKind},
        checked_statement::{CheckedStmt, CheckedStmtKind},
    },
    check::{scope::Scope, SemanticError, SemanticErrorKind},
};

pub fn check_returns(
    statements: &[CheckedStmt],
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> Vec<CheckedExpr> {
    let mut returns: Vec<CheckedExpr> = vec![];

    let stmt_count = statements.len();

    for (i, stmt) in statements.iter().enumerate() {
        match &stmt.kind {
            CheckedStmtKind::Return(expr) => {
                if i < stmt_count - 1 {
                    errors.push(SemanticError::new(
                        SemanticErrorKind::ReturnNotLastStatement,
                        stmt.span,
                    ));
                }
                returns.push(expr.clone());
            }
            CheckedStmtKind::While { body, .. } => {
                returns.extend(check_returns(&body.statements, errors, scope.clone()));
            }
            CheckedStmtKind::Expression(expr) => {
                if let CheckedExprKind::If {
                    then_branch,
                    else_if_branches,
                    else_branch,
                    ..
                } = &expr.kind
                {
                    returns.extend(check_returns(
                        &then_branch.statements,
                        errors,
                        scope.clone(),
                    ));

                    for (_, block) in else_if_branches {
                        returns.extend(check_returns(&block.statements, errors, scope.clone()));
                    }
                    if let Some(else_block) = else_branch {
                        returns.extend(check_returns(
                            &else_block.statements,
                            errors,
                            scope.clone(),
                        ));
                    }
                } else if let CheckedExprKind::Block(block) = &expr.kind {
                    returns.extend(check_returns(&block.statements, errors, scope.clone()));
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

```

`src/check/utils/get_numeric_type_rank.rs`:

```rs
use crate::ast::checked::checked_type::{CheckedType, CheckedTypeKind};

pub fn get_numeric_type_rank(ty: &CheckedType) -> i32 {
    use CheckedTypeKind::*;
    match &ty.kind {
        I8 | U8 => 1,
        I16 | U16 => 2,
        I32 | U32 | ISize | USize => 3,
        I64 | U64 => 4,
        F32 => 5,
        F64 => 6,
        _ => 0,
    }
}

```

`src/check/utils/infer_generics.rs`:

```rs
use crate::{
    ast::checked::checked_type::{CheckedType, CheckedTypeKind},
    check::{SemanticError, SemanticErrorKind},
};

use super::substitute_generics::GenericSubstitutionMap;

pub fn infer_generics(
    expected: &CheckedType,
    received: &CheckedType,
    substitution: &mut GenericSubstitutionMap,
    errors: &mut Vec<SemanticError>,
) {
    match (&expected.kind, &received.kind) {
        // Handle generics
        (CheckedTypeKind::GenericParam(gp), received_kind) => {
            let name = &gp.identifier.name;
            if let Some(existing) = substitution.get(name) {
                if &existing.kind != received_kind {
                    errors.push(SemanticError::new(
                        SemanticErrorKind::ConflictingGenericBinding {
                            existing: existing.clone(),
                            new: received.clone(),
                        },
                        received.unwrap_annotation_span(),
                    ));
                }
            } else {
                substitution.insert(name.clone(), received.clone());
            }
        }
        // Recursively check components (arrays, structs, etc.)
        (
            CheckedTypeKind::Array {
                item_type: maybe_generic,
                ..
            },
            CheckedTypeKind::Array {
                item_type: concrete,
                ..
            },
        ) => {
            infer_generics(maybe_generic, concrete, substitution, errors);
        }
        (
            CheckedTypeKind::GenericStructDecl(maybe_generic),
            CheckedTypeKind::GenericStructDecl(concrete),
        ) => {
            for (maybe_generic_prop, concrete_prop) in maybe_generic
                .properties
                .iter()
                .zip(concrete.properties.iter())
            {
                infer_generics(
                    &maybe_generic_prop.constraint,
                    &concrete_prop.constraint,
                    substitution,
                    errors,
                );
            }
        }
        (
            CheckedTypeKind::GenericFnType {
                params: maybe_generic_params,
                return_type: maybe_generic_return_type,
                generic_params: _,
            },
            CheckedTypeKind::GenericFnType {
                params: concrete_params,
                return_type: concrete_return_type,
                generic_params: _,
            },
        ) => {
            todo!("Implement inferring types for functions")
        }
        (CheckedTypeKind::Union(maybe_generic), CheckedTypeKind::Union(concrete)) => {
            todo!("Implement inferring types for unions")
        }
        _ => {}
    }
}

```

`src/check/utils/is_float.rs`:

```rs
use crate::ast::checked::checked_type::{CheckedType, CheckedTypeKind};

pub fn is_float(ty: &CheckedType) -> bool {
    use CheckedTypeKind::*;
    matches!(ty.kind, F32 | F64)
}

```

`src/check/utils/is_integer.rs`:

```rs
use crate::ast::checked::checked_type::{CheckedType, CheckedTypeKind};

pub fn is_integer(ty: &CheckedType) -> bool {
    use CheckedTypeKind::*;
    matches!(
        ty.kind,
        I8 | I16 | I32 | I64 | U8 | U16 | U32 | U64 | ISize | USize
    )
}

```

`src/check/utils/is_signed.rs`:

```rs
use crate::ast::checked::checked_type::{CheckedType, CheckedTypeKind};

pub fn is_signed(ty: &CheckedType) -> bool {
    use CheckedTypeKind::*;
    matches!(ty.kind, I8 | I16 | I32 | I64 | ISize | F32 | F64)
}

```

`src/check/utils/mod.rs`:

```rs
pub mod check_binary_numeric_operation;
pub mod check_is_assignable;
pub mod check_returns;
pub mod get_numeric_type_rank;
pub mod infer_generics;
pub mod is_float;
pub mod is_integer;
pub mod is_signed;
pub mod substitute_generics;
pub mod type_annotation_to_semantic;
pub mod union_of;

```

`src/check/utils/substitute_generics.rs`:

```rs
use std::collections::HashMap;

use crate::{
    ast::checked::{
        checked_declaration::{CheckedParam, StructDecl, TypeAliasDecl},
        checked_type::{CheckedType, CheckedTypeKind},
    },
    check::{SemanticError, SemanticErrorKind},
};

use super::union_of::union_of;

pub type GenericSubstitutionMap = HashMap<String, CheckedType>;

pub fn substitute_generics(
    ty: &CheckedType,
    substitution: &GenericSubstitutionMap,
    errors: &mut Vec<SemanticError>,
) -> CheckedType {
    match &ty.kind {
        CheckedTypeKind::GenericParam(gp) => substitution
            .get(&gp.identifier.name)
            .cloned()
            .unwrap_or_else(|| {
                let span = ty.unwrap_annotation_span();

                errors.push(SemanticError::new(
                    SemanticErrorKind::UnresolvedGenericParam(gp.identifier.name.clone()),
                    span,
                ));

                CheckedType {
                    kind: CheckedTypeKind::Unknown,
                    span: ty.span,
                }
            }),
        CheckedTypeKind::GenericFnType {
            params,
            return_type,
            generic_params: _, // not needed
        } => {
            // IMPORTANT: When substituting within a function type, we DON'T
            // substitute its *own* generic parameters.
            // We only substitute types that came from an outer scope's substitution.
            let substituted_params = params
                .iter()
                .map(|p| CheckedParam {
                    identifier: p.identifier.clone(),
                    constraint: substitute_generics(&p.constraint, substitution, errors),
                })
                .collect();

            let substituted_return_type = substitute_generics(return_type, substitution, errors);

            CheckedType {
                kind: CheckedTypeKind::FnType {
                    params: substituted_params,
                    return_type: Box::new(substituted_return_type),
                },
                span: ty.span,
            }
        }
        CheckedTypeKind::FnType {
            params,
            return_type,
        } => {
            // This case could be needed when a closure uses generic parameter which was defined by parent

            let substituted_params = params
                .iter()
                .map(|p| CheckedParam {
                    identifier: p.identifier.clone(),
                    constraint: substitute_generics(&p.constraint, substitution, errors),
                })
                .collect();

            let substituted_return_type = substitute_generics(return_type, substitution, errors);

            CheckedType {
                kind: CheckedTypeKind::FnType {
                    params: substituted_params,
                    return_type: Box::new(substituted_return_type),
                },
                span: ty.span,
            }
        }
        CheckedTypeKind::GenericStructDecl(decl) => {
            // Similar to FnType, a struct definition's generic params are local.
            // We substitute types *within* its properties if those types refer
            // to generics from the *outer* substitution context.
            let substituted_props = decl
                .properties
                .iter()
                .map(|p| CheckedParam {
                    identifier: p.identifier.clone(),
                    constraint: substitute_generics(&p.constraint, substitution, errors),
                })
                .collect();

            CheckedType {
                kind: CheckedTypeKind::StructDecl(StructDecl {
                    properties: substituted_props,
                    documentation: decl.documentation.clone(),
                    identifier: decl.identifier.clone(), // maybe we should rename this?
                }),
                span: ty.span,
            }
        }
        CheckedTypeKind::GenericTypeAliasDecl(decl) => {
            let substituted_value = substitute_generics(&decl.value, substitution, errors);

            CheckedType {
                kind: CheckedTypeKind::TypeAliasDecl(TypeAliasDecl {
                    value: Box::new(substituted_value),
                    documentation: decl.documentation.clone(),
                    identifier: decl.identifier.clone(), // maybe we should rename this?
                }),
                span: ty.span,
            }
        }
        CheckedTypeKind::Array { item_type, size } => CheckedType {
            kind: CheckedTypeKind::Array {
                item_type: Box::new(substitute_generics(item_type, substitution, errors)),
                size: *size,
            },
            span: ty.span,
        },
        CheckedTypeKind::Union(items) => {
            let substituted_items = items
                .iter()
                .map(|t| substitute_generics(t, substitution, errors));

            // Re-apply union_of logic to simplify the result
            union_of(substituted_items)
        }
        CheckedTypeKind::I8
        | CheckedTypeKind::I16
        | CheckedTypeKind::I32
        | CheckedTypeKind::I64
        | CheckedTypeKind::ISize
        | CheckedTypeKind::U8
        | CheckedTypeKind::U16
        | CheckedTypeKind::U32
        | CheckedTypeKind::U64
        | CheckedTypeKind::USize
        | CheckedTypeKind::F32
        | CheckedTypeKind::F64
        | CheckedTypeKind::Bool
        | CheckedTypeKind::Char
        | CheckedTypeKind::Void
        | CheckedTypeKind::Null
        | CheckedTypeKind::Unknown
        | CheckedTypeKind::TypeAliasDecl(_)
        | CheckedTypeKind::StructDecl(_)
        | CheckedTypeKind::Enum(_) => ty.clone(),
    }
}

```

`src/check/utils/type_annotation_to_semantic.rs`:

```rs
use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{
        base::base_type::{TypeAnnotation, TypeAnnotationKind},
        checked::{
            checked_declaration::{CheckedParam, GenericStructDecl, GenericTypeAliasDecl},
            checked_type::{CheckedType, CheckedTypeKind, TypeSpan},
        },
    },
    check::{
        check_stmt::check_generic_params,
        scope::{Scope, ScopeKind, SymbolEntry},
        SemanticError, SemanticErrorKind,
    },
    tokenizer::NumberKind,
};

pub fn check_type(
    arg: &TypeAnnotation,
    errors: &mut Vec<SemanticError>,
    scope: Rc<RefCell<Scope>>,
) -> CheckedType {
    let kind = match &arg.kind {
        TypeAnnotationKind::Void => CheckedTypeKind::Void,
        TypeAnnotationKind::Null => CheckedTypeKind::Null,
        TypeAnnotationKind::Bool => CheckedTypeKind::Bool,
        TypeAnnotationKind::U8 => CheckedTypeKind::U8,
        TypeAnnotationKind::U16 => CheckedTypeKind::U16,
        TypeAnnotationKind::U32 => CheckedTypeKind::U32,
        TypeAnnotationKind::U64 => CheckedTypeKind::U64,
        TypeAnnotationKind::USize => CheckedTypeKind::USize,
        TypeAnnotationKind::ISize => CheckedTypeKind::ISize,
        TypeAnnotationKind::I8 => CheckedTypeKind::I8,
        TypeAnnotationKind::I16 => CheckedTypeKind::I16,
        TypeAnnotationKind::I32 => CheckedTypeKind::I32,
        TypeAnnotationKind::I64 => CheckedTypeKind::I64,
        TypeAnnotationKind::F32 => CheckedTypeKind::F32,
        TypeAnnotationKind::F64 => CheckedTypeKind::F64,
        TypeAnnotationKind::Char => CheckedTypeKind::Char,
        TypeAnnotationKind::GenericApply { left, args } => {
            let checked_target = check_type(&left, errors, scope.clone());
            let checked_args = args
                .into_iter()
                .map(|arg| check_type(&arg, errors, scope.clone()))
                .collect::<Vec<CheckedType>>();

            match checked_target.kind {
                CheckedTypeKind::GenericFnType {
                    params,
                    return_type,
                    generic_params,
                } => {
                    todo!("Return specialized type")
                }
                CheckedTypeKind::GenericStructDecl(GenericStructDecl {
                    identifier,
                    generic_params,
                    documentation,
                    properties,
                }) => {
                    todo!("Return specialized type")
                }
                CheckedTypeKind::GenericTypeAliasDecl(GenericTypeAliasDecl {
                    identifier,
                    generic_params,
                    documentation,
                    value,
                }) => {
                    todo!("Return specialized type")
                }
                _ => {
                    todo!("Push an error when target is non generic type")
                }
            }
        }
        TypeAnnotationKind::Identifier(id) => scope
            .borrow()
            .lookup(&id.name)
            .map(|entry| match entry {
                SymbolEntry::GenericStructDecl(decl) => CheckedTypeKind::GenericStructDecl(decl),
                SymbolEntry::StructDecl(decl) => CheckedTypeKind::StructDecl(decl),
                SymbolEntry::EnumDecl(decl) => CheckedTypeKind::Enum(decl),
                SymbolEntry::GenericTypeAliasDecl(decl) => {
                    CheckedTypeKind::GenericTypeAliasDecl(decl)
                }
                SymbolEntry::TypeAliasDecl(decl) => CheckedTypeKind::TypeAliasDecl(decl),
                SymbolEntry::GenericParam(generic_param) => {
                    CheckedTypeKind::GenericParam(generic_param)
                }
                SymbolEntry::VarDecl(_) => {
                    errors.push(SemanticError::new(
                        SemanticErrorKind::CannotUseVariableDeclarationAsType,
                        arg.span,
                    ));
                    CheckedTypeKind::Unknown
                }
            })
            .unwrap_or_else(|| {
                errors.push(SemanticError::new(
                    SemanticErrorKind::UndeclaredType(id.name.clone()),
                    arg.span,
                ));
                CheckedTypeKind::Unknown
            }),

        TypeAnnotationKind::GenericFnType {
            params,
            return_type,
            generic_params,
        } => {
            let fn_type_scope = scope.borrow().child(ScopeKind::FnType);

            let checked_generic_params =
                check_generic_params(&generic_params, errors, fn_type_scope.clone());

            let checked_params = params
                .into_iter()
                .map(|p| CheckedParam {
                    constraint: check_type(&p.constraint, errors, fn_type_scope.clone()),
                    identifier: p.identifier.clone(),
                })
                .collect();

            CheckedTypeKind::GenericFnType {
                params: checked_params,
                return_type: Box::new(check_type(&return_type, errors, fn_type_scope.clone())),
                generic_params: checked_generic_params,
            }
        }
        TypeAnnotationKind::FnType {
            params,
            return_type,
        } => {
            let fn_type_scope = scope.borrow().child(ScopeKind::FnType);

            let checked_params = params
                .into_iter()
                .map(|p| CheckedParam {
                    constraint: check_type(&p.constraint, errors, fn_type_scope.clone()),
                    identifier: p.identifier.clone(),
                })
                .collect();

            CheckedTypeKind::FnType {
                params: checked_params,
                return_type: Box::new(check_type(&return_type, errors, fn_type_scope.clone())),
            }
        }
        TypeAnnotationKind::Union(items) => CheckedTypeKind::Union(
            items
                .iter()
                .map(|i| check_type(&i, errors, scope.clone()))
                .collect(),
        ),
        TypeAnnotationKind::Array { left, size } => {
            let maybe_size: Option<usize> = match size {
                &NumberKind::USize(v) => Some(v),
                &NumberKind::U64(v) => v.try_into().ok(),
                &NumberKind::U32(v) => v.try_into().ok(),
                &NumberKind::U16(v) => Some(v as usize),
                &NumberKind::U8(v) => Some(v as usize),

                &NumberKind::ISize(v) => v.try_into().ok(),
                &NumberKind::I64(v) => v.try_into().ok(),
                &NumberKind::I32(v) => v.try_into().ok(),
                &NumberKind::I16(v) => v.try_into().ok(),
                &NumberKind::I8(v) => v.try_into().ok(),

                &NumberKind::F32(_) | &NumberKind::F64(_) => None,
            };

            match maybe_size {
                Some(valid_size) => {
                    let item_type = check_type(&left, errors, scope.clone());
                    CheckedTypeKind::Array {
                        item_type: Box::new(item_type),
                        size: valid_size,
                    }
                }
                None => {
                    errors.push(SemanticError::new(
                        SemanticErrorKind::InvalidArraySizeValue(*size),
                        arg.span,
                    ));
                    let _ = check_type(&left, errors, scope.clone()); // Process for errors, ignore result
                    CheckedTypeKind::Unknown
                }
            }
        }
        TypeAnnotationKind::Error(_) => CheckedTypeKind::Unknown,
    };

    CheckedType {
        kind,
        span: TypeSpan::Annotation(arg.span),
    }
}

```

`src/check/utils/union_of.rs`:

```rs
use std::collections::HashSet;

use crate::ast::checked::checked_type::{CheckedType, CheckedTypeKind, TypeSpan};

pub fn union_of(types: impl Iterator<Item = CheckedType>) -> CheckedType {
    let mut union_items: HashSet<CheckedType> = HashSet::new();

    for t in types {
        match t.kind {
            CheckedTypeKind::Union(items) => {
                union_items.extend(items);
            }
            _ => {
                union_items.insert(t);
            }
        };
    }

    CheckedType {
        kind: CheckedTypeKind::Union(union_items),
        span: TypeSpan::None,
    }
}

```

`src/compile/mod.rs`:

```rs
use crate::{
    check::{check_stmts::check_stmts, SemanticError},
    parse::{Parser, ParsingError},
    tokenizer::{TokenizationError, Tokenizer},
};

pub enum CompilationError {
    TokenizerError(TokenizationError),
    ParsingError(ParsingError),
    SemanticError(SemanticError),
    CodegenError(),
}

// pub fn compile(source_code: String) {
//     let tokens = Tokenizer::tokenize(source_code);
//     let parse_tree = Parser::parse(tokens);
//     let analyzed_tree = check_stmts(parse_tree, errors, scope);
// }

```

`src/lib.rs`:

```rs
pub mod ast;
pub mod check;
pub mod codegen;
pub mod compile;
pub mod parse;
pub mod tokenizer;

```

`src/main.rs`:

```rs
fn main() {
    println!("Hello, world!");
}

```

`src/parse/expressions/mod.rs`:

```rs
pub mod parse_codeblock_expr;
pub mod parse_fn_call_expr;
pub mod parse_fn_expr;
pub mod parse_if_expr;
pub mod parse_parenthesized_expr;
pub mod parse_struct_init_expr;

use crate::{
    ast::{
        base::base_expression::{Expr, ExprKind},
        Span,
    },
    tokenizer::{KeywordKind, PunctuationKind, Token, TokenKind},
};

use super::{Parser, ParsingError, ParsingErrorKind};

fn prefix_bp(token_kind: &TokenKind) -> Option<((), u8)> {
    use PunctuationKind::*;
    use TokenKind::*;

    let priority = match token_kind {
        Punctuation(Minus) | Punctuation(Not) => ((), 13),
        _ => return None,
    };

    Some(priority)
}

fn infix_bp(token_kind: &TokenKind) -> Option<(u8, u8)> {
    use PunctuationKind::*;
    use TokenKind::*;

    let priority = match token_kind {
        Punctuation(DoubleOr) => (1, 2),
        Punctuation(DoubleAnd) => (3, 4),
        Punctuation(DoubleEq) | Punctuation(NotEq) => (5, 6),
        Punctuation(Lt) | Punctuation(Lte) | Punctuation(Gt) | Punctuation(Gte) => (7, 8),
        Punctuation(Plus) | Punctuation(Minus) => (9, 10),
        Punctuation(Star) | Punctuation(Slash) | Punctuation(Percent) => (11, 12),
        _ => return None,
    };

    Some(priority)
}

fn suffix_bp(token_kind: &TokenKind) -> Option<(u8, ())> {
    use PunctuationKind::*;
    use TokenKind::*;

    let priority = match token_kind {
        Punctuation(LParen) | Punctuation(LBrace) => (14, ()), // fn call and struct init
        Punctuation(Dot) | Punctuation(DoubleCol) => (14, ()), // member/static accesses
        Punctuation(Lt) => (14, ()),                           // generic struct/fn call
        _ => return None,
    };

    Some(priority)
}

pub fn is_start_of_expr(token_kind: &TokenKind) -> bool {
    match token_kind {
        TokenKind::Identifier(_)
        | TokenKind::Number(_)
        | TokenKind::String(_)
        | TokenKind::Keyword(KeywordKind::True)
        | TokenKind::Keyword(KeywordKind::False)
        | TokenKind::Keyword(KeywordKind::Null)
        | TokenKind::Keyword(KeywordKind::If)               // if expressions
        | TokenKind::Punctuation(PunctuationKind::LParen)   // Parenthesized or fn expr
        | TokenKind::Punctuation(PunctuationKind::LBrace)   // Codeblock expr
        | TokenKind::Punctuation(PunctuationKind::LBracket) // Array literal
        | TokenKind::Punctuation(PunctuationKind::Lt)       // fn expression
        | TokenKind::Punctuation(PunctuationKind::Minus)    // Negation
        | TokenKind::Punctuation(PunctuationKind::And)      // Address-of
        | TokenKind::Punctuation(PunctuationKind::Not)      // Logical NOT
        => true,
        _ => false,
    }
}

impl Parser {
    pub fn parse_expr(&mut self, min_prec: u8) -> Result<Expr, ParsingError> {
        let token = self.current().ok_or(self.unexpected_end_of_input())?;

        let token_span = token.span;

        let mut lhs = match token {
            Token {
                kind: TokenKind::Identifier(_),
                ..
            } => {
                let id = self.consume_identifier()?;
                Expr {
                    kind: ExprKind::Identifier(id),
                    span: token_span.clone(),
                }
            }
            Token {
                kind: TokenKind::Number(_),
                ..
            } => {
                let number = self.consume_number()?;
                Expr {
                    kind: ExprKind::Number { value: number },
                    span: token_span.clone(),
                }
            }
            Token {
                kind: TokenKind::Punctuation(PunctuationKind::Lt),
                ..
            } => self.parse_fn_expr()?,
            Token {
                kind: TokenKind::Punctuation(PunctuationKind::LParen),
                ..
            } => {
                self.place_checkpoint();
                let result = self.parse_fn_expr().or_else(|_| {
                    self.goto_checkpoint();
                    self.parse_parenthesized_expr()
                    // TODO: report an error when all parsing attempts fail
                })?;

                result
            }
            Token {
                kind: TokenKind::Punctuation(PunctuationKind::LBrace),
                ..
            } => {
                let start_offset = self.offset;

                let block_contents = self.parse_codeblock_expr()?;

                Expr {
                    kind: ExprKind::Block(block_contents),
                    span: self.get_span(start_offset, self.offset - 1)?,
                }
            }
            Token {
                kind: TokenKind::Punctuation(PunctuationKind::LBracket),
                ..
            } => {
                let start_offset = self.offset;
                self.consume_punctuation(PunctuationKind::LBracket)?;
                let items: Vec<Expr> = self
                    .comma_separated(
                        |p| p.parse_expr(0),
                        |p| p.match_token(0, TokenKind::Punctuation(PunctuationKind::RBracket)),
                    )?
                    .into_iter()
                    .map(|item| item)
                    .collect();

                self.consume_punctuation(PunctuationKind::RBracket)?;
                let span = self.get_span(start_offset, self.offset - 1)?;

                Expr {
                    kind: ExprKind::ArrayLiteral { items },
                    span,
                }
            }
            Token {
                kind: TokenKind::Punctuation(PunctuationKind::Minus),
                ..
            } => {
                let ((), r_bp) =
                    prefix_bp(&TokenKind::Punctuation(PunctuationKind::Minus)).unwrap();
                let start_offset = self.offset;

                self.consume_punctuation(PunctuationKind::Minus)?;
                let expr = self.parse_expr(r_bp)?;
                Expr {
                    kind: ExprKind::Neg {
                        right: Box::new(expr),
                    },
                    span: self.get_span(start_offset, self.offset - 1)?,
                }
            }
            Token {
                kind: TokenKind::Punctuation(PunctuationKind::Not),
                ..
            } => {
                let ((), r_bp) = prefix_bp(&TokenKind::Punctuation(PunctuationKind::Not)).unwrap();
                let start_offset = self.offset;

                self.consume_punctuation(PunctuationKind::Not)?;
                let expr = self.parse_expr(r_bp)?;
                Expr {
                    kind: ExprKind::Not {
                        right: Box::new(expr),
                    },
                    span: self.get_span(start_offset, self.offset - 1)?,
                }
            }
            Token {
                kind: TokenKind::Keyword(KeywordKind::If),
                ..
            } => self.parse_if_expr()?,
            Token {
                kind: TokenKind::Keyword(KeywordKind::True),
                ..
            } => {
                let start_offset = self.offset;

                self.consume_keyword(KeywordKind::True)?;
                Expr {
                    kind: ExprKind::BoolLiteral { value: true },
                    span: self.get_span(start_offset, self.offset - 1)?,
                }
            }
            Token {
                kind: TokenKind::Keyword(KeywordKind::False),
                ..
            } => {
                let start_offset = self.offset;

                self.consume_keyword(KeywordKind::False)?;

                Expr {
                    kind: ExprKind::BoolLiteral { value: false },
                    span: self.get_span(start_offset, self.offset - 1)?,
                }
            }
            Token {
                kind: TokenKind::Keyword(KeywordKind::Null),
                ..
            } => {
                let start_offset = self.offset;

                self.consume_keyword(KeywordKind::Null)?;
                Expr {
                    kind: ExprKind::Null,
                    span: self.get_span(start_offset, self.offset - 1)?,
                }
            }
            Token {
                kind: TokenKind::String(_),
                ..
            } => {
                let start_offset = self.offset;

                let val = self.consume_string()?;
                Expr {
                    kind: ExprKind::String(val),
                    span: self.get_span(start_offset, self.offset - 1)?,
                }
            }
            t => {
                return Err(ParsingError::new(
                    ParsingErrorKind::ExpectedAnExpressionButFound(t.clone()),
                    t.span,
                ))
            }
        };

        loop {
            let op = match self.current() {
                Some(o) => o.clone(),
                None => break,
            };

            if let Some((left_prec, ())) = suffix_bp(&op.kind) {
                if left_prec < min_prec {
                    break;
                }
                let lhs_clone = lhs.clone();

                let new_lhs = match op.kind {
                    TokenKind::Punctuation(PunctuationKind::Dot) => {
                        let start_offset = self.offset;

                        self.consume_punctuation(PunctuationKind::Dot)?;
                        let field = self.consume_identifier()?;
                        Some(Expr {
                            kind: ExprKind::Access {
                                left: Box::new(lhs_clone),
                                field: field,
                            },
                            span: self.get_span(start_offset, self.offset - 1)?,
                        })
                    }
                    TokenKind::Punctuation(PunctuationKind::DoubleCol) => {
                        let start_offset = self.offset;

                        self.consume_punctuation(PunctuationKind::DoubleCol)?;
                        let field = self.consume_identifier()?;
                        Some(Expr {
                            kind: ExprKind::StaticAccess {
                                left: Box::new(lhs_clone),
                                field,
                            },
                            span: self.get_span(start_offset, self.offset - 1)?,
                        })
                    }
                    TokenKind::Punctuation(PunctuationKind::Lt) => {
                        self.place_checkpoint();

                        if let Ok((generic_args, generic_args_span)) =
                            self.parse_optional_generic_args()
                        {
                            Some(Expr {
                                kind: ExprKind::GenericApply {
                                    left: Box::new(lhs_clone),
                                    args: generic_args,
                                },
                                span: generic_args_span,
                            })
                        } else {
                            self.goto_checkpoint();
                            None
                        }
                    }
                    TokenKind::Punctuation(PunctuationKind::LParen) => {
                        if let ExprKind::StaticAccess { left, field } = lhs.kind.clone() {
                            if field.name == "as" || field.name == "is" {
                                let start_offset = self.offset;
                                self.consume_punctuation(PunctuationKind::LParen)?;
                                let target_type = self.parse_type_annotation(0)?;
                                self.consume_punctuation(PunctuationKind::RParen)?;
                                let span_end = self.get_span(start_offset, self.offset - 1)?;

                                let kind = if field.name == "as" {
                                    ExprKind::TypeCast {
                                        left,
                                        target: target_type,
                                    }
                                } else {
                                    ExprKind::IsType {
                                        left,
                                        target: target_type,
                                    }
                                };

                                Some(Expr {
                                    kind,
                                    span: Span {
                                        start: lhs.span.start,
                                        end: span_end.end,
                                    },
                                })
                            } else {
                                return Err(ParsingError::new(
                                    ParsingErrorKind::UnknownStaticMethod(field.to_owned()),
                                    field.span,
                                ));
                            }
                        } else {
                            Some(self.parse_fn_call_expr(lhs_clone)?)
                        }
                    }
                    TokenKind::Punctuation(PunctuationKind::LBrace) => {
                        Some(self.parse_struct_init_expr(lhs_clone)?)
                    }
                    _ => {
                        return Err(ParsingError::new(
                            ParsingErrorKind::InvalidSuffixOperator(op.clone()),
                            op.span,
                        ))
                    }
                };

                if let Some(expr) = new_lhs {
                    lhs = expr;
                    continue;
                }
            }

            if let Some((left_prec, right_prec)) = infix_bp(&op.kind) {
                if left_prec < min_prec {
                    break;
                }

                let start_pos = lhs.span.start;

                self.advance();

                let rhs = self.parse_expr(right_prec)?;

                let end_pos = rhs.span.end;

                let expr_kind = match op.kind {
                    TokenKind::Punctuation(PunctuationKind::Plus) => ExprKind::Add {
                        left: Box::new(lhs),
                        right: Box::new(rhs),
                    },
                    TokenKind::Punctuation(PunctuationKind::Minus) => ExprKind::Subtract {
                        left: Box::new(lhs),
                        right: Box::new(rhs),
                    },
                    TokenKind::Punctuation(PunctuationKind::Star) => ExprKind::Multiply {
                        left: Box::new(lhs),
                        right: Box::new(rhs),
                    },
                    TokenKind::Punctuation(PunctuationKind::Slash) => ExprKind::Divide {
                        left: Box::new(lhs),
                        right: Box::new(rhs),
                    },
                    TokenKind::Punctuation(PunctuationKind::Percent) => ExprKind::Modulo {
                        left: Box::new(lhs),
                        right: Box::new(rhs),
                    },
                    TokenKind::Punctuation(PunctuationKind::Lt) => ExprKind::LessThan {
                        left: Box::new(lhs),
                        right: Box::new(rhs),
                    },
                    TokenKind::Punctuation(PunctuationKind::Lte) => ExprKind::LessThanOrEqual {
                        left: Box::new(lhs),
                        right: Box::new(rhs),
                    },
                    TokenKind::Punctuation(PunctuationKind::Gt) => ExprKind::GreaterThan {
                        left: Box::new(lhs),
                        right: Box::new(rhs),
                    },
                    TokenKind::Punctuation(PunctuationKind::Gte) => ExprKind::GreaterThanOrEqual {
                        left: Box::new(lhs),
                        right: Box::new(rhs),
                    },
                    TokenKind::Punctuation(PunctuationKind::DoubleEq) => ExprKind::Equal {
                        left: Box::new(lhs),
                        right: Box::new(rhs),
                    },
                    TokenKind::Punctuation(PunctuationKind::NotEq) => ExprKind::NotEqual {
                        left: Box::new(lhs),
                        right: Box::new(rhs),
                    },
                    TokenKind::Punctuation(PunctuationKind::DoubleAnd) => ExprKind::And {
                        left: Box::new(lhs),
                        right: Box::new(rhs),
                    },
                    TokenKind::Punctuation(PunctuationKind::DoubleOr) => ExprKind::Or {
                        left: Box::new(lhs),
                        right: Box::new(rhs),
                    },
                    _ => break,
                };

                lhs = Expr {
                    kind: expr_kind,
                    span: Span {
                        start: start_pos,
                        end: end_pos,
                    },
                };

                continue;
            }

            break;
        }

        Ok(lhs)
    }

    fn synchronize_expr(&mut self) {
        loop {
            match self.current() {
                Some(token) => {
                    if is_start_of_expr(&token.kind) {
                        return;
                    }
                    if token.kind == TokenKind::Punctuation(PunctuationKind::SemiCol) {
                        self.advance();
                        return;
                    }
                    if token.kind == TokenKind::Punctuation(PunctuationKind::RBrace) {
                        self.advance();
                        return;
                    }
                    if token.kind == TokenKind::Punctuation(PunctuationKind::RParen) {
                        self.advance();
                        return;
                    }
                    if token.kind == TokenKind::Punctuation(PunctuationKind::RBracket) {
                        self.advance();
                        return;
                    }
                    if token.kind == TokenKind::Punctuation(PunctuationKind::Comma) {
                        self.advance();
                        return;
                    }

                    self.advance();
                }
                None => return,
            }
        }
    }
}

```

`src/parse/expressions/parse_codeblock_expr.rs`:

```rs
use crate::{
    ast::{
        base::{
            base_expression::BlockContents,
            base_statement::{Stmt, StmtKind},
        },
        Span,
    },
    parse::{statements::is_start_of_stmt, Parser, ParsingError, ParsingErrorKind},
    tokenizer::{PunctuationKind, TokenKind},
};

use super::is_start_of_expr;

impl Parser {
    pub fn parse_codeblock_expr(&mut self) -> Result<BlockContents, ParsingError> {
        self.consume_punctuation(PunctuationKind::LBrace)?;

        let mut statements = Vec::new();
        let mut final_expr = None;

        loop {
            if self.match_token(0, TokenKind::Punctuation(PunctuationKind::RBrace)) {
                break;
            }

            let current_token = self
                .current()
                .cloned()
                .ok_or_else(|| self.unexpected_end_of_input())?;
            let current_token_span = current_token.span;

            if is_start_of_stmt(&current_token.kind) {
                if final_expr.is_some() {
                    return Err(ParsingError::new(
                        ParsingErrorKind::UnexpectedStatementAfterFinalExpression,
                        current_token_span,
                    ));
                }

                let stmt = self.parse_stmt().map_err(|e| {
                    self.synchronize_stmt();
                    e
                })?;
                statements.push(stmt);
                final_expr = None;
            } else if is_start_of_expr(&current_token.kind) {
                if final_expr.is_some() {
                    return Err(ParsingError::new(
                        ParsingErrorKind::UnexpectedTokenAfterFinalExpression {
                            found: current_token.kind.clone(),
                        },
                        current_token_span,
                    ));
                }

                let expr = self.parse_expr(0).map_err(|e| {
                    self.synchronize_expr();
                    e
                })?;

                if self.match_token(0, TokenKind::Punctuation(PunctuationKind::SemiCol)) {
                    let semi_offset = self.offset;
                    self.advance();
                    let end_span = self.get_span(semi_offset, self.offset - 1)?;

                    statements.push(Stmt {
                        span: Span {
                            start: expr.span.start,
                            end: end_span.end,
                        },
                        kind: StmtKind::Expression(expr),
                    });

                    final_expr = None;
                } else {
                    final_expr = Some(Box::new(expr));
                }
            } else {
                return Err(ParsingError::new(
                    ParsingErrorKind::ExpectedStatementOrExpression {
                        found: current_token.kind.clone(),
                    },
                    current_token_span,
                ));
            }

            if final_expr.is_some()
                && !self.match_token(0, TokenKind::Punctuation(PunctuationKind::RBrace))
            {
                let unexpected_token = self
                    .current()
                    .cloned()
                    .ok_or_else(|| self.unexpected_end_of_input())?;
                return Err(ParsingError::new(
                    ParsingErrorKind::UnexpectedTokenAfterFinalExpression {
                        found: unexpected_token.kind.clone(),
                    },
                    unexpected_token.span,
                ));
            }
        }

        self.consume_punctuation(PunctuationKind::RBrace)?;

        Ok(BlockContents {
            statements,
            final_expr,
        })
    }
}

```

`src/parse/expressions/parse_fn_call_expr.rs`:

```rs
use crate::{
    ast::base::base_expression::{Expr, ExprKind},
    parse::{Parser, ParsingError},
    tokenizer::{PunctuationKind, TokenKind},
};

impl Parser {
    pub fn parse_fn_call_args(&mut self) -> Result<Vec<Expr>, ParsingError> {
        self.consume_punctuation(PunctuationKind::LParen)?;
        let args = self.comma_separated(
            |p| p.parse_expr(0),
            |p| p.match_token(0, TokenKind::Punctuation(PunctuationKind::RParen)),
        );
        self.consume_punctuation(PunctuationKind::RParen)?;
        args
    }

    pub fn parse_fn_call_expr(&mut self, left: Expr) -> Result<Expr, ParsingError> {
        let start_offset = self.offset;

        let args = self.parse_fn_call_args()?;
        let mut span = left.span;
        let end = self.get_span(start_offset, self.offset - 1)?;
        span.end = end.end;

        Ok(Expr {
            kind: ExprKind::FnCall {
                left: Box::new(left),
                args,
            },
            span,
        })
    }
}

```

`src/parse/expressions/parse_fn_expr.rs`:

```rs
use crate::{
    ast::base::{
        base_declaration::Param,
        base_expression::{Expr, ExprKind},
    },
    parse::{Parser, ParsingError},
    tokenizer::{PunctuationKind, TokenKind},
};

impl Parser {
    pub fn parse_fn_expr(&mut self) -> Result<Expr, ParsingError> {
        let start_offset = self.offset;

        let generic_params = self.parse_optional_generic_params()?;
        self.consume_punctuation(PunctuationKind::LParen)?;
        let params = self.comma_separated(
            |p| {
                let identifier = p.consume_identifier()?;
                p.consume_punctuation(PunctuationKind::Col)?;
                let constraint = p.parse_type_annotation(0)?;

                Ok(Param {
                    constraint,
                    identifier,
                })
            },
            |p| p.match_token(0, TokenKind::Punctuation(PunctuationKind::RParen)),
        )?;
        self.consume_punctuation(PunctuationKind::RParen)?;

        let return_type = if self.match_token(0, TokenKind::Punctuation(PunctuationKind::Col)) {
            self.advance();

            let return_type = self.parse_type_annotation(0)?;
            Some(return_type)
        } else {
            None
        };

        self.consume_punctuation(PunctuationKind::FatArrow)?;

        let body = self.parse_codeblock_expr()?;

        Ok(Expr {
            kind: ExprKind::Fn {
                params,
                body,
                return_type,
                generic_params,
            },
            span: self.get_span(start_offset, self.offset - 1)?,
        })
    }
}

#[cfg(test)]
mod test {
    use super::Parser;
    use crate::{
        ast::{
            base::{
                base_declaration::{GenericParam, Param},
                base_expression::{BlockContents, Expr, ExprKind},
                base_type::{TypeAnnotation, TypeAnnotationKind},
            },
            IdentifierNode, Position, Span,
        },
        tokenizer::Tokenizer,
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn parses_basic_function() {
        let tokens = Tokenizer::tokenize("() => {}".to_owned());
        let mut parser = Parser {
            checkpoint_offset: 0,
            offset: 0,
            tokens,
        };
        let actual_ast = parser.parse_expr(0);
        let expected_ast = Ok(Expr {
            kind: ExprKind::Fn {
                params: vec![],
                body: BlockContents {
                    final_expr: None,
                    statements: vec![],
                },
                return_type: None,
                generic_params: vec![],
            },
            span: Span {
                start: Position { line: 1, col: 1 },
                end: Position { line: 1, col: 9 },
            },
        });
        assert_eq!(actual_ast, expected_ast)
    }

    #[test]
    fn parses_function_with_params() {
        let tokens = Tokenizer::tokenize("(const a: i32) => {}".to_owned());
        let mut parser = Parser {
            checkpoint_offset: 0,
            offset: 0,
            tokens,
        };
        let actual_ast = parser.parse_expr(0);
        let expected_ast = Ok(Expr {
            kind: ExprKind::Fn {
                params: vec![Param {
                    identifier: IdentifierNode {
                        name: String::from("a"),
                        span: Span {
                            start: Position { line: 0, col: 0 },
                            end: Position { line: 0, col: 0 },
                        },
                    },
                    constraint: TypeAnnotation {
                        kind: TypeAnnotationKind::I32,
                        span: Span {
                            start: Position { line: 1, col: 5 },
                            end: Position { line: 1, col: 8 },
                        },
                    },
                }],
                body: BlockContents {
                    final_expr: None,
                    statements: vec![],
                },
                return_type: None,
                generic_params: vec![],
            },
            span: Span {
                start: Position { line: 1, col: 1 },
                end: Position { line: 1, col: 15 },
            },
        });
        assert_eq!(actual_ast, expected_ast)
    }

    #[test]
    fn parses_function_with_generic_params() {
        let tokens = Tokenizer::tokenize("<AParam>(a: AParam) => {}".to_owned());
        let mut parser = Parser {
            checkpoint_offset: 0,
            offset: 0,
            tokens,
        };
        let actual_ast = parser.parse_expr(0);
        let expected_ast = Ok(Expr {
            kind: ExprKind::Fn {
                params: vec![Param {
                    identifier: IdentifierNode {
                        name: String::from("a"),
                        span: Span {
                            start: Position { line: 0, col: 0 },
                            end: Position { line: 0, col: 0 },
                        },
                    },
                    constraint: TypeAnnotation {
                        kind: TypeAnnotationKind::Identifier(IdentifierNode {
                            name: String::from("AParam"),
                            span: Span {
                                start: Position { line: 0, col: 0 },
                                end: Position { line: 0, col: 0 },
                            },
                        }),
                        span: Span {
                            start: Position { line: 1, col: 13 },
                            end: Position { line: 1, col: 19 },
                        },
                    },
                }],
                body: BlockContents {
                    final_expr: None,
                    statements: vec![],
                },
                return_type: None,
                generic_params: vec![GenericParam {
                    constraint: None,
                    identifier: IdentifierNode {
                        name: String::from("AParam"),
                        span: Span {
                            start: Position { line: 0, col: 0 },
                            end: Position { line: 0, col: 0 },
                        },
                    },
                }],
            },
            span: Span {
                start: Position { line: 1, col: 1 },
                end: Position { line: 1, col: 26 },
            },
        });
        assert_eq!(actual_ast, expected_ast)
    }

    #[test]
    fn parses_function_with_return_type() {
        let tokens = Tokenizer::tokenize("<AParam>(a: AParam): i32 => {}".to_owned());
        let mut parser = Parser {
            checkpoint_offset: 0,
            offset: 0,
            tokens,
        };
        let actual_ast = parser.parse_expr(0);
        let expected_ast = Ok(Expr {
            kind: ExprKind::Fn {
                params: vec![Param {
                    identifier: IdentifierNode {
                        name: String::from("a"),
                        span: Span {
                            start: Position { line: 0, col: 0 },
                            end: Position { line: 0, col: 0 },
                        },
                    },
                    constraint: TypeAnnotation {
                        kind: TypeAnnotationKind::Identifier(IdentifierNode {
                            name: String::from("AParam"),
                            span: Span {
                                start: Position { line: 0, col: 0 },
                                end: Position { line: 0, col: 0 },
                            },
                        }),
                        span: Span {
                            start: Position { line: 1, col: 13 },
                            end: Position { line: 1, col: 19 },
                        },
                    },
                }],
                body: BlockContents {
                    final_expr: None,
                    statements: vec![],
                },
                return_type: Some(TypeAnnotation {
                    kind: TypeAnnotationKind::I32,
                    span: Span {
                        start: Position { line: 1, col: 22 },
                        end: Position { line: 1, col: 25 },
                    },
                }),
                generic_params: vec![GenericParam {
                    constraint: None,
                    identifier: IdentifierNode {
                        name: String::from("AParam"),
                        span: Span {
                            start: Position { line: 0, col: 0 },
                            end: Position { line: 0, col: 0 },
                        },
                    },
                }],
            },
            span: Span {
                start: Position { line: 1, col: 1 },
                end: Position { line: 1, col: 31 },
            },
        });
        assert_eq!(actual_ast, expected_ast)
    }
}

```

`src/parse/expressions/parse_if_expr.rs`:

```rs
use crate::{
    ast::base::base_expression::{Expr, ExprKind},
    parse::{Parser, ParsingError},
    tokenizer::{KeywordKind, TokenKind},
};

impl Parser {
    pub fn parse_if_expr(&mut self) -> Result<Expr, ParsingError> {
        let start_offset = self.offset;

        self.consume_keyword(KeywordKind::If)?;

        let condition = self.parse_expr(0)?;
        let then_branch = self.parse_codeblock_expr()?;

        let mut else_if_branches = Vec::new();
        while self.match_token(0, TokenKind::Keyword(KeywordKind::Else))
            && self.match_token(1, TokenKind::Keyword(KeywordKind::If))
        {
            self.advance();
            self.advance();

            let else_if_condition = self.parse_expr(0)?;
            let else_if_body = self.parse_codeblock_expr()?;
            else_if_branches.push((Box::new(else_if_condition), else_if_body));
        }

        let else_branch = if self.match_token(0, TokenKind::Keyword(KeywordKind::Else)) {
            self.advance();

            let else_body = self.parse_codeblock_expr()?;
            Some(else_body)
        } else {
            None
        };

        Ok(Expr {
            kind: ExprKind::If {
                condition: Box::new(condition),
                then_branch,
                else_if_branches,
                else_branch,
            },
            span: self.get_span(start_offset, self.offset - 1)?,
        })
    }
}

```

`src/parse/expressions/parse_parenthesized_expr.rs`:

```rs
use crate::{
    ast::base::base_expression::Expr,
    parse::{Parser, ParsingError},
    tokenizer::PunctuationKind,
};

impl Parser {
    pub fn parse_parenthesized_expr(&mut self) -> Result<Expr, ParsingError> {
        let start_offset = self.offset;

        self.consume_punctuation(PunctuationKind::LParen)?;
        let expr = self.parse_expr(0)?;
        self.consume_punctuation(PunctuationKind::RParen)?;

        let span = self.get_span(start_offset, self.offset - 1)?;

        Ok(Expr {
            kind: expr.kind,
            span,
        })
    }
}

```

`src/parse/expressions/parse_struct_init_expr.rs`:

```rs
use crate::{
    ast::{
        base::base_expression::{Expr, ExprKind},
        IdentifierNode,
    },
    parse::{Parser, ParsingError},
    tokenizer::{PunctuationKind, TokenKind},
};

impl Parser {
    pub fn parse_struct_init_fields(
        &mut self,
    ) -> Result<Vec<(IdentifierNode, Expr)>, ParsingError> {
        self.consume_punctuation(PunctuationKind::LBrace)?;
        let args = self.comma_separated(
            |p| {
                let name = p.consume_identifier()?;
                p.consume_punctuation(PunctuationKind::Col)?;
                let value = p.parse_expr(0)?;
                Ok((name, value))
            },
            |p| p.match_token(0, TokenKind::Punctuation(PunctuationKind::RBrace)),
        )?;
        self.consume_punctuation(PunctuationKind::RBrace)?;
        Ok(args)
    }

    pub fn parse_struct_init_expr(&mut self, left: Expr) -> Result<Expr, ParsingError> {
        let start_offset = self.offset;

        let mut span = left.span;
        let fields = self.parse_struct_init_fields()?;
        let span_end = self.get_span(start_offset, self.offset - 1)?;
        span.end = span_end.end;

        Ok(Expr {
            kind: ExprKind::StructInit {
                left: Box::new(left),
                fields,
            },
            span,
        })
    }
}

```

`src/parse/mod.rs`:

```rs
mod expressions;
mod parse_generic_args;
mod parse_generic_params;
mod statements;
mod type_annotations;

pub struct Parser {
    pub offset: usize,
    pub tokens: Vec<Token>,
    pub checkpoint_offset: usize,
}

use crate::{
    ast::{
        base::base_statement::{Stmt, StmtKind},
        IdentifierNode, Position, Span, StringNode,
    },
    tokenizer::{KeywordKind, NumberKind, PunctuationKind, Token, TokenKind},
};

#[derive(Debug, Clone, PartialEq)]
pub enum ParsingErrorKind {
    DocMustBeFollowedByDeclaration,
    ExpectedNumberOfArguments(usize),
    ExpectedAnExpressionButFound(Token),
    ExpectedATypeButFound(Token),
    InvalidTypeOperator(Token),
    InvalidPrefixOperator(Token),
    InvalidSuffixOperator(Token),
    InvalidInfixOperator(Token),
    InvalidArraySize,
    InvalidArrayIndex,
    UnexpectedToken(Token),
    InvalidImportPath,
    InvalidDocumentationString,
    MissingElseBranch,
    UnexpectedEndOfInput,
    ExpectedAnIdentifier,
    ExpectedAPunctuationMark(PunctuationKind),
    ExpectedAKeyword(KeywordKind),
    ExpectedAStringValue,
    ExpectedANumericValue,
    UnknownStaticMethod(IdentifierNode),
    UnexpectedStatementAfterFinalExpression,
    ExpectedStatementOrExpression { found: TokenKind },
    UnexpectedTokenAfterFinalExpression { found: TokenKind },
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParsingError {
    pub kind: ParsingErrorKind,
    pub span: Span,
    code: usize,
}

impl ParsingError {
    fn kind_to_code(kind: &ParsingErrorKind) -> usize {
        match kind {
            ParsingErrorKind::DocMustBeFollowedByDeclaration { .. } => 1,
            ParsingErrorKind::ExpectedNumberOfArguments(..) => 2,
            ParsingErrorKind::ExpectedAnExpressionButFound(..) => 3,
            ParsingErrorKind::ExpectedATypeButFound(..) => 4,
            ParsingErrorKind::InvalidTypeOperator(..) => 5,
            ParsingErrorKind::InvalidPrefixOperator(..) => 6,
            ParsingErrorKind::InvalidSuffixOperator(..) => 7,
            ParsingErrorKind::InvalidInfixOperator(..) => 8,
            ParsingErrorKind::InvalidArraySize => 9,
            ParsingErrorKind::InvalidArrayIndex => 10,
            ParsingErrorKind::UnexpectedToken(..) => 11,
            ParsingErrorKind::InvalidImportPath => 12,
            ParsingErrorKind::InvalidDocumentationString => 13,
            ParsingErrorKind::MissingElseBranch => 14,
            ParsingErrorKind::UnexpectedEndOfInput => 15,
            ParsingErrorKind::ExpectedAnIdentifier => 16,
            ParsingErrorKind::ExpectedAPunctuationMark(..) => 17,
            ParsingErrorKind::ExpectedAKeyword(..) => 18,
            ParsingErrorKind::ExpectedAStringValue => 19,
            ParsingErrorKind::ExpectedANumericValue => 20,
            ParsingErrorKind::UnknownStaticMethod(..) => 21,
            ParsingErrorKind::UnexpectedStatementAfterFinalExpression => 22,
            ParsingErrorKind::ExpectedStatementOrExpression { .. } => 23,
            ParsingErrorKind::UnexpectedTokenAfterFinalExpression { .. } => 24,
        }
    }

    fn new(kind: ParsingErrorKind, span: Span) -> ParsingError {
        ParsingError {
            code: Self::kind_to_code(&kind),
            kind,
            span,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DocAnnotation {
    message: String,
    span: Span,
}

impl Parser {
    fn match_token(&self, index: usize, kind: TokenKind) -> bool {
        if let Some(token) = self.tokens.get(self.offset + index) {
            return token.kind == kind;
        }

        false
    }

    fn advance(&mut self) {
        self.offset += 1;
    }

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.offset)
    }

    fn unexpected_end_of_input(&self) -> ParsingError {
        let first_token_span = Span {
            start: Position { line: 1, col: 1 },
            end: Position { line: 1, col: 1 },
        };

        let last_token_span = self
            .tokens
            .last()
            .map(|t| &t.span)
            .unwrap_or(&first_token_span);

        ParsingError::new(
            ParsingErrorKind::UnexpectedEndOfInput,
            last_token_span.clone(),
        )
    }

    fn get_span(&mut self, start_offset: usize, end_offset: usize) -> Result<Span, ParsingError> {
        let start = self
            .tokens
            .get(start_offset)
            .ok_or(self.unexpected_end_of_input())?;

        let end = self
            .tokens
            .get(end_offset)
            .ok_or(self.unexpected_end_of_input())?;

        Ok(Span {
            start: start.span.start,
            end: end.span.end,
        })
    }

    fn place_checkpoint(&mut self) {
        self.checkpoint_offset = self.offset;
    }

    fn goto_checkpoint(&mut self) {
        self.offset = self.checkpoint_offset;
    }

    pub fn consume_string(&mut self) -> Result<StringNode, ParsingError> {
        if let Some(t) = self.current() {
            match t.clone() {
                Token {
                    kind: TokenKind::String(value),
                    span,
                } => {
                    self.advance();

                    Ok(StringNode { span, value })
                }
                t => {
                    return Err(ParsingError::new(
                        ParsingErrorKind::ExpectedAStringValue,
                        t.span,
                    ))
                }
            }
        } else {
            Err(self.unexpected_end_of_input())
        }
    }

    pub fn consume_punctuation(&mut self, expected: PunctuationKind) -> Result<(), ParsingError> {
        if let Some(token) = self.current() {
            match &token.kind {
                TokenKind::Punctuation(pk) if pk == &expected => {
                    self.advance();
                    Ok(())
                }
                _ => Err(ParsingError::new(
                    ParsingErrorKind::ExpectedAPunctuationMark(expected),
                    token.span,
                )),
            }
        } else {
            Err(self.unexpected_end_of_input())
        }
    }

    pub fn consume_number(&mut self) -> Result<NumberKind, ParsingError> {
        if let Some(token) = self.current() {
            match token.kind {
                TokenKind::Number(number_kind) => {
                    self.advance();
                    return Ok(number_kind);
                }
                _ => {
                    return Err(ParsingError::new(
                        ParsingErrorKind::ExpectedANumericValue,
                        token.span,
                    ))
                }
            }
        }

        Err(self.unexpected_end_of_input())
    }

    pub fn consume_keyword(&mut self, expected: KeywordKind) -> Result<(), ParsingError> {
        if let Some(token) = self.current() {
            match token.kind {
                TokenKind::Keyword(keyword_kind) if keyword_kind == expected => {
                    self.advance();
                    Ok(())
                }
                _ => Err(ParsingError::new(
                    ParsingErrorKind::ExpectedAKeyword(expected),
                    token.span,
                )),
            }
        } else {
            Err(self.unexpected_end_of_input())
        }
    }

    pub fn consume_identifier(&mut self) -> Result<IdentifierNode, ParsingError> {
        if let Some(token) = self.current() {
            match &token.kind {
                TokenKind::Identifier(id) => {
                    let identifier = id.clone();
                    let span = token.span;
                    self.advance();
                    Ok(IdentifierNode {
                        name: identifier,
                        span,
                    })
                }
                _ => Err(ParsingError::new(
                    ParsingErrorKind::ExpectedAnIdentifier,
                    token.span,
                )),
            }
        } else {
            Err(self.unexpected_end_of_input())
        }
    }

    pub fn consume_optional_doc(&mut self) -> Option<DocAnnotation> {
        let result = if let Some(Token {
            kind: TokenKind::Doc(doc),
            span,
        }) = self.current()
        {
            Some(DocAnnotation {
                message: doc.to_owned(),
                span: span.clone(),
            })
        } else {
            None
        };

        if result.is_some() {
            self.advance();
        };

        result
    }

    pub fn comma_separated<F, T, E>(
        &mut self,
        mut parser: F,
        is_end: E,
    ) -> Result<Vec<T>, ParsingError>
    where
        F: FnMut(&mut Self) -> Result<T, ParsingError>,
        E: Fn(&Self) -> bool,
    {
        let mut items = Vec::new();

        if is_end(self) {
            return Ok(items);
        }

        let first_item = parser(self)?;
        items.push(first_item);

        loop {
            if is_end(self) {
                break;
            }

            self.consume_punctuation(PunctuationKind::Comma)?;

            if is_end(self) {
                break;
            }

            let item = parser(self)?;
            items.push(item);
        }

        Ok(items)
    }

    pub fn parse(tokens: Vec<Token>) -> Vec<Stmt> {
        let mut state = Parser {
            offset: 0,
            checkpoint_offset: 0,
            tokens,
        };

        let mut statements: Vec<Stmt> = vec![];

        while state.current().is_some() {
            let stmt = state.parse_stmt();
            let unwrapped = stmt.unwrap_or_else(|e| Stmt {
                kind: StmtKind::Error(e),
                span: Span {
                    start: Position { line: 0, col: 0 },
                    end: Position { line: 0, col: 0 },
                },
            });

            statements.push(unwrapped);
        }

        statements
    }
}

```

`src/parse/parse_generic_args.rs`:

```rs
use crate::{
    ast::{base::base_type::TypeAnnotation, Span},
    tokenizer::{PunctuationKind, TokenKind},
};

use super::{Parser, ParsingError};

impl Parser {
    pub fn parse_optional_generic_args(
        &mut self,
    ) -> Result<(Vec<TypeAnnotation>, Span), ParsingError> {
        let start_offset = self.offset;
        if self.match_token(0, TokenKind::Punctuation(PunctuationKind::Lt)) {
            self.advance();
            let result = self.comma_separated(
                |p| p.parse_type_annotation(0),
                |p| p.match_token(0, TokenKind::Punctuation(PunctuationKind::Gt)),
            )?;
            self.consume_punctuation(PunctuationKind::Gt)?;
            let span = self.get_span(start_offset, self.offset - 1)?;

            return Ok((result, span));
        }
        let span = self.get_span(start_offset, self.offset - 1)?;

        Ok((vec![], span))
    }
}

```

`src/parse/parse_generic_params.rs`:

```rs
use crate::{
    ast::{base::base_declaration::GenericParam, base::base_type::TypeAnnotation},
    tokenizer::{PunctuationKind, TokenKind},
};

use super::{Parser, ParsingError};

impl Parser {
    pub fn parse_generic_param_constraint(
        &mut self,
    ) -> Result<Option<TypeAnnotation>, ParsingError> {
        if self.match_token(0, TokenKind::Punctuation(PunctuationKind::Col)) {
            self.advance();
            Ok(Some(self.parse_type_annotation(0)?))
        } else {
            Ok(None)
        }
    }

    pub fn parse_optional_generic_params(&mut self) -> Result<Vec<GenericParam>, ParsingError> {
        if self.match_token(0, TokenKind::Punctuation(PunctuationKind::Lt)) {
            self.advance();
            let result = self.comma_separated(
                |p| {
                    let name = p.consume_identifier()?;
                    let constraint = p.parse_generic_param_constraint()?;

                    Ok(GenericParam {
                        constraint,
                        identifier: name,
                    })
                },
                |p| p.match_token(0, TokenKind::Punctuation(PunctuationKind::Gt)),
            );
            self.consume_punctuation(PunctuationKind::Gt)?;

            return result;
        }

        Ok(vec![])
    }
}

```

`src/parse/statements/mod.rs`:

```rs
pub mod parse_assignment_stmt;
pub mod parse_break_stmt;
pub mod parse_continue_stmt;
pub mod parse_enum_decl;
pub mod parse_expr_stmt;
pub mod parse_from_stmt;
pub mod parse_return_stmt;
pub mod parse_struct_decl;
pub mod parse_type_alias_decl;
pub mod parse_var_decl;
pub mod parse_while_stmt;

use crate::{
    ast::base::base_statement::Stmt,
    parse::{Parser, ParsingErrorKind},
    tokenizer::{KeywordKind, PunctuationKind, TokenKind},
};

use super::{expressions::is_start_of_expr, ParsingError};

pub fn is_start_of_stmt(token_kind: &TokenKind) -> bool {
    match token_kind {
        TokenKind::Keyword(KeywordKind::From)
        | TokenKind::Keyword(KeywordKind::While)
        | TokenKind::Keyword(KeywordKind::Return)
        | TokenKind::Keyword(KeywordKind::Break)
        | TokenKind::Keyword(KeywordKind::Continue)
        | TokenKind::Keyword(KeywordKind::Struct)
        | TokenKind::Keyword(KeywordKind::Type)
        | TokenKind::Keyword(KeywordKind::Let)
        | TokenKind::Doc(_) => true,
        _ => false,
    }
}

impl Parser {
    pub fn parse_stmt(&mut self) -> Result<Stmt, ParsingError> {
        let result = if self.match_token(0, TokenKind::Keyword(KeywordKind::From)) {
            self.parse_from_stmt()
        } else if self.match_token(0, TokenKind::Keyword(KeywordKind::While)) {
            self.parse_while_stmt()
        } else if self.match_token(0, TokenKind::Keyword(KeywordKind::Return)) {
            self.parse_return_stmt()
        } else if self.match_token(0, TokenKind::Keyword(KeywordKind::Break)) {
            self.parse_break_stmt()
        } else if self.match_token(0, TokenKind::Keyword(KeywordKind::Continue)) {
            self.parse_continue_stmt()
        } else {
            let documentation = self.consume_optional_doc();

            if self.match_token(0, TokenKind::Keyword(KeywordKind::Struct)) {
                self.parse_struct_decl(documentation)
            } else if self.match_token(0, TokenKind::Keyword(KeywordKind::Enum)) {
                self.parse_enum_decl(documentation)
            } else if self.match_token(0, TokenKind::Keyword(KeywordKind::Type)) {
                self.parse_type_alias_decl(documentation)
            } else if self.match_token(0, TokenKind::Keyword(KeywordKind::Let)) {
                self.parse_var_decl(documentation)
            } else if let Some(doc) = documentation {
                Err(ParsingError::new(
                    ParsingErrorKind::DocMustBeFollowedByDeclaration,
                    doc.span,
                ))
            } else {
                let lhs = self.parse_expr(0);

                match lhs {
                    Ok(lhs) => {
                        if self.match_token(0, TokenKind::Punctuation(PunctuationKind::Eq)) {
                            // it's an assignment statement
                            self.parse_assignment_stmt(lhs)
                        } else {
                            // It's a standalone expression statement
                            self.parse_expr_stmt(lhs)
                        }
                    }
                    Err(e) => Err(e),
                }
            }
        };

        result.map_err(|e| {
            self.synchronize_stmt();
            e
        })
    }

    pub fn synchronize_stmt(&mut self) {
        loop {
            match self.current() {
                Some(token) => {
                    if is_start_of_stmt(&token.kind) || is_start_of_expr(&token.kind) {
                        return;
                    }
                    if token.kind == TokenKind::Punctuation(PunctuationKind::SemiCol) {
                        self.advance();
                        return;
                    }

                    self.advance();
                }
                None => return,
            }
        }
    }
}

```

`src/parse/statements/parse_assignment_stmt.rs`:

```rs
use crate::{
    ast::{
        base::{
            base_expression::Expr,
            base_statement::{Stmt, StmtKind},
        },
        Span,
    },
    parse::{Parser, ParsingError},
    tokenizer::PunctuationKind,
};

impl Parser {
    pub fn parse_assignment_stmt(&mut self, lhs: Expr) -> Result<Stmt, ParsingError> {
        let start_offset = self.offset;
        self.consume_punctuation(PunctuationKind::Eq)?;
        let value = self.parse_expr(0)?;
        self.consume_punctuation(PunctuationKind::SemiCol)?;
        let span_end = self.get_span(start_offset, self.offset - 1)?;
        Ok(Stmt {
            span: Span {
                start: lhs.span.start,
                end: span_end.end,
            },
            kind: StmtKind::Assignment { target: lhs, value },
        })
    }
}

```

`src/parse/statements/parse_break_stmt.rs`:

```rs
use crate::{
    ast::base::base_statement::{Stmt, StmtKind},
    parse::{Parser, ParsingError},
    tokenizer::KeywordKind,
};

impl Parser {
    pub fn parse_break_stmt(&mut self) -> Result<Stmt, ParsingError> {
        let start_offset = self.offset;
        self.consume_keyword(KeywordKind::Break)?;
        let span = self.get_span(start_offset, self.offset - 1)?;
        return Ok(Stmt {
            kind: StmtKind::Break,
            span,
        });
    }
}

```

`src/parse/statements/parse_continue_stmt.rs`:

```rs
use crate::{
    ast::base::base_statement::{Stmt, StmtKind},
    parse::{Parser, ParsingError},
    tokenizer::KeywordKind,
};

impl Parser {
    pub fn parse_continue_stmt(&mut self) -> Result<Stmt, ParsingError> {
        let start_offset = self.offset;
        self.consume_keyword(KeywordKind::Continue)?;
        let span = self.get_span(start_offset, self.offset - 1)?;
        return Ok(Stmt {
            kind: StmtKind::Continue,
            span,
        });
    }
}

```

`src/parse/statements/parse_enum_decl.rs`:

```rs
use crate::{
    ast::base::{
        base_declaration::EnumDecl,
        base_statement::{Stmt, StmtKind},
    },
    parse::{DocAnnotation, Parser, ParsingError},
    tokenizer::{KeywordKind, PunctuationKind, TokenKind},
};

impl Parser {
    pub fn parse_enum_decl(
        &mut self,
        documentation: Option<DocAnnotation>,
    ) -> Result<Stmt, ParsingError> {
        let start_offset = self.offset;

        self.consume_keyword(KeywordKind::Enum)?;
        let identifier = self.consume_identifier()?;
        self.consume_punctuation(PunctuationKind::LBrace)?;
        let variants = self.comma_separated(
            |p| p.consume_identifier(),
            |p| p.match_token(0, TokenKind::Punctuation(PunctuationKind::RBrace)),
        )?;
        self.consume_punctuation(PunctuationKind::RBrace)?;

        let span = self.get_span(start_offset, self.offset - 1)?;

        Ok(Stmt {
            kind: StmtKind::EnumDecl(EnumDecl {
                identifier,
                documentation,
                variants,
            }),
            span,
        })
    }
}

```

`src/parse/statements/parse_expr_stmt.rs`:

```rs
use crate::{
    ast::{
        base::{
            base_expression::Expr,
            base_statement::{Stmt, StmtKind},
        },
        Span,
    },
    parse::{Parser, ParsingError},
    tokenizer::PunctuationKind,
};

impl Parser {
    pub fn parse_expr_stmt(&mut self, lhs: Expr) -> Result<Stmt, ParsingError> {
        let start_offset = self.offset;
        self.consume_punctuation(PunctuationKind::SemiCol)?;
        let span_end = self.get_span(start_offset, self.offset - 1)?;
        Ok(Stmt {
            span: Span {
                start: lhs.span.start,
                end: span_end.end,
            },
            kind: StmtKind::Expression(lhs),
        })
    }
}

```

`src/parse/statements/parse_from_stmt.rs`:

```rs
use crate::{
    ast::base::base_statement::{Stmt, StmtKind},
    parse::{Parser, ParsingError},
    tokenizer::{KeywordKind, PunctuationKind, TokenKind},
};

impl Parser {
    pub fn parse_from_stmt(&mut self) -> Result<Stmt, ParsingError> {
        let start_offset = self.offset;

        self.consume_keyword(KeywordKind::From)?;
        let path = self.consume_string()?;

        self.consume_punctuation(PunctuationKind::LBrace)?;
        let identifiers = self.comma_separated(
            |p| {
                let identifier = p.consume_identifier()?;
                let alias = if p.match_token(0, TokenKind::Punctuation(PunctuationKind::Col)) {
                    p.advance();
                    Some(p.consume_identifier()?)
                } else {
                    None
                };

                Ok((identifier, alias))
            },
            |p| p.match_token(0, TokenKind::Punctuation(PunctuationKind::RBrace)),
        )?;
        self.consume_punctuation(PunctuationKind::RBrace)?;

        let span = self.get_span(start_offset, self.offset - 1)?;

        return Ok(Stmt {
            kind: StmtKind::From { path, identifiers },
            span,
        });
    }
}

```

`src/parse/statements/parse_return_stmt.rs`:

```rs
use crate::{
    ast::base::base_statement::{Stmt, StmtKind},
    parse::{Parser, ParsingError},
    tokenizer::{KeywordKind, PunctuationKind},
};

impl Parser {
    pub fn parse_return_stmt(&mut self) -> Result<Stmt, ParsingError> {
        let start_offset = self.offset;

        self.consume_keyword(KeywordKind::Return)?;
        let expr = self.parse_expr(0)?;
        self.consume_punctuation(PunctuationKind::SemiCol)?;

        let span = self.get_span(start_offset, self.offset - 1)?;

        return Ok(Stmt {
            kind: StmtKind::Return(expr),
            span,
        });
    }
}

```

`src/parse/statements/parse_struct_decl.rs`:

```rs
use crate::{
    ast::base::{
        base_declaration::{Param, StructDecl},
        base_statement::{Stmt, StmtKind},
    },
    parse::{DocAnnotation, Parser, ParsingError},
    tokenizer::{KeywordKind, PunctuationKind, TokenKind},
};

impl Parser {
    pub fn parse_struct_decl(
        &mut self,
        documentation: Option<DocAnnotation>,
    ) -> Result<Stmt, ParsingError> {
        let start_offset = self.offset;

        self.consume_keyword(KeywordKind::Struct)?;
        let name = self.consume_identifier()?;
        let generic_params = self.parse_optional_generic_params()?;
        self.consume_punctuation(PunctuationKind::LBrace)?;
        let properties = self.comma_separated(
            |p| {
                let name = p.consume_identifier()?;
                p.consume_punctuation(PunctuationKind::Col)?;
                let constraint = p.parse_type_annotation(0)?;

                Ok(Param {
                    constraint,
                    identifier: name,
                })
            },
            |p| p.match_token(0, TokenKind::Punctuation(PunctuationKind::RBrace)),
        )?;
        self.consume_punctuation(PunctuationKind::RBrace)?;

        let span = self.get_span(start_offset, self.offset - 1)?;

        Ok(Stmt {
            kind: StmtKind::StructDecl(StructDecl {
                identifier: name,
                documentation,
                generic_params,
                properties,
            }),
            span,
        })
    }
}

```

`src/parse/statements/parse_type_alias_decl.rs`:

```rs
use crate::{
    ast::base::{
        base_declaration::TypeAliasDecl,
        base_statement::{Stmt, StmtKind},
    },
    parse::{DocAnnotation, Parser, ParsingError},
    tokenizer::{KeywordKind, PunctuationKind},
};

impl Parser {
    pub fn parse_type_alias_decl(
        &mut self,
        documentation: Option<DocAnnotation>,
    ) -> Result<Stmt, ParsingError> {
        let start_offset = self.offset;

        self.consume_keyword(KeywordKind::Type)?;

        let name = self.consume_identifier()?;
        let generic_params = self.parse_optional_generic_params()?;

        self.consume_punctuation(PunctuationKind::Eq)?;

        let ty = self.parse_type_annotation(0)?;

        self.consume_punctuation(PunctuationKind::SemiCol)?;

        let span = self.get_span(start_offset, self.offset - 1)?;

        Ok(Stmt {
            kind: StmtKind::TypeAliasDecl(TypeAliasDecl {
                identifier: name,
                documentation,
                generic_params,
                value: ty,
            }),
            span,
        })
    }
}

```

`src/parse/statements/parse_var_decl.rs`:

```rs
use crate::{
    ast::base::{
        base_declaration::VarDecl,
        base_statement::{Stmt, StmtKind},
    },
    parse::{DocAnnotation, Parser, ParsingError},
    tokenizer::{KeywordKind, PunctuationKind, TokenKind},
};

impl Parser {
    pub fn parse_var_decl(
        &mut self,
        documentation: Option<DocAnnotation>,
    ) -> Result<Stmt, ParsingError> {
        let start_offset = self.offset;

        self.consume_keyword(KeywordKind::Let)?;

        let name = self.consume_identifier()?;

        let constraint = if self.match_token(0, TokenKind::Punctuation(PunctuationKind::Col)) {
            self.advance();
            Some(self.parse_type_annotation(0)?)
        } else {
            None
        };

        let value = if self.match_token(0, TokenKind::Punctuation(PunctuationKind::Eq)) {
            self.consume_punctuation(PunctuationKind::Eq)?;
            Some(self.parse_expr(0)?)
        } else {
            None
        };

        self.consume_punctuation(PunctuationKind::SemiCol)?;

        let span = self.get_span(start_offset, self.offset - 1)?;

        Ok(Stmt {
            kind: StmtKind::VarDecl(VarDecl {
                documentation,
                identifier: name,
                constraint,
                value,
            }),
            span,
        })
    }
}

```

`src/parse/statements/parse_while_stmt.rs`:

```rs
use crate::{
    ast::base::base_statement::{Stmt, StmtKind},
    parse::{Parser, ParsingError},
    tokenizer::KeywordKind,
};

impl Parser {
    pub fn parse_while_stmt(&mut self) -> Result<Stmt, ParsingError> {
        let start_offset = self.offset;

        self.consume_keyword(KeywordKind::While)?;
        let condition = Box::new(self.parse_expr(0)?);
        let body = self.parse_codeblock_expr()?;

        let span = self.get_span(start_offset, self.offset - 1)?;

        Ok(Stmt {
            kind: StmtKind::While { condition, body },
            span,
        })
    }
}

```

`src/parse/type_annotations/mod.rs`:

```rs
pub mod parse_fn_type_annotation;
pub mod parse_parenthesized_type_annotation;

use super::{Parser, ParsingError, ParsingErrorKind};
use crate::{
    ast::{
        base::base_type::{TypeAnnotation, TypeAnnotationKind},
        Span,
    },
    tokenizer::{KeywordKind, PunctuationKind, Token, TokenKind},
};

fn infix_bp(token_kind: &TokenKind) -> Option<(u8, u8)> {
    use PunctuationKind::*;
    use TokenKind::*;

    let priority = match token_kind {
        Punctuation(Or) => (1, 2),
        _ => return None,
    };

    Some(priority)
}

fn suffix_bp(token_kind: &TokenKind) -> Option<(u8, ())> {
    use PunctuationKind::*;
    use TokenKind::*;

    let priority = match token_kind {
        Punctuation(LBracket) => (3, ()),
        Punctuation(Lt) => (3, ()),
        _ => return None,
    };

    Some(priority)
}

impl Parser {
    pub fn parse_type_annotation(&mut self, min_prec: u8) -> Result<TypeAnnotation, ParsingError> {
        let token = self.current().ok_or(self.unexpected_end_of_input())?;

        let mut lhs = match token {
            Token {
                kind: TokenKind::Keyword(KeywordKind::Void),
                ..
            } => {
                let start_offset = self.offset;

                self.consume_keyword(KeywordKind::Void)?;
                let span = self.get_span(start_offset, self.offset - 1)?;
                TypeAnnotation {
                    kind: TypeAnnotationKind::Void,
                    span,
                }
            }
            Token {
                kind: TokenKind::Keyword(KeywordKind::Null),
                ..
            } => {
                let start_offset = self.offset;

                self.consume_keyword(KeywordKind::Null)?;
                let span = self.get_span(start_offset, self.offset - 1)?;
                TypeAnnotation {
                    kind: TypeAnnotationKind::Null,
                    span,
                }
            }
            Token {
                kind: TokenKind::Keyword(KeywordKind::Bool),
                ..
            } => {
                let start_offset = self.offset;

                self.consume_keyword(KeywordKind::Bool)?;
                let span = self.get_span(start_offset, self.offset - 1)?;
                TypeAnnotation {
                    kind: TypeAnnotationKind::Bool,
                    span,
                }
            }
            Token {
                kind: TokenKind::Keyword(KeywordKind::U8),
                ..
            } => {
                let start_offset = self.offset;

                self.consume_keyword(KeywordKind::U8)?;
                let span = self.get_span(start_offset, self.offset - 1)?;
                TypeAnnotation {
                    kind: TypeAnnotationKind::U8,
                    span,
                }
            }
            Token {
                kind: TokenKind::Keyword(KeywordKind::U16),
                ..
            } => {
                let start_offset = self.offset;

                self.consume_keyword(KeywordKind::U16)?;
                let span = self.get_span(start_offset, self.offset - 1)?;
                TypeAnnotation {
                    kind: TypeAnnotationKind::U16,
                    span,
                }
            }
            Token {
                kind: TokenKind::Keyword(KeywordKind::U32),
                ..
            } => {
                let start_offset = self.offset;

                self.consume_keyword(KeywordKind::U32)?;
                let span = self.get_span(start_offset, self.offset - 1)?;
                TypeAnnotation {
                    kind: TypeAnnotationKind::U32,
                    span,
                }
            }
            Token {
                kind: TokenKind::Keyword(KeywordKind::U64),
                ..
            } => {
                let start_offset = self.offset;

                self.consume_keyword(KeywordKind::U64)?;
                let span = self.get_span(start_offset, self.offset - 1)?;
                TypeAnnotation {
                    kind: TypeAnnotationKind::U64,
                    span,
                }
            }
            Token {
                kind: TokenKind::Keyword(KeywordKind::USize),
                ..
            } => {
                let start_offset = self.offset;

                self.consume_keyword(KeywordKind::USize)?;
                let span = self.get_span(start_offset, self.offset - 1)?;
                TypeAnnotation {
                    kind: TypeAnnotationKind::USize,
                    span,
                }
            }
            Token {
                kind: TokenKind::Keyword(KeywordKind::ISize),
                ..
            } => {
                let start_offset = self.offset;

                self.consume_keyword(KeywordKind::ISize)?;
                let span = self.get_span(start_offset, self.offset - 1)?;
                TypeAnnotation {
                    kind: TypeAnnotationKind::ISize,
                    span,
                }
            }
            Token {
                kind: TokenKind::Keyword(KeywordKind::I8),
                ..
            } => {
                let start_offset = self.offset;

                self.consume_keyword(KeywordKind::I8)?;
                let span = self.get_span(start_offset, self.offset - 1)?;
                TypeAnnotation {
                    kind: TypeAnnotationKind::I8,
                    span,
                }
            }
            Token {
                kind: TokenKind::Keyword(KeywordKind::I16),
                ..
            } => {
                let start_offset = self.offset;

                self.consume_keyword(KeywordKind::I16)?;
                let span = self.get_span(start_offset, self.offset - 1)?;
                TypeAnnotation {
                    kind: TypeAnnotationKind::I16,
                    span,
                }
            }
            Token {
                kind: TokenKind::Keyword(KeywordKind::I32),
                ..
            } => {
                let start_offset = self.offset;

                self.consume_keyword(KeywordKind::I32)?;
                let span = self.get_span(start_offset, self.offset - 1)?;
                TypeAnnotation {
                    kind: TypeAnnotationKind::I32,
                    span,
                }
            }
            Token {
                kind: TokenKind::Keyword(KeywordKind::I64),
                ..
            } => {
                let start_offset = self.offset;

                self.consume_keyword(KeywordKind::I64)?;
                let span = self.get_span(start_offset, self.offset - 1)?;
                TypeAnnotation {
                    kind: TypeAnnotationKind::I64,
                    span,
                }
            }
            Token {
                kind: TokenKind::Keyword(KeywordKind::F32),
                ..
            } => {
                let start_offset = self.offset;

                self.consume_keyword(KeywordKind::F32)?;
                let span = self.get_span(start_offset, self.offset - 1)?;
                TypeAnnotation {
                    kind: TypeAnnotationKind::F32,
                    span,
                }
            }
            Token {
                kind: TokenKind::Keyword(KeywordKind::F64),
                ..
            } => {
                let start_offset = self.offset;

                self.consume_keyword(KeywordKind::F64)?;
                let span = self.get_span(start_offset, self.offset - 1)?;
                TypeAnnotation {
                    kind: TypeAnnotationKind::F64,
                    span,
                }
            }
            Token {
                kind: TokenKind::Keyword(KeywordKind::Char),
                ..
            } => {
                let start_offset = self.offset;

                self.consume_keyword(KeywordKind::Char)?;
                let span = self.get_span(start_offset, self.offset - 1)?;
                TypeAnnotation {
                    kind: TypeAnnotationKind::Char,
                    span,
                }
            }
            Token {
                kind: TokenKind::Punctuation(PunctuationKind::LParen),
                ..
            } => {
                self.place_checkpoint();
                let type_annotation = self.parse_fn_type_annotation().or_else(|_| {
                    self.goto_checkpoint();
                    self.parse_parenthesized_type_annotation()
                    // TODO: report an error when all parsing attempts fail
                })?;

                type_annotation
            }
            Token {
                kind: TokenKind::Identifier(_),
                ..
            } => {
                let start_offset = self.offset;

                let id = self.consume_identifier()?;
                let span = self.get_span(start_offset, self.offset - 1)?;
                TypeAnnotation {
                    kind: TypeAnnotationKind::Identifier(id),
                    span,
                }
            }
            t => {
                return Err(ParsingError::new(
                    ParsingErrorKind::ExpectedATypeButFound(t.clone()),
                    t.span,
                ))
            }
        };

        loop {
            let op = match self.current() {
                Some(o) => o,
                None => break,
            };

            if let Some((left_prec, ())) = suffix_bp(&op.kind) {
                if left_prec < min_prec {
                    break;
                }

                lhs = match op.kind {
                    TokenKind::Punctuation(PunctuationKind::Lt) => {
                        let (generic_args, generic_args_span) =
                            self.parse_optional_generic_args()?;

                        TypeAnnotation {
                            kind: TypeAnnotationKind::GenericApply {
                                left: Box::new(lhs.clone()),
                                args: generic_args,
                            },
                            span: generic_args_span,
                        }
                    }
                    TokenKind::Punctuation(PunctuationKind::LBracket) => {
                        self.consume_punctuation(PunctuationKind::LBracket)?;
                        let start_offset = self.offset;

                        let size = self.consume_number()?;
                        let span = self.get_span(start_offset, self.offset - 1)?;
                        self.consume_punctuation(PunctuationKind::RBracket)?;
                        TypeAnnotation {
                            kind: TypeAnnotationKind::Array {
                                left: Box::new(lhs),
                                size,
                            },
                            span,
                        }
                    }
                    _ => break,
                };

                continue;
            }

            if let Some((left_prec, right_prec)) = infix_bp(&op.kind) {
                if left_prec < min_prec {
                    break;
                }

                lhs = match op.kind {
                    TokenKind::Punctuation(PunctuationKind::Or) => {
                        let start_offset = self.offset;

                        self.advance();
                        let rhs = self.parse_type_annotation(right_prec)?;
                        let end_span = self.get_span(start_offset, self.offset - 1)?;
                        let span = Span {
                            start: lhs.span.start,
                            end: end_span.end,
                        };

                        let kind =
                            if let TypeAnnotationKind::Union(existing_variants) = &mut lhs.kind {
                                existing_variants.push(rhs);
                                lhs.kind
                            } else {
                                TypeAnnotationKind::Union(vec![lhs, rhs])
                            };

                        TypeAnnotation { kind, span }
                    }
                    _ => break,
                };
                continue;
            }

            break;
        }

        Ok(lhs)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::{
            base::base_type::{TypeAnnotation, TypeAnnotationKind},
            Span,
        },
        parse::Parser,
    };

    #[test]
    fn parses_primitive_types() {
        use crate::ast::Position;
        use crate::tokenizer::Tokenizer;
        use pretty_assertions::assert_eq;

        let test_cases = vec![
            (
                "i8",
                TypeAnnotation {
                    kind: TypeAnnotationKind::I8,
                    span: Span {
                        start: Position { line: 1, col: 1 },
                        end: Position { line: 1, col: 3 },
                    },
                },
            ),
            (
                "i16",
                TypeAnnotation {
                    kind: TypeAnnotationKind::I16,
                    span: Span {
                        start: Position { line: 1, col: 1 },
                        end: Position { line: 1, col: 4 },
                    },
                },
            ),
            (
                "i32",
                TypeAnnotation {
                    kind: TypeAnnotationKind::I32,
                    span: Span {
                        start: Position { line: 1, col: 1 },
                        end: Position { line: 1, col: 4 },
                    },
                },
            ),
            (
                "i64",
                TypeAnnotation {
                    kind: TypeAnnotationKind::I64,
                    span: Span {
                        start: Position { line: 1, col: 1 },
                        end: Position { line: 1, col: 4 },
                    },
                },
            ),
            (
                "f32",
                TypeAnnotation {
                    kind: TypeAnnotationKind::F32,
                    span: Span {
                        start: Position { line: 1, col: 1 },
                        end: Position { line: 1, col: 4 },
                    },
                },
            ),
            (
                "f64",
                TypeAnnotation {
                    kind: TypeAnnotationKind::F64,
                    span: Span {
                        start: Position { line: 1, col: 1 },
                        end: Position { line: 1, col: 4 },
                    },
                },
            ),
            (
                "u8",
                TypeAnnotation {
                    kind: TypeAnnotationKind::U8,
                    span: Span {
                        start: Position { line: 1, col: 1 },
                        end: Position { line: 1, col: 3 },
                    },
                },
            ),
            (
                "u16",
                TypeAnnotation {
                    kind: TypeAnnotationKind::U16,
                    span: Span {
                        start: Position { line: 1, col: 1 },
                        end: Position { line: 1, col: 4 },
                    },
                },
            ),
            (
                "u32",
                TypeAnnotation {
                    kind: TypeAnnotationKind::U32,
                    span: Span {
                        start: Position { line: 1, col: 1 },
                        end: Position { line: 1, col: 4 },
                    },
                },
            ),
            (
                "u64",
                TypeAnnotation {
                    kind: TypeAnnotationKind::U64,
                    span: Span {
                        start: Position { line: 1, col: 1 },
                        end: Position { line: 1, col: 4 },
                    },
                },
            ),
            (
                "usize",
                TypeAnnotation {
                    kind: TypeAnnotationKind::USize,
                    span: Span {
                        start: Position { line: 1, col: 1 },
                        end: Position { line: 1, col: 6 },
                    },
                },
            ),
            (
                "void",
                TypeAnnotation {
                    kind: TypeAnnotationKind::Void,
                    span: Span {
                        start: Position { line: 1, col: 1 },
                        end: Position { line: 1, col: 5 },
                    },
                },
            ),
            (
                "null",
                TypeAnnotation {
                    kind: TypeAnnotationKind::Null,
                    span: Span {
                        start: Position { line: 1, col: 1 },
                        end: Position { line: 1, col: 5 },
                    },
                },
            ),
            (
                "bool",
                TypeAnnotation {
                    kind: TypeAnnotationKind::Bool,
                    span: Span {
                        start: Position { line: 1, col: 1 },
                        end: Position { line: 1, col: 5 },
                    },
                },
            ),
            (
                "char",
                TypeAnnotation {
                    kind: TypeAnnotationKind::Char,
                    span: Span {
                        start: Position { line: 1, col: 1 },
                        end: Position { line: 1, col: 5 },
                    },
                },
            ),
        ];

        for (input, expected) in test_cases {
            let tokens = Tokenizer::tokenize(input.to_owned());
            let mut parser = Parser {
                offset: 0,
                checkpoint_offset: 0,
                tokens,
            };
            let result = parser.parse_type_annotation(0);

            assert_eq!(result, Ok(expected))
        }
    }

    #[test]
    fn parses_union_types() {
        use crate::ast::Position;
        use crate::tokenizer::Tokenizer;
        use pretty_assertions::assert_eq;

        let tokens = Tokenizer::tokenize("i8 | i16 | i32 | i64".to_owned());
        let mut parser = Parser {
            offset: 0,
            checkpoint_offset: 0,
            tokens,
        };
        let result = parser.parse_type_annotation(0);

        assert_eq!(
            result,
            Ok(TypeAnnotation {
                kind: TypeAnnotationKind::Union(vec![
                    TypeAnnotation {
                        kind: TypeAnnotationKind::I8,
                        span: Span {
                            start: Position { line: 1, col: 1 },
                            end: Position { line: 1, col: 3 }
                        }
                    },
                    TypeAnnotation {
                        kind: TypeAnnotationKind::I16,
                        span: Span {
                            start: Position { line: 1, col: 6 },
                            end: Position { line: 1, col: 9 }
                        }
                    },
                    TypeAnnotation {
                        kind: TypeAnnotationKind::I32,
                        span: Span {
                            start: Position { line: 1, col: 12 },
                            end: Position { line: 1, col: 15 }
                        }
                    },
                    TypeAnnotation {
                        kind: TypeAnnotationKind::I64,
                        span: Span {
                            start: Position { line: 1, col: 18 },
                            end: Position { line: 1, col: 21 }
                        }
                    }
                ]),
                span: Span {
                    start: Position { line: 1, col: 1 },
                    end: Position { line: 1, col: 21 }
                }
            })
        )
    }
}

```

`src/parse/type_annotations/parse_fn_type_annotation.rs`:

```rs
use crate::{
    ast::base::{
        base_declaration::Param,
        base_type::{TypeAnnotation, TypeAnnotationKind},
    },
    parse::ParsingError,
    tokenizer::{PunctuationKind, TokenKind},
};

use super::Parser;

impl Parser {
    pub fn parse_fn_type_annotation(&mut self) -> Result<TypeAnnotation, ParsingError> {
        let start_offset = self.offset;

        let generic_params = self.parse_optional_generic_params()?;
        self.consume_punctuation(PunctuationKind::LParen)?;
        let params = self.comma_separated(
            |p| {
                let identifier = p.consume_identifier()?;
                p.consume_punctuation(PunctuationKind::Col)?;
                let constraint = p.parse_type_annotation(0)?;

                Ok(Param {
                    constraint,
                    identifier,
                })
            },
            |p| p.match_token(0, TokenKind::Punctuation(PunctuationKind::RParen)),
        )?;
        self.consume_punctuation(PunctuationKind::RParen)?;

        self.consume_punctuation(PunctuationKind::FatArrow)?;

        let return_type = Box::new(self.parse_type_annotation(0)?);

        let span = self.get_span(start_offset, self.offset - 1)?;

        let type_kind = if generic_params.is_empty() {
            TypeAnnotationKind::FnType {
                params,
                return_type,
            }
        } else {
            TypeAnnotationKind::GenericFnType {
                params,
                return_type,
                generic_params,
            }
        };

        Ok(TypeAnnotation {
            kind: type_kind,
            span,
        })
    }
}

```

`src/parse/type_annotations/parse_parenthesized_type_annotation.rs`:

```rs
use crate::{
    ast::base::base_type::TypeAnnotation, parse::ParsingError, tokenizer::PunctuationKind,
};

use super::Parser;

impl Parser {
    pub fn parse_parenthesized_type_annotation(&mut self) -> Result<TypeAnnotation, ParsingError> {
        self.consume_punctuation(PunctuationKind::LParen)?;
        let item = self.parse_type_annotation(0)?;
        self.consume_punctuation(PunctuationKind::RParen)?;

        Ok(item)
    }
}

```

`src/tokenizer/mod.rs`:

```rs
use unicode_segmentation::UnicodeSegmentation;

pub mod tokenize_documentation;
pub mod tokenize_identifier;
pub mod tokenize_number;
pub mod tokenize_punctuation;
pub mod tokenize_string;

use crate::ast::{Position, Span};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenizationError {
    UnknownToken,
    UnknownEscapeSequence,
    InvalidFloatingNumber,
    InvalidIntegerNumber,
    UnterminatedString,
    UnterminatedDoc,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PunctuationKind {
    DoubleCol,
    DoubleOr,
    DoubleAnd,
    DoubleEq,
    Col,
    SemiCol,
    Lt,
    Gt,
    Lte,
    Gte,
    Or,
    And,
    Not,
    Dot,
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Eq,
    NotEq,
    Plus,
    Minus,
    Slash,
    Star,
    Percent,
    Comma,
    Dollar,
    Question,
    FatArrow,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum KeywordKind {
    Struct,
    Enum,
    Let,
    Return,
    If,
    Else,
    While,
    Break,
    Continue,
    Type,
    From,
    Void,
    Null,
    True,
    False,
    Pub,
    Char,
    Bool,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    USize,
    ISize,
    F32,
    F64,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum NumberKind {
    I64(i64),
    I32(i32),
    I16(i16),
    I8(i8),
    F32(f32),
    F64(f64),
    U64(u64),
    U32(u32),
    U16(u16),
    U8(u8),
    USize(usize),
    ISize(isize),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Identifier(String),
    Punctuation(PunctuationKind),
    Keyword(KeywordKind),
    String(String),
    Number(NumberKind),
    Doc(String),
    Error(TokenizationError),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub span: Span,
    pub kind: TokenKind,
}

#[derive(Debug)]
pub struct Tokenizer {
    input: String,
    offset: usize,
    line: usize,
    col: usize,
}

impl Tokenizer {
    fn current(&self) -> Option<&str> {
        self.input.graphemes(true).nth(self.offset)
    }

    fn consume(&mut self) {
        if let Some(c) = self.current() {
            if c == "\n" {
                self.line += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
            self.offset += 1;
        }
    }

    fn peek(&self, i: usize) -> Option<&str> {
        self.input.graphemes(true).nth(self.offset + i)
    }

    fn slice(&self, start: usize, end: usize) -> &str {
        let grapheme_indices: Vec<(usize, &str)> = self.input.grapheme_indices(true).collect();

        let start_idx = grapheme_indices[start].0;
        let end_idx = if end < grapheme_indices.len() {
            grapheme_indices[end].0
        } else {
            self.input.len()
        };

        &self.input[start_idx..end_idx]
    }

    fn synchronize(&mut self) {
        while let Some(ch) = self.current() {
            let is_whitespace = ch.chars().all(|c| c.is_whitespace());

            if is_whitespace || ch == ";" || ch == "," {
                self.consume();
                break;
            } else {
                self.consume();
            }
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current() {
            let is_whitespace = ch.chars().all(|c| c.is_whitespace());

            if is_whitespace {
                self.consume();
            } else {
                break;
            }
        }
    }

    fn skip_comment(&mut self) {
        if self.peek(0) == Some("/") && self.peek(1) == Some("/") {
            while let Some(c) = self.current() {
                if c == "\n" {
                    break;
                }
                self.consume();
            }
        }
    }

    pub fn tokenize(input: String) -> Vec<Token> {
        let mut state = Tokenizer {
            input,
            offset: 0,
            line: 1,
            col: 1,
        };
        let mut tokens: Vec<Token> = vec![];

        loop {
            state.skip_whitespace();
            state.skip_comment();

            let start_pos = Position {
                line: state.line,
                col: state.col,
            };

            match state.current() {
                Some(letter) if is_letter(letter) => {
                    let identifier = state.tokenize_identifier();
                    let keyword = is_keyword(&identifier);
                    let kind = if let Some(keyword_kind) = keyword {
                        TokenKind::Keyword(keyword_kind)
                    } else {
                        TokenKind::Identifier(identifier)
                    };
                    let end_pos = Position {
                        line: state.line,
                        col: state.col,
                    };

                    tokens.push(Token {
                        span: Span {
                            start: start_pos,
                            end: end_pos,
                        },
                        kind,
                    });
                }
                Some("\"") => match state.string() {
                    Ok(value) => {
                        let end_pos = Position {
                            line: state.line,
                            col: state.col,
                        };
                        tokens.push(Token {
                            span: Span {
                                start: start_pos,
                                end: end_pos,
                            },
                            kind: TokenKind::String(value),
                        })
                    }
                    Err(kind) => {
                        let end_pos = Position {
                            line: state.line,
                            col: state.col,
                        };
                        tokens.push(Token {
                            kind: TokenKind::Error(kind),
                            span: Span {
                                start: start_pos,
                                end: end_pos,
                            },
                        });
                        state.synchronize();
                    }
                },
                Some(digit) if is_digit(digit) => match state.tokenize_number() {
                    Ok(number_kind) => {
                        let end_pos = Position {
                            line: state.line,
                            col: state.col,
                        };
                        tokens.push(Token {
                            kind: TokenKind::Number(number_kind),
                            span: Span {
                                start: start_pos,
                                end: end_pos,
                            },
                        })
                    }
                    Err(kind) => {
                        let end_pos = Position {
                            line: state.line,
                            col: state.col,
                        };
                        tokens.push(Token {
                            kind: TokenKind::Error(kind),
                            span: Span {
                                start: start_pos,
                                end: end_pos,
                            },
                        });
                        state.synchronize();
                    }
                },
                Some("-") if state.peek(1) == Some("-") && state.peek(2) == Some("-") => {
                    match state.tokenize_documentation() {
                        Ok(content) => {
                            let end_pos = Position {
                                line: state.line,
                                col: state.col,
                            };
                            tokens.push(Token {
                                kind: TokenKind::Doc(content),
                                span: Span {
                                    start: start_pos,
                                    end: end_pos,
                                },
                            })
                        }
                        Err(kind) => {
                            let end_pos = Position {
                                line: state.line,
                                col: state.col,
                            };
                            tokens.push(Token {
                                kind: TokenKind::Error(kind),
                                span: Span {
                                    start: start_pos,
                                    end: end_pos,
                                },
                            });
                            state.synchronize();
                        }
                    }
                }
                Some(punct) => match state.tokenize_punctuation(&punct.to_owned()) {
                    Some(kind) => {
                        let end_pos = Position {
                            line: state.line,
                            col: state.col,
                        };
                        tokens.push(Token {
                            kind: TokenKind::Punctuation(kind),
                            span: Span {
                                start: start_pos,
                                end: end_pos,
                            },
                        })
                    }
                    None => {
                        let end_pos = Position {
                            line: state.line,
                            col: state.col,
                        };
                        tokens.push(Token {
                            kind: TokenKind::Error(TokenizationError::UnknownToken),
                            span: Span {
                                start: start_pos,
                                end: end_pos,
                            },
                        });
                        state.synchronize();
                    }
                },
                None => break,
            };
        }

        tokens
    }
}

fn is_letter(value: &str) -> bool {
    value.graphemes(true).count() == 1 && value.chars().all(char::is_alphabetic)
}

fn is_digit(value: &str) -> bool {
    value.graphemes(true).count() == 1 && value.chars().all(|x| char::is_ascii_digit(&x))
}

fn is_alphanumeric(value: &str) -> bool {
    value.graphemes(true).count() == 1 && value.chars().all(char::is_alphanumeric)
}

fn is_keyword(identifier: &str) -> Option<KeywordKind> {
    match identifier {
        "struct" => Some(KeywordKind::Struct),
        "enum" => Some(KeywordKind::Enum),
        "let" => Some(KeywordKind::Let),
        "return" => Some(KeywordKind::Return),
        "if" => Some(KeywordKind::If),
        "else" => Some(KeywordKind::Else),
        "while" => Some(KeywordKind::While),
        "break" => Some(KeywordKind::Break),
        "continue" => Some(KeywordKind::Continue),
        "type" => Some(KeywordKind::Type),
        "from" => Some(KeywordKind::From),
        "void" => Some(KeywordKind::Void),
        "null" => Some(KeywordKind::Null),
        "true" => Some(KeywordKind::True),
        "false" => Some(KeywordKind::False),
        "pub" => Some(KeywordKind::Pub),
        "char" => Some(KeywordKind::Char),
        "bool" => Some(KeywordKind::Bool),
        "i8" => Some(KeywordKind::I8),
        "i16" => Some(KeywordKind::I16),
        "i32" => Some(KeywordKind::I32),
        "i64" => Some(KeywordKind::I64),
        "u8" => Some(KeywordKind::U8),
        "u16" => Some(KeywordKind::U16),
        "u32" => Some(KeywordKind::U32),
        "u64" => Some(KeywordKind::U64),
        "f32" => Some(KeywordKind::F32),
        "f64" => Some(KeywordKind::F64),
        "usize" => Some(KeywordKind::USize),
        "isize" => Some(KeywordKind::ISize),
        _ => None,
    }
}

```

`src/tokenizer/tokenize_documentation.rs`:

```rs
use super::{TokenizationError, Tokenizer};

impl Tokenizer {
    pub fn tokenize_documentation(&mut self) -> Result<String, TokenizationError> {
        self.consume();
        self.consume();
        self.consume();

        let start = self.offset;
        while let Some(c) = self.current() {
            if c == "-" && self.peek(1) == Some("-") && self.peek(2) == Some("-") {
                let doc_content = self.slice(start, self.offset).to_owned();
                self.consume();
                self.consume();
                self.consume();
                return Ok(doc_content);
            }
            self.consume();
        }

        Err(TokenizationError::UnterminatedDoc)
    }
}

```

`src/tokenizer/tokenize_identifier.rs`:

```rs
use super::{is_alphanumeric, Tokenizer};

impl Tokenizer {
    pub fn tokenize_identifier(&mut self) -> String {
        let start = self.offset;
        while let Some(c) = self.current() {
            if is_alphanumeric(c) || c == "_" {
                self.consume();
            } else {
                break;
            }
        }

        self.slice(start, self.offset).to_owned()
    }
}

```

`src/tokenizer/tokenize_number.rs`:

```rs
use super::{is_digit, is_letter, NumberKind, TokenizationError, Tokenizer};

impl Tokenizer {
    pub fn tokenize_number(&mut self) -> Result<NumberKind, TokenizationError> {
        let start = self.offset;
        let mut has_dot = false;

        while let Some(c) = self.current() {
            if is_digit(c) {
                self.consume();
            } else if c == "." && !has_dot {
                has_dot = true;
                self.consume();
            } else if c == "." && has_dot {
                return Err(TokenizationError::InvalidFloatingNumber);
            } else if is_letter(c) {
                self.consume();
            } else {
                break;
            }
        }

        let number_str = self.slice(start, self.offset);

        parse_number(number_str)
    }
}

fn parse_number(number: &str) -> Result<NumberKind, TokenizationError> {
    if number.contains('.') {
        if let Ok(value) = number.parse::<f64>() {
            return Ok(NumberKind::F64(value));
        }
        if let Ok(value) = number.parse::<f32>() {
            return Ok(NumberKind::F32(value));
        }
        return Err(TokenizationError::InvalidFloatingNumber);
    } else {
        if let Ok(value) = number.parse::<i64>() {
            return Ok(NumberKind::I64(value));
        }
        if let Ok(value) = number.parse::<i32>() {
            return Ok(NumberKind::I32(value));
        }
        if let Ok(value) = number.parse::<i16>() {
            return Ok(NumberKind::I16(value));
        }
        if let Ok(value) = number.parse::<i8>() {
            return Ok(NumberKind::I8(value));
        }
        if let Ok(value) = number.parse::<u64>() {
            return Ok(NumberKind::U64(value));
        }
        if let Ok(value) = number.parse::<u32>() {
            return Ok(NumberKind::U32(value));
        }
        if let Ok(value) = number.parse::<u16>() {
            return Ok(NumberKind::U16(value));
        }
        if let Ok(value) = number.parse::<u8>() {
            return Ok(NumberKind::U8(value));
        }
        if let Ok(value) = number.parse::<usize>() {
            return Ok(NumberKind::USize(value));
        }
        if let Ok(value) = number.parse::<isize>() {
            return Ok(NumberKind::ISize(value));
        }

        return Err(TokenizationError::InvalidIntegerNumber);
    }
}

```

`src/tokenizer/tokenize_punctuation.rs`:

```rs
use super::{PunctuationKind, Tokenizer};

impl Tokenizer {
    pub fn tokenize_punctuation(&mut self, punct: &str) -> Option<PunctuationKind> {
        match punct {
            ":" => match self.peek(1) {
                Some(":") => {
                    self.consume();
                    self.consume();
                    Some(PunctuationKind::DoubleCol)
                }
                _ => {
                    self.consume();
                    Some(PunctuationKind::Col)
                }
            },
            "|" => match self.peek(1) {
                Some("|") => {
                    self.consume();
                    self.consume();
                    Some(PunctuationKind::DoubleOr)
                }
                _ => {
                    self.consume();
                    Some(PunctuationKind::Or)
                }
            },
            "&" => match self.peek(1) {
                Some("&") => {
                    self.consume();
                    self.consume();
                    Some(PunctuationKind::DoubleAnd)
                }
                _ => {
                    self.consume();
                    Some(PunctuationKind::And)
                }
            },
            "=" => match self.peek(1) {
                Some("=") => {
                    self.consume();
                    self.consume();
                    Some(PunctuationKind::DoubleEq)
                }
                Some(">") => {
                    self.consume();
                    self.consume();
                    Some(PunctuationKind::FatArrow)
                }
                _ => {
                    self.consume();
                    Some(PunctuationKind::Eq)
                }
            },
            "<" => match self.peek(1) {
                Some("=") => {
                    self.consume();
                    self.consume();
                    Some(PunctuationKind::Lte)
                }
                _ => {
                    self.consume();
                    Some(PunctuationKind::Lt)
                }
            },
            ">" => match self.peek(1) {
                Some("=") => {
                    self.consume();
                    self.consume();
                    Some(PunctuationKind::Gte)
                }
                _ => {
                    self.consume();
                    Some(PunctuationKind::Gt)
                }
            },
            "!" => match self.peek(1) {
                Some("=") => {
                    self.consume();
                    self.consume();
                    Some(PunctuationKind::NotEq)
                }
                _ => {
                    self.consume();
                    Some(PunctuationKind::Not)
                }
            },
            ";" => {
                self.consume();
                Some(PunctuationKind::SemiCol)
            }
            "." => {
                self.consume();
                Some(PunctuationKind::Dot)
            }
            "(" => {
                self.consume();
                Some(PunctuationKind::LParen)
            }
            ")" => {
                self.consume();
                Some(PunctuationKind::RParen)
            }
            "[" => {
                self.consume();
                Some(PunctuationKind::LBracket)
            }
            "]" => {
                self.consume();
                Some(PunctuationKind::RBracket)
            }
            "{" => {
                self.consume();
                Some(PunctuationKind::LBrace)
            }
            "}" => {
                self.consume();
                Some(PunctuationKind::RBrace)
            }
            "+" => {
                self.consume();
                Some(PunctuationKind::Plus)
            }
            "-" => {
                self.consume();
                Some(PunctuationKind::Minus)
            }
            "*" => {
                self.consume();
                Some(PunctuationKind::Star)
            }
            "/" => {
                self.consume();
                Some(PunctuationKind::Slash)
            }
            "%" => {
                self.consume();
                Some(PunctuationKind::Percent)
            }
            "," => {
                self.consume();
                Some(PunctuationKind::Comma)
            }
            "$" => {
                self.consume();
                Some(PunctuationKind::Dollar)
            }
            "?" => {
                self.consume();
                Some(PunctuationKind::Question)
            }
            _ => None,
        }
    }
}

```

`src/tokenizer/tokenize_string.rs`:

```rs
use super::{TokenizationError, Tokenizer};

impl Tokenizer {
    pub fn string(&mut self) -> Result<String, TokenizationError> {
        self.consume();
        let literal_start = self.offset;

        while let Some(c) = self.current() {
            match c {
                "\"" => {
                    let result = self.slice(literal_start, self.offset).to_owned();
                    self.consume();
                    return Ok(result);
                }
                "\\" => {
                    self.consume();
                    if let Some(next_char) = self.current() {
                        match next_char {
                            "\"" | "\\" | "$" | "{" | "}" | "n" | "r" | "t" => {
                                self.consume();
                            }
                            _ => {
                                return Err(TokenizationError::UnknownEscapeSequence);
                            }
                        }
                    } else {
                        return Err(TokenizationError::UnterminatedString);
                    }
                }
                _ => self.consume(),
            }
        }

        Err(TokenizationError::UnterminatedString)
    }
}

```