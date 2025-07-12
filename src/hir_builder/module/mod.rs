use std::collections::{HashMap, HashSet};

use crate::{
    ast::{decl::EnumDecl, IdentifierNode, ModuleId},
    cfg::{ControlFlowGraph, FunctionId},
    hir_builder::types::checked_declaration::{CheckedTypeAliasDecl, CheckedVarDecl},
};

#[derive(Clone, Debug)]
pub struct CheckedModule {
    pub id: ModuleId,
    pub name: String,
    pub functions: HashMap<FunctionId, ControlFlowGraph>,
    pub type_declarations: HashMap<IdentifierNode, CheckedTypeAliasDecl>,
    pub enum_declarations: HashMap<IdentifierNode, EnumDecl>,
    pub global_variables: HashMap<IdentifierNode, CheckedVarDecl>,
    pub initializer_cfg: Option<ControlFlowGraph>,
    pub exports: HashSet<IdentifierNode>,
}
