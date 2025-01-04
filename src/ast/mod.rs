pub mod base;
pub mod checked;
pub mod monomorphized;

#[derive(Debug, Clone, PartialEq)]
pub struct IdentifierNode {
    pub name: String,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StringNode {
    pub value: String,
    pub span: Span,
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
