use std::collections::HashMap;

use crate::{
    ast::{base::base_declaration::EnumDecl, DefinitionId, ModuleId},
    cfg::ControlFlowGraph,
    compile::string_interner::InternerId,
    hir_builder::types::checked_declaration::{CheckedTypeAliasDecl, CheckedVarDecl},
};

#[derive(Clone, Debug)]
pub struct CheckedModule {
    pub id: ModuleId,
    pub name: String,
    pub functions: HashMap<DefinitionId, ControlFlowGraph>,
    pub type_declarations: HashMap<DefinitionId, CheckedTypeAliasDecl>,
    pub enum_declarations: HashMap<DefinitionId, EnumDecl>,
    pub global_variables: HashMap<DefinitionId, CheckedVarDecl>,
    pub initializer_cfg: Option<ControlFlowGraph>,
    pub exports: HashMap<InternerId, DefinitionId>,
}
