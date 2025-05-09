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
