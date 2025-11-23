use crate::pin::*;
use crate::types::*;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GateType {
    And,
    Not,
    Source,
    Output,
    Chip,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Gate {
    And(AndGate),
    Not(NotGate),
    Source(SourceGate),
    Output(OutputGate),
    Chip(crate::circuit::Chip),
}

impl Gate {
    pub fn new(gate_type: GateType, _input: Vec<PinValue>) -> Self {
        // FIX: Use global counter
        let id = next_uuid();
        match gate_type {
            GateType::And => Gate::And(AndGate::new(id)),
            GateType::Not => Gate::Not(NotGate::new(id)),
            GateType::Source => Gate::Source(SourceGate::new(id)),
            GateType::Output => Gate::Output(OutputGate::new(id)),
            GateType::Chip => Gate::Chip(crate::circuit::Chip::new(id)),
        }
    }

    pub fn clone_with_new_ids(&self, id_map: &mut HashMap<usize, usize>) -> Self {
        // FIX: Use global counter
        let new_id = next_uuid();

        match self {
            Gate::And(_) => {
                let new_gate = AndGate::new(new_id);
                let old_inputs = self.input();
                let new_inputs = new_gate.input.clone();
                for (old, new) in old_inputs.iter().zip(new_inputs.iter()) {
                    id_map.insert(*old, *new);
                }
                let old_outputs = self.output();
                let new_outputs = new_gate.output.clone();
                for (old, new) in old_outputs.iter().zip(new_outputs.iter()) {
                    id_map.insert(*old, *new);
                }
                Gate::And(new_gate)
            }
            Gate::Not(_) => {
                let new_gate = NotGate::new(new_id);
                let old_inputs = self.input();
                let new_inputs = new_gate.input.clone();
                for (old, new) in old_inputs.iter().zip(new_inputs.iter()) {
                    id_map.insert(*old, *new);
                }
                let old_outputs = self.output();
                let new_outputs = new_gate.output.clone();
                for (old, new) in old_outputs.iter().zip(new_outputs.iter()) {
                    id_map.insert(*old, *new);
                }
                Gate::Not(new_gate)
            }
            Gate::Chip(c) => {
                let mut new_chip = c.deep_copy();
                new_chip.id = new_id;
                for (old, new) in c.input.iter().zip(new_chip.input.iter()) {
                    id_map.insert(*old, *new);
                }
                for (old, new) in c.output.iter().zip(new_chip.output.iter()) {
                    id_map.insert(*old, *new);
                }
                Gate::Chip(new_chip)
            }
            _ => panic!("Cannot clone Source/Output gates into a chip"),
        }
    }

    pub fn evaluate(&mut self) -> bool {
        match self {
            Gate::And(g) => g.evaluate(),
            Gate::Not(g) => g.evaluate(),
            Gate::Source(_) => false,
            Gate::Output(g) => g.evaluate(),
            Gate::Chip(c) => c.simulate(),
        }
    }

    pub fn set_pin(&mut self, id: &usize, val: Option<PinValue>) {
        match self {
            Gate::And(g) => g.set_pin(id, val),
            Gate::Not(g) => g.set_pin(id, val),
            Gate::Source(g) => g.set_pin(id, val),
            Gate::Output(g) => g.set_pin(id, val),
            Gate::Chip(c) => c.set_pin(id, val),
        }
    }

    pub fn pins(&self) -> &HashMap<usize, Pin> {
        match self {
            Gate::And(g) => &g.pins,
            Gate::Not(g) => &g.pins,
            Gate::Source(g) => &g.pins,
            Gate::Output(g) => &g.pins,
            Gate::Chip(c) => &c.pins,
        }
    }

    pub fn id(&self) -> usize {
        match self {
            Gate::And(g) => g.id,
            Gate::Not(g) => g.id,
            Gate::Source(g) => g.id,
            Gate::Output(g) => g.id,
            Gate::Chip(c) => c.id,
        }
    }

    pub fn input(&self) -> &[usize] {
        match self {
            Gate::And(g) => &g.input,
            Gate::Not(g) => &g.input,
            Gate::Source(_) => &[],
            Gate::Output(g) => &g.input,
            Gate::Chip(c) => &c.input,
        }
    }

    pub fn output(&self) -> &[usize] {
        match self {
            Gate::And(g) => &g.output,
            Gate::Not(g) => &g.output,
            Gate::Source(g) => &g.output,
            Gate::Output(_) => &[],
            Gate::Chip(c) => &c.output,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SourceGate {
    pub id: usize,
    pub pins: HashMap<usize, Pin>,
    pub output: Vec<usize>,
}

impl SourceGate {
    pub fn new(id: usize) -> Self {
        let pin = Pin::new(PinType::GateOutput, id, 0);
        let mut pins = HashMap::new();
        pins.insert(pin.id, pin);
        Self {
            id,
            pins,
            output: vec![pin.id],
        }
    }
    pub fn set_pin(&mut self, id: &usize, val: Option<PinValue>) {
        if let Some(p) = self.pins.get_mut(id) {
            p.val = val;
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct OutputGate {
    pub id: usize,
    pub pins: HashMap<usize, Pin>,
    pub input: Vec<usize>,
}

impl OutputGate {
    pub fn new(id: usize) -> Self {
        let pin = Pin::new(PinType::GateInput, id, 0);
        let mut pins = HashMap::new();
        pins.insert(pin.id, pin);
        Self {
            id,
            pins,
            input: vec![pin.id],
        }
    }
    pub fn evaluate(&mut self) -> bool {
        self.pins
            .get(&self.input[0])
            .and_then(|p| p.val)
            .unwrap_or(0)
            == 1
    }
    pub fn set_pin(&mut self, id: &usize, val: Option<PinValue>) {
        if let Some(p) = self.pins.get_mut(id) {
            p.val = val;
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AndGate {
    pub id: usize,
    pub pins: HashMap<usize, Pin>,
    pub input: Vec<usize>,
    pub output: Vec<usize>,
}

impl AndGate {
    pub fn new(id: usize) -> Self {
        let p1 = Pin::new(PinType::GateInput, id, 0);
        let p2 = Pin::new(PinType::GateInput, id, 0);
        let out = Pin::new(PinType::GateOutput, id, 42);
        let mut pins = HashMap::new();
        pins.insert(p1.id, p1);
        pins.insert(p2.id, p2);
        pins.insert(out.id, out);
        Self {
            id,
            pins,
            input: vec![p1.id, p2.id],
            output: vec![out.id],
        }
    }
    pub fn evaluate(&mut self) -> bool {
        let v1 = self
            .pins
            .get(&self.input[0])
            .and_then(|p| p.val)
            .unwrap_or(0);
        let v2 = self
            .pins
            .get(&self.input[1])
            .and_then(|p| p.val)
            .unwrap_or(0);
        let res = (v1 & v2) == 1;
        if let Some(p) = self.pins.get_mut(&self.output[0]) {
            p.val = Some(res as u8);
        }
        res
    }
    pub fn set_pin(&mut self, id: &usize, val: Option<PinValue>) {
        if let Some(p) = self.pins.get_mut(id) {
            p.val = val;
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NotGate {
    pub id: usize,
    pub pins: HashMap<usize, Pin>,
    pub input: Vec<usize>,
    pub output: Vec<usize>,
}

impl NotGate {
    pub fn new(id: usize) -> Self {
        let p1 = Pin::new(PinType::GateInput, id, 0);
        let out = Pin::new(PinType::GateOutput, id, 42);
        let mut pins = HashMap::new();
        pins.insert(p1.id, p1);
        pins.insert(out.id, out);
        Self {
            id,
            pins,
            input: vec![p1.id],
            output: vec![out.id],
        }
    }
    pub fn evaluate(&mut self) -> bool {
        let v1 = self
            .pins
            .get(&self.input[0])
            .and_then(|p| p.val)
            .unwrap_or(0);
        let res = v1 == 0;
        if let Some(p) = self.pins.get_mut(&self.output[0]) {
            p.val = Some(res as u8);
        }
        res
    }
    pub fn set_pin(&mut self, id: &usize, val: Option<PinValue>) {
        if let Some(p) = self.pins.get_mut(id) {
            p.val = val;
        }
    }
}
