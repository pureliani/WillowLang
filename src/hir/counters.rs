use crate::hir::cfg::{BasicBlockId, ValueId};
use crate::hir::types::checked_type::Type;
use std::sync::atomic::{AtomicUsize, Ordering};

static VALUE_COUNTER: AtomicUsize = AtomicUsize::new(0);
static BLOCK_COUNTER: AtomicUsize = AtomicUsize::new(0);
static CONSTANT_COUNTER: AtomicUsize = AtomicUsize::new(0);
static DECLARATION_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub fn next_value_id() -> ValueId {
    ValueId(VALUE_COUNTER.fetch_add(1, Ordering::SeqCst))
}

pub fn next_block_id() -> BasicBlockId {
    BasicBlockId(BLOCK_COUNTER.fetch_add(1, Ordering::SeqCst))
}

pub fn next_constant_id() -> BasicBlockId {
    BasicBlockId(CONSTANT_COUNTER.fetch_add(1, Ordering::SeqCst))
}

pub fn next_declaration_id() -> BasicBlockId {
    BasicBlockId(DECLARATION_COUNTER.fetch_add(1, Ordering::SeqCst))
}

pub fn reset_counters() {
    VALUE_COUNTER.store(0, Ordering::SeqCst);
    BLOCK_COUNTER.store(0, Ordering::SeqCst);
    CONSTANT_COUNTER.store(0, Ordering::SeqCst);
    DECLARATION_COUNTER.store(0, Ordering::SeqCst);
}
