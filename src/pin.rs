use crate::types::*;
use std::sync::atomic::{AtomicUsize, Ordering};

// --- GLOBAL ID GENERATOR ---
// This ensures every single object (Gate, Pin, Chip Instance) gets a unique number.
static GLOBAL_ID_COUNTER: AtomicUsize = AtomicUsize::new(1);

pub fn next_uuid() -> usize {
    GLOBAL_ID_COUNTER.fetch_add(1, Ordering::Relaxed)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PinType {
    GateInput,
    GateOutput,
    ChipInput,
    ChipOutput,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pin {
    pub id: usize,
    pub kind: PinType,
    pub val: Option<PinValue>,
}

impl Pin {
    pub fn new(kind: PinType, _gate_id: usize, val: PinValue) -> Self {
        // FIX: Use global counter
        let id = next_uuid();
        let val_opt = if val == 42 { None } else { Some(val) };
        Pin {
            id,
            kind,
            val: val_opt,
        }
    }
}
