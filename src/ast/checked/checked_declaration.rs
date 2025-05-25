use std::hash::{Hash, Hasher};

use crate::{ast::IdentifierNode, parse::DocAnnotation};

use super::{checked_expression::CheckedExpr, checked_type::CheckedType};

#[derive(Clone, Debug)]
pub struct CheckedParam {
    pub identifier: IdentifierNode,
    pub constraint: CheckedType,
}

impl CheckedParam {
    pub fn to_string(&self) -> String {
        format!(
            "{}: {}",
            self.identifier.name.clone(),
            self.constraint.to_string()
        )
    }
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

impl CheckedGenericParam {
    pub fn to_string(&self) -> String {
        match &self.constraint {
            Some(c) => {
                format!("{}: {}", self.identifier.name.clone(), c.to_string())
            }
            None => {
                format!("{}", self.identifier.name.clone())
            }
        }
    }
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
pub struct CheckedGenericStructDecl {
    pub identifier: IdentifierNode,
    pub documentation: Option<DocAnnotation>,
    pub generic_params: Vec<CheckedGenericParam>,
    pub properties: Vec<CheckedParam>,
}

impl CheckedGenericStructDecl {
    pub fn to_string(&self) -> String {
        let generic_params_str = if !self.generic_params.is_empty() {
            let joined = self
                .generic_params
                .iter()
                .map(|gp| gp.to_string())
                .collect::<Vec<String>>()
                .join(", ");

            format!("<{}>", joined)
        } else {
            "".to_owned()
        };

        let params_str = {
            let joined = self
                .properties
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<String>>()
                .join(",\n");

            format!("{{{}}}", joined)
        };

        format!(
            "struct {}{} {}",
            self.identifier.name, generic_params_str, params_str
        )
    }
}

impl Eq for CheckedGenericStructDecl {}
impl PartialEq for CheckedGenericStructDecl {
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier
            && self.generic_params == other.generic_params
            && self.properties == other.properties
    }
}
impl Hash for CheckedGenericStructDecl {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.identifier.hash(state);
        self.generic_params.hash(state);
        self.properties.hash(state);
    }
}

#[derive(Clone, Debug)]
pub struct CheckedStructDecl {
    pub identifier: IdentifierNode,
    pub documentation: Option<DocAnnotation>,
    pub properties: Vec<CheckedParam>,
}

impl CheckedStructDecl {
    pub fn to_string(&self) -> String {
        let params_str = {
            let joined = self
                .properties
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<String>>()
                .join(",\n");

            format!("{{{}}}", joined)
        };

        format!("struct {} {}", self.identifier.name, params_str)
    }
}

impl Eq for CheckedStructDecl {}
impl PartialEq for CheckedStructDecl {
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier && self.properties == other.properties
    }
}
impl Hash for CheckedStructDecl {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.identifier.hash(state);
        self.properties.hash(state);
    }
}

#[derive(Clone, Debug)]
pub struct CheckedGenericTypeAliasDecl {
    pub identifier: IdentifierNode,
    pub documentation: Option<DocAnnotation>,
    pub generic_params: Vec<CheckedGenericParam>,
    pub value: Box<CheckedType>,
}

impl CheckedGenericTypeAliasDecl {
    pub fn to_string(&self) -> String {
        let generic_params_str = if !self.generic_params.is_empty() {
            let joined = self
                .generic_params
                .iter()
                .map(|gp| gp.to_string())
                .collect::<Vec<String>>()
                .join(", ");

            format!("<{}>", joined)
        } else {
            "".to_owned()
        };

        format!(
            "type {}{} = {}",
            self.identifier.name,
            generic_params_str,
            self.value.to_string()
        )
    }
}

impl Eq for CheckedGenericTypeAliasDecl {}
impl PartialEq for CheckedGenericTypeAliasDecl {
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier
            && self.generic_params == other.generic_params
            && self.value == other.value
    }
}
impl Hash for CheckedGenericTypeAliasDecl {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.identifier.hash(state);
        self.generic_params.hash(state);
        self.value.hash(state);
    }
}

#[derive(Clone, Debug)]
pub struct CheckedTypeAliasDecl {
    pub identifier: IdentifierNode,
    pub documentation: Option<DocAnnotation>,
    pub value: Box<CheckedType>,
}

impl CheckedTypeAliasDecl {
    pub fn to_string(&self) -> String {
        format!("type {} = {}", self.identifier.name, self.value.to_string())
    }
}

impl Eq for CheckedTypeAliasDecl {}
impl PartialEq for CheckedTypeAliasDecl {
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier && self.value == other.value
    }
}
impl Hash for CheckedTypeAliasDecl {
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
