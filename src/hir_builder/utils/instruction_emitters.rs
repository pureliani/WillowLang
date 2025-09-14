use crate::{
    cfg::{BasicBlock, Instruction, ValueId},
    hir_builder::{
        types::checked_type::{Type, TypeKind},
        FunctionBuilder, ModuleBuilder,
    },
};

impl FunctionBuilder {
    pub fn get_current_basic_block(&mut self) -> &mut BasicBlock {
        self.cfg.blocks.get_mut(&self.current_block_id).unwrap_or_else(|| {
            panic!(
                "INTERNAL COMPILER ERROR: Basic block with id '{}' does not exist.",
                self.current_block_id.0
            )
        })
    }

    /// Returns ValueId which holds pointer: TypeKind::Pointer(Box<Type>)
    pub fn emit_alloc(&mut self, ty: Type) -> ValueId {
        let destination = self.new_value_id();

        self.cfg.value_types.insert(
            destination,
            Type {
                span: ty.span,
                kind: TypeKind::Pointer(Box::new(ty)),
            },
        );

        self.get_current_basic_block()
            .instructions
            .push(Instruction::Alloc { destination });

        destination
    }

    /// Returns ValueId which holds pointer: TypeKind::Pointer(Box<Type>)
    pub fn emit_new(&mut self, ty: Type) -> ValueId {
        let destination = self.new_value_id();
        let allocation_id = todo!(); // TODO: get globally unique id

        self.cfg.value_types.insert(
            destination,
            Type {
                span: ty.span,
                kind: TypeKind::Pointer(Box::new(ty)),
            },
        );

        self.get_current_basic_block().instructions.push(Instruction::New {
            destination,
            allocation_site_id: allocation_id,
        });

        destination
    }

    pub fn emit_store(&mut self, module_builder: &mut ModuleBuilder) {
        todo!()
    }

    pub fn emit_load(&mut self, module_builder: &mut ModuleBuilder) {
        todo!()
    }

    pub fn emit_get_field_ptr(&mut self, module_builder: &mut ModuleBuilder) {
        todo!()
    }

    pub fn emit_get_element_ptr(&mut self, module_builder: &mut ModuleBuilder) {
        todo!()
    }

    pub fn emit_unary_op(&mut self, module_builder: &mut ModuleBuilder) {
        todo!()
    }

    pub fn emit_binary_op(&mut self, module_builder: &mut ModuleBuilder) {
        todo!()
    }

    pub fn emit_phi(&mut self, module_builder: &mut ModuleBuilder) {
        todo!()
    }

    pub fn emit_function_call(&mut self, module_builder: &mut ModuleBuilder) {
        todo!()
    }

    pub fn emit_type_cast(&mut self, module_builder: &mut ModuleBuilder) {
        todo!()
    }

    pub fn emit_nop(&mut self, module_builder: &mut ModuleBuilder) {
        todo!()
    }
}
