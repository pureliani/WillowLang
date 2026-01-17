use crate::{
    ast::DeclarationId,
    hir::{
        cfg::{BinaryOperationKind, ConstantId, UnaryOperationKind, ValueId},
        types::checked_type::Type,
    },
    tokenize::NumberKind,
};

#[derive(Clone, Debug)]
pub enum Instruction {
    // Constants
    ConstInt {
        dest: ValueId,
        val: NumberKind,
    },
    ConstBool {
        dest: ValueId,
        val: bool,
    },
    ConstString {
        dest: ValueId,
        constant_id: ConstantId,
    },
    ConstFn {
        dest: ValueId,
        decl_id: DeclarationId,
    },
    ConstVoid {
        dest: ValueId,
    },
    // Operations
    UnaryOp {
        op_kind: UnaryOperationKind,
        destination: ValueId,
        operand: ValueId,
    },
    BinaryOp {
        op_kind: BinaryOperationKind,
        destination: ValueId,
        left: ValueId,
        right: ValueId,
    },
    // Memory
    StackAlloc {
        destination: ValueId,
        count: usize,
    },
    HeapAlloc {
        destination: ValueId,
        count: ValueId,
    },
    HeapFree {
        ptr: ValueId,
    },
    Store {
        ptr: ValueId,
        value: ValueId,
    },
    Load {
        destination: ValueId,
        ptr: ValueId,
    },
    // Access
    GetFieldPtr {
        destination: ValueId,
        base_ptr: ValueId,
        field_index: usize,
    },
    GetElementPtr {
        destination: ValueId,
        base_ptr: ValueId,
        index: ValueId,
    },
    // Calls
    FunctionCall {
        destination: Option<ValueId>,
        func: ValueId,
        args: Vec<ValueId>,
    },
    TypeCast {
        destination: ValueId,
        operand: ValueId,
        target_type: Type,
    },
    Nop,
}
