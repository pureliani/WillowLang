use crate::{
    cfg::{Value, ValueId},
    hir_builder::{
        types::checked_type::{Type, TypeKind},
        FunctionBuilder,
    },
    tokenize::NumberKind,
};

impl FunctionBuilder {
    pub fn get_value_id_type(&self, value_id: &ValueId) -> Type {
        self.cfg
            .value_types
            .get(value_id)
            .expect("INTERNAL COMPILER ERROR: All ValueIds must have a corresponding type")
            .clone()
    }

    pub fn get_value_type(&self, value: &Value) -> Type {
        match value {
            Value::VoidLiteral => Type {
                kind: TypeKind::Void,
                span: Default::default(),
            },
            Value::BoolLiteral(_) => Type {
                kind: TypeKind::Bool,
                // TODO: fix this later
                span: Default::default(),
            },
            Value::NumberLiteral(kind) => {
                let kind = match kind {
                    NumberKind::I64(_) => TypeKind::I64,
                    NumberKind::I32(_) => TypeKind::I32,
                    NumberKind::I16(_) => TypeKind::I16,
                    NumberKind::I8(_) => TypeKind::I8,
                    NumberKind::F32(_) => TypeKind::F32,
                    NumberKind::F64(_) => TypeKind::F64,
                    NumberKind::U64(_) => TypeKind::U64,
                    NumberKind::U32(_) => TypeKind::U32,
                    NumberKind::U16(_) => TypeKind::U16,
                    NumberKind::U8(_) => TypeKind::U8,
                    NumberKind::USize(_) => TypeKind::USize,
                    NumberKind::ISize(_) => TypeKind::ISize,
                };

                Type {
                    kind,
                    span: Default::default(),
                }
            }
            Value::StringLiteral(_) => Type {
                kind: TypeKind::String,
                span: Default::default(),
            },
            Value::FunctionAddr { ty, .. } => ty.clone(),
            Value::Use(value_id) => self.get_value_id_type(value_id),
        }
    }
}
