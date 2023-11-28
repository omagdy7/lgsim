use crate::types::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PinType {
    Undetermined,
    ChipInput,
    GateInput,
    GateOutput,
    ChipOutput,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pin {
    pub id: usize,
    pub kind: PinType,
    pub gate_id: usize,
    pub val: Option<PinValue>,
}

impl Pin {
    pub fn new(kind: PinType, gate_id: usize, val: PinValue) -> Self {
        static mut ID: usize = 0;
        unsafe {
            ID += 1;
            // println!("Creating pin with id: {} and gate_id: {}", ID, gate_id);
            match kind {
                PinType::Undetermined => Pin {
                    id: ID,
                    gate_id,
                    kind,
                    val: None,
                },
                _ => Pin {
                    id: ID,
                    gate_id,
                    kind,
                    val: Some(val),
                },
            }
        }
    }
}
