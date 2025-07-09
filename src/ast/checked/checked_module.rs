use std::collections::HashMap;

use crate::{
    ast::{
        base::base_declaration::EnumDecl,
        checked::checked_declaration::{CheckedTypeAliasDecl, CheckedVarDecl},
        DefinitionId, ModuleId,
    },
    cfg::ControlFlowGraph,
    compile::string_interner::InternerId,
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
