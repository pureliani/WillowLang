use std::hash::{Hash, Hasher};

use crate::{
    ast::{IdentifierNode, ModuleId, Span},
    cfg::ValueId,
    hir_builder::types::checked_type::Type,
    parse::DocAnnotation,
};

#[derive(Clone, Debug)]
pub struct CheckedParam {
    pub identifier: IdentifierNode,
    pub constraint: Type,
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
    pub constraint: Option<Box<Type>>,
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
pub struct CheckedFnType {
    pub params: Vec<CheckedParam>,
    pub return_type: Box<Type>,
    pub generic_params: Vec<CheckedGenericParam>,
    pub applied_type_args: Vec<Type>,
    pub span: Span,
}

impl Eq for CheckedFnType {}
impl PartialEq for CheckedFnType {
    fn eq(&self, other: &Self) -> bool {
        self.generic_params == other.generic_params
            && self.applied_type_args == other.applied_type_args
            && self.params == other.params
            && self.return_type == other.return_type
    }
}
impl Hash for CheckedFnType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.generic_params.hash(state);
        self.applied_type_args.hash(state);
        self.params.hash(state);
        self.return_type.hash(state);
    }
}

#[derive(Clone, Debug)]
pub struct CheckedTypeAliasDecl {
    pub identifier: IdentifierNode,
    pub module_id: ModuleId,
    pub documentation: Option<DocAnnotation>,
    pub generic_params: Vec<CheckedGenericParam>,
    pub value: Box<Type>,
    pub applied_type_args: Vec<Type>,
    pub span: Span,
}

impl Eq for CheckedTypeAliasDecl {}
impl PartialEq for CheckedTypeAliasDecl {
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier
            && self.module_id == other.module_id
            && self.applied_type_args == other.applied_type_args
    }
}
impl Hash for CheckedTypeAliasDecl {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.identifier.hash(state);
        self.module_id.hash(state);
        self.applied_type_args.hash(state);
    }
}

#[derive(Clone, Debug)]
pub struct CheckedEnumVariant {
    pub identifier: IdentifierNode,
    pub payload_type: Option<Type>,
}

impl Eq for CheckedEnumVariant {}
impl PartialEq for CheckedEnumVariant {
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier && self.payload_type == other.payload_type
    }
}
impl Hash for CheckedEnumVariant {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.identifier.hash(state);
        self.payload_type.hash(state);
    }
}

#[derive(Clone, Debug)]
pub struct CheckedEnumDecl {
    pub identifier: IdentifierNode,
    pub module_id: ModuleId,
    pub documentation: Option<DocAnnotation>,
    pub generic_params: Vec<CheckedGenericParam>,
    pub variants: Vec<CheckedEnumVariant>,
    pub applied_type_args: Vec<Type>,
    pub span: Span,
}

impl Eq for CheckedEnumDecl {}
impl PartialEq for CheckedEnumDecl {
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier
            && self.module_id == other.module_id
            && self.applied_type_args == other.applied_type_args
    }
}
impl Hash for CheckedEnumDecl {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.identifier.hash(state);
        self.module_id.hash(state);
        self.applied_type_args.hash(state);
    }
}

#[derive(Clone, Debug)]
pub struct CheckedVarDecl {
    pub value_id: ValueId, // pointer to stack variable
    pub identifier: IdentifierNode,
    pub documentation: Option<DocAnnotation>,
    pub constraint: Type,
}
