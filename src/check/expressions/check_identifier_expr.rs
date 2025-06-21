use std::collections::{HashMap, HashSet};

use crate::{
    ast::{
        checked::{
            checked_expression::{CheckedExpr, CheckedExprKind},
            checked_type::{CheckedType, CheckedTypeKind},
        },
        DefinitionId, IdentifierNode, Span,
    },
    check::{utils::scope::SymbolEntry, SemanticChecker, SemanticError},
    tfg::{TFGNodeId, TFGNodeKind, TypeFlowGraph},
};

impl<'a> SemanticChecker<'a> {
    fn get_type_at_entry_of_node(
        &self,
        node_id: TFGNodeId,
        var_id: DefinitionId,
        graph: &TypeFlowGraph,
        visited: &mut HashSet<TFGNodeId>,
        cache: &mut HashMap<(TFGNodeId, DefinitionId), Option<CheckedTypeKind>>,
    ) -> Option<CheckedTypeKind> {
        if let Some(cached_result) = cache.get(&(node_id, var_id)) {
            return cached_result.clone();
        }

        if visited.contains(&node_id) {
            return None;
        }
        visited.insert(node_id);

        let tfg_node = graph.get_node(node_id).expect("Node must exist in TFG graph.");

        if tfg_node.predecessors.is_empty() {
            visited.remove(&node_id);
            cache.insert((node_id, var_id), None);
            return None;
        }

        let mut merged_type_so_far: Option<CheckedTypeKind> = None;
        let mut is_first_pred_path = true;

        for pred_id in &tfg_node.predecessors {
            let pred_node_data = graph.get_node(*pred_id).expect("Predecessor node must exist.");
            let mut type_from_this_pred_path: Option<CheckedTypeKind> = None;

            match &pred_node_data.kind {
                TFGNodeKind::Entry { .. } => {
                    type_from_this_pred_path = self.get_type_at_entry_of_node(*pred_id, var_id, graph, visited, cache);
                }
                TFGNodeKind::Narrowing { narrowing, .. } => {
                    if narrowing.target == var_id {
                        type_from_this_pred_path = Some(narrowing.narrowed_type.clone());
                    } else {
                        type_from_this_pred_path = self.get_type_at_entry_of_node(*pred_id, var_id, graph, visited, cache);
                    }
                }
                TFGNodeKind::BranchNarrowing {
                    narrowing_if_true,
                    next_node_if_true,
                    narrowing_if_false,
                    next_node_if_false,
                } => {
                    let mut narrowed_on_this_branch = false;

                    if next_node_if_true == &Some(node_id) {
                        if let Some(ref ni_true) = narrowing_if_true {
                            if ni_true.target == var_id {
                                type_from_this_pred_path = Some(ni_true.narrowed_type.clone());
                                narrowed_on_this_branch = true;
                            }
                        }
                    } else if next_node_if_false == &Some(node_id) {
                        if let Some(ref ni_false) = narrowing_if_false {
                            if ni_false.target == var_id {
                                type_from_this_pred_path = Some(ni_false.narrowed_type.clone());
                                narrowed_on_this_branch = true;
                            }
                        }
                    }

                    if !narrowed_on_this_branch {
                        type_from_this_pred_path = self.get_type_at_entry_of_node(*pred_id, var_id, graph, visited, cache);
                    }
                }
                TFGNodeKind::NoOp { .. } => {
                    type_from_this_pred_path = self.get_type_at_entry_of_node(*pred_id, var_id, graph, visited, cache);
                }
                TFGNodeKind::Exit => {
                    type_from_this_pred_path = None;
                }
            }

            if is_first_pred_path {
                merged_type_so_far = type_from_this_pred_path;
                is_first_pred_path = false;
            } else {
                if merged_type_so_far != type_from_this_pred_path {
                    merged_type_so_far = None;
                    break;
                }
            }
        }

        visited.remove(&node_id);
        cache.insert((node_id, var_id), merged_type_so_far.clone());
        merged_type_so_far
    }

    pub fn get_definition_id_type(&self, current_node_id: TFGNodeId, id: DefinitionId) -> Option<CheckedTypeKind> {
        let tfg_context = self.tfg_contexts.last().expect("TFG context stack should not be empty.");
        let graph = &tfg_context.graph;

        let mut memo: HashMap<(TFGNodeId, DefinitionId), Option<CheckedTypeKind>> = HashMap::new();

        let current_node_data = graph.get_node(current_node_id).expect("Current TFG node must exist.");

        match &current_node_data.kind {
            TFGNodeKind::Narrowing { narrowing, .. } => {
                if narrowing.target == id {
                    Some(narrowing.narrowed_type.clone())
                } else {
                    self.get_type_at_entry_of_node(current_node_id, id, graph, &mut HashSet::new(), &mut memo)
                }
            }

            TFGNodeKind::BranchNarrowing { .. } | TFGNodeKind::NoOp { .. } | TFGNodeKind::Entry { .. } => {
                self.get_type_at_entry_of_node(current_node_id, id, graph, &mut HashSet::new(), &mut memo)
            }
            TFGNodeKind::Exit => None,
        }
    }

    pub fn check_identifier_expr(&mut self, id: IdentifierNode, span: Span) -> CheckedExpr {
        let entry = self.scope_lookup(id.name);

        let kind = match entry {
            Some(e) => match e {
                SymbolEntry::StructDecl(decl) => CheckedTypeKind::StructDecl(decl.clone()),
                SymbolEntry::TypeAliasDecl(decl) => CheckedTypeKind::TypeAliasDecl(decl.clone()),
                SymbolEntry::EnumDecl(decl) => CheckedTypeKind::EnumDecl(decl.clone()),
                SymbolEntry::VarDecl(decl) => {
                    let target = decl.borrow();
                    let mut constraint_kind: CheckedTypeKind = decl.borrow().constraint.kind.clone();

                    if let Some(ctx) = self.tfg_contexts.last() {
                        if let Some(current_node) = ctx.graph.get_node(ctx.current_node) {
                            if let Some(kind) = self.get_definition_id_type(current_node.id, target.id) {
                                constraint_kind = kind
                            }
                        }
                    }

                    constraint_kind
                }
                SymbolEntry::GenericParam(_) => {
                    self.errors.push(SemanticError::CannotUseGenericParameterAsValue { span });

                    CheckedTypeKind::Unknown
                }
            },
            None => {
                self.errors.push(SemanticError::UndeclaredIdentifier { id });

                CheckedTypeKind::Unknown
            }
        };

        CheckedExpr {
            ty: CheckedType { kind, span },
            kind: CheckedExprKind::Identifier(id),
        }
    }
}
