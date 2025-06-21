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
    fn resolve_type_at_entry(
        &self,
        node_id: TFGNodeId,
        var_id: DefinitionId,
        graph: &TypeFlowGraph,
        visited: &mut HashSet<TFGNodeId>,
        cache: &mut HashMap<(TFGNodeId, DefinitionId), Option<Option<CheckedTypeKind>>>,
    ) -> Option<Option<CheckedTypeKind>> {
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

        let mut merged_result: Option<Option<CheckedTypeKind>> = None;

        for pred_id in &tfg_node.predecessors {
            let pred_node_data = graph.get_node(*pred_id).expect("Predecessor node must exist.");

            let path_result = match &pred_node_data.kind {
                TFGNodeKind::Narrowing { narrowing, .. } if narrowing.target == var_id => {
                    Some(Some(narrowing.narrowed_type.clone()))
                }
                TFGNodeKind::BranchNarrowing {
                    narrowing_if_true,
                    next_node_if_true,
                    narrowing_if_false,
                    next_node_if_false,
                    ..
                } => {
                    let mut result = None;
                    if next_node_if_true == &Some(node_id) {
                        if let Some(ni) = narrowing_if_true {
                            if ni.target == var_id {
                                result = Some(Some(ni.narrowed_type.clone()));
                            }
                        }
                    } else if next_node_if_false == &Some(node_id) {
                        if let Some(ni) = narrowing_if_false {
                            if ni.target == var_id {
                                result = Some(Some(ni.narrowed_type.clone()));
                            }
                        }
                    }

                    if result.is_some() {
                        result
                    } else {
                        self.resolve_type_at_entry(*pred_id, var_id, graph, visited, cache)
                    }
                }
                TFGNodeKind::Exit => None,
                _ => self.resolve_type_at_entry(*pred_id, var_id, graph, visited, cache),
            };

            merged_result = match (merged_result, path_result) {
                (Some(None), _) => Some(None),
                (_, Some(None)) => Some(None),
                (None, new_res) => new_res,
                (Some(Some(merged_type)), Some(Some(path_type))) => {
                    if merged_type == path_type {
                        Some(Some(merged_type))
                    } else {
                        Some(None)
                    }
                }
                (current_res @ Some(Some(_)), None) => current_res,
            };
        }

        visited.remove(&node_id);
        cache.insert((node_id, var_id), merged_result.clone());
        merged_result
    }

    pub fn get_definition_id_type(&self, current_node_id: TFGNodeId, id: DefinitionId) -> Option<CheckedTypeKind> {
        let tfg_context = self.tfg_contexts.last().expect("TFG context stack should not be empty.");
        let graph = &tfg_context.graph;

        let mut cache: HashMap<(TFGNodeId, DefinitionId), Option<Option<CheckedTypeKind>>> = HashMap::new();

        let current_node_data = graph.get_node(current_node_id).expect("Current TFG node must exist.");

        let resolution = match &current_node_data.kind {
            TFGNodeKind::Narrowing { narrowing, .. } if narrowing.target == id => Some(Some(narrowing.narrowed_type.clone())),
            TFGNodeKind::Exit => None,
            _ => self.resolve_type_at_entry(current_node_id, id, graph, &mut HashSet::new(), &mut cache),
        };

        match resolution {
            Some(Some(concrete_type)) => Some(concrete_type),
            Some(None) => None,
            None => None,
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
