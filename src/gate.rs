use crate::pin::*;
use crate::types::*;
use std::collections::HashMap;

#[derive(Debug)]
pub enum GateType {
    And,
    Or,
    Not,
    Buffer,
    Chip,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Gate {
    And(AndGate),
    Or(OrGate),
    Not(NotGate),
    Buffer(BufferGate),
    Chip(Chip),
}

impl Gate {
    pub fn new(gate_type: GateType, input: Vec<PinValue>, kind: PinType) -> Self {
        static mut ID: usize = 0;
        unsafe {
            ID += 1;
            match gate_type {
                GateType::And => Gate::And(AndGate::new(input, ID, kind)),
                GateType::Or => Gate::Or(OrGate::new(input, ID)),
                GateType::Not => Gate::Not(NotGate::new(input, ID)),
                GateType::Buffer => Gate::Buffer(BufferGate::new(input, ID)),
                GateType::Chip => Gate::Chip(Chip::new()),
            }
        }
    }

    pub fn evaluate(&mut self) -> bool {
        match self {
            Gate::And(gate) => gate.evaluate(),

            Gate::Or(gate) => gate.evaluate(),

            Gate::Not(gate) => gate.evaluate(),

            Gate::Buffer(gate) => gate.evaluate(),

            Gate::Chip(chip) => chip.simulate(),
        }
    }

    fn input(&self) -> &Pins {
        match self {
            Gate::And(gate) => &gate.input,

            Gate::Or(gate) => &gate.input,

            Gate::Not(gate) => &gate.input,

            Gate::Buffer(gate) => &gate.input,

            Gate::Chip(chip) => &chip.input,
        }
    }

    fn input_mut(&mut self) -> &mut Pins {
        match self {
            Gate::And(gate) => &mut gate.input,

            Gate::Or(gate) => &mut gate.input,

            Gate::Not(gate) => &mut gate.input,

            Gate::Buffer(gate) => &mut gate.input,

            Gate::Chip(chip) => &mut chip.input,
        }
    }

    fn output(&self) -> &Pin {
        match self {
            Gate::And(gate) => &gate.output,

            Gate::Or(gate) => &gate.output,

            Gate::Not(gate) => &gate.output,

            Gate::Buffer(gate) => &gate.output,

            Gate::Chip(_) => todo!(),
        }
    }

    fn id(&self) -> usize {
        match self {
            Gate::And(gate) => gate.id,

            Gate::Or(gate) => gate.id,

            Gate::Not(gate) => gate.id,

            Gate::Buffer(gate) => gate.id,

            Gate::Chip(_) => 0,
        }
    }
}

impl PartialEq for AndGate {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialEq for OrGate {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialEq for NotGate {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialEq for BufferGate {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Debug, Clone)]
pub struct AndGate {
    id: usize,
    input: Pins,
    output: Pin,
}

impl AndGate {
    fn new(input: Vec<PinValue>, id: usize, kind: PinType) -> Self {
        if input.is_empty() {
            let pin_1 = Pin::new(PinType::Undetermined, id, 42);
            let pin_2 = Pin::new(PinType::Undetermined, id, 42);
            Self {
                id,
                input: vec![pin_1, pin_2],
                output: Pin::new(PinType::Undetermined, id, 42),
            }
        } else if input.len() == 1 {
            let pin_1 = Pin::new(kind, id, input[0]);
            Self {
                id,
                input: vec![pin_1, Pin::new(PinType::Undetermined, id, 42)],
                output: Pin::new(PinType::Undetermined, id, 42),
            }
        } else {
            let pin_1 = Pin::new(kind, id, input[0]);
            let pin_2 = Pin::new(kind, id, input[1]);
            Self {
                id,
                input: vec![pin_1, pin_2],
                output: Pin::new(PinType::Undetermined, id, 42),
            }
        }
    }

    fn evaluate(&mut self) -> bool {
        let res = (self.input[0].val.unwrap() & self.input[1].val.unwrap()) == 1;
        self.output.val = Some(res as u8);
        self.output.kind = PinType::GateOutput;
        res
    }
}

#[derive(Debug, Clone)]
pub struct OrGate {
    id: usize,
    input: Pins,
    output: Pin,
}

impl OrGate {
    fn new(input: Vec<PinValue>, id: usize) -> Self {
        let pin_1 = Pin::new(PinType::ChipInput, id, input[0]);
        let pin_2 = Pin::new(PinType::ChipInput, id, input[1]);
        Self {
            id,
            input: vec![pin_1, pin_2],
            output: Pin::new(PinType::Undetermined, id, 42),
        }
    }

    fn evaluate(&mut self) -> bool {
        let res = (self.input[0].val.unwrap() | self.input[1].val.unwrap()) == 1;
        self.output.val = Some(res as u8);
        self.output.kind = PinType::GateOutput;
        res
    }
}

#[derive(Debug, Clone)]
pub struct NotGate {
    id: usize,
    input: Pins,
    output: Pin,
}

impl NotGate {
    fn new(input: Vec<PinValue>, id: usize) -> Self {
        if input.is_empty() {
            let pin_1 = Pin::new(PinType::Undetermined, id, 42);
            Self {
                id,
                input: vec![pin_1],
                output: Pin::new(PinType::Undetermined, id, 42),
            }
        } else {
            let pin_1 = Pin::new(PinType::ChipInput, id, input[0]);
            Self {
                id,
                input: vec![pin_1],
                output: Pin::new(PinType::Undetermined, id, 42),
            }
        }
    }

    fn evaluate(&mut self) -> bool {
        let res = !(self.input[0].val.unwrap() == 1);
        self.output.val = Some(res as u8);
        self.output.kind = PinType::GateOutput;
        res
    }
}

#[derive(Debug, Clone, Eq)]
pub struct BufferGate {
    id: usize,
    input: Pins,
    output: Pin,
}

impl BufferGate {
    fn new(input: Vec<PinValue>, id: usize) -> Self {
        let pin_1 = Pin::new(PinType::ChipInput, id, input[0]);
        Self {
            id,
            input: vec![pin_1],
            output: Pin::new(PinType::Undetermined, id, 42),
        }
    }

    fn evaluate(&mut self) -> bool {
        let res = self.input[0].val.unwrap() == 1;
        self.output.val = Some(res as u8);
        self.output.kind = PinType::GateOutput;
        res
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Chip {
    gates: Gates,
    pub connections: Connections,
    input: Pins,
    output: Pins,
}

impl Chip {
    pub fn new() -> Chip {
        Chip {
            gates: Gates::new(),
            connections: HashMap::new(),
            input: Pins::new(),
            output: Pins::new(),
        }
    }

    fn add_input(&mut self, pin: Pin) {
        self.input.push(pin);
    }

    fn add_output(&mut self, pin: Pin) {
        self.output.push(pin);
    }

    fn add_gate(&mut self, gate: Gate) -> usize {
        let id = gate.id();
        self.gates.push(gate);
        id
    }

    pub fn connect_gate(&mut self, from: &mut Gate, to: &mut Gate, to_pin_idx: usize) {
        self.connections
            .entry(to.input()[to_pin_idx])
            .and_modify(|val| val.push(*from.output()))
            .or_insert_with(|| vec![*from.output()]);
        // self.connections
        //     .entry(to.input()[to_pin_idx].id)
        //     .or_insert(vec![from.output().id]);
        from.evaluate();
        to.input_mut()[to_pin_idx] =
            Pin::new(PinType::GateInput, to.id(), from.output().val.unwrap());
        if !self.gates.contains(&from) {
            // TODO: Remove this clone
            self.add_gate(from.clone());
        }
        if !self.gates.contains(&to) {
            // TODO: Remove this clone
            self.add_gate(to.clone());
        }
    }

    pub fn pins(&self) -> Vec<&Pin> {
        let mut pins = vec![];
        for gate in &self.gates {
            for pin in gate.input() {
                pins.push(pin);
            }
            pins.push(gate.output());
        }
        pins
    }

    // fn gate_dag(&self) -> HashMap<&Pin, Vec<&Pin>> {
    //     let mut dag = HashMap::new();
    //     for connection in &self.connections {
    //         for pin in *connection.1 {
    //             dag.entry(*connection.0)
    //                 .and_modify(|val: &mut Vec<&Pin>| val.push(pin))
    //                 .or_insert_with(|| vec![pin]);
    //         }
    //     }
    //     dag
    // }

    fn simulate(&self) -> bool {
        todo!()
    }
}
