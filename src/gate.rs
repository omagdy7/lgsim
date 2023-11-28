use crate::pin::*;
use crate::types::*;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
pub enum GateType {
    And,
    Not,
    Chip,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Gate {
    And(AndGate),
    Not(NotGate),
    Chip(Chip),
}

impl Gate {
    pub fn new(gate_type: GateType, input: Vec<PinValue>, kind: PinType) -> Self {
        static mut ID: usize = 0;
        unsafe {
            ID += 1;
            match gate_type {
                GateType::And => Gate::And(AndGate::new(input, ID, kind)),
                GateType::Not => Gate::Not(NotGate::new(input, ID)),
                GateType::Chip => Gate::Chip(Chip::new()),
            }
        }
    }

    pub fn evaluate(&mut self) -> bool {
        match self {
            Gate::And(gate) => gate.evaluate(),
            Gate::Not(gate) => gate.evaluate(),
            Gate::Chip(chip) => chip.simulate(),
        }
    }

    fn add_input(&mut self, val: PinValue, connections: &mut Connections) -> usize {
        match self {
            Gate::And(gate) => gate.add_input(val, connections),
            Gate::Not(gate) => gate.add_input(val, connections),
            Gate::Chip(_) => todo!(),
        }
    }

    fn pins(&self) -> &HashMap<usize, Pin> {
        match self {
            Gate::And(gate) => &gate.pins,
            Gate::Not(gate) => &gate.pins,
            Gate::Chip(_) => todo!(),
        }
    }

    fn pins_mut(&mut self) -> &mut HashMap<usize, Pin> {
        match self {
            Gate::And(gate) => &mut gate.pins,
            Gate::Not(gate) => &mut gate.pins,
            Gate::Chip(_) => todo!(),
        }
    }

    fn set_pin(&mut self, id: &usize, val: Option<PinValue>) {
        match self {
            Gate::And(gate) => gate.set_pin(id, val),
            Gate::Not(gate) => gate.set_pin(id, val),
            Gate::Chip(_) => todo!(),
        }
    }

    fn input(&self) -> &Vec<usize> {
        match self {
            Gate::And(gate) => &gate.input,
            Gate::Not(gate) => &gate.input,
            Gate::Chip(chip) => &chip.input,
        }
    }

    pub fn pin(&self, id: usize) -> &Pin {
        match self {
            Gate::And(gate) => gate.pin(id),
            Gate::Not(gate) => gate.pin(id),
            Gate::Chip(_) => todo!(),
        }
    }

    fn pin_mut(&mut self, id: usize) -> &Pin {
        match self {
            Gate::And(gate) => gate.pin_mut(id),
            Gate::Not(gate) => gate.pin_mut(id),
            Gate::Chip(_) => todo!(),
        }
    }

    fn output(&self) -> &usize {
        match self {
            Gate::And(gate) => &gate.output,
            Gate::Not(gate) => &gate.output,
            Gate::Chip(_) => todo!(),
        }
    }

    pub fn id(&self) -> usize {
        match self {
            Gate::And(gate) => gate.id,
            Gate::Not(gate) => gate.id,
            Gate::Chip(_) => 0,
        }
    }
}

impl PartialEq for AndGate {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialEq for NotGate {
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
            let pin_1 = Pin::new(PinType::GateInput, id, 42);
            let pin_2 = Pin::new(PinType::GateInput, id, 42);
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
        println!("{:?}", self);
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
    pub gates: Gates,
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
        to.input_mut()[to_pin_idx] = Pin::new(PinType::GateInput, to.id(), 42);
        self.connections
            .entry(to.input()[to_pin_idx])
            .and_modify(|val| val.push(*from.output()))
            .or_insert_with(|| vec![*from.output()]);
        if !self.gates.contains(&from) {
            // TODO: Remove this clone
            self.add_gate(from.clone());
        }
        if !self.gates.contains(&to) {
            // TODO: Remove this clone
            self.add_gate(to.clone());
        }
    }

    pub fn pins(&self) -> HashMap<usize, Vec<&Pin>> {
        let mut pins = HashMap::new();
        for gate in &self.gates {
            for pin in gate.input() {
                pins.entry(pin.id)
                    .and_modify(|val: &mut Vec<&Pin>| val.push(pin))
                    .or_insert_with(|| vec![pin]);
            }
            pins.entry(gate.output().id)
                .and_modify(|val: &mut Vec<&Pin>| val.push(gate.output()))
                .or_insert_with(|| vec![gate.output()]);
        }
        pins
    }

    pub fn gate_dag(&self) -> HashMap<usize, Vec<usize>> {
        let mut dag = HashMap::new();
        for connection in &self.connections {
            for pin in connection.1 {
                dag.entry(connection.0.gate_id)
                    .and_modify(|val: &mut Vec<usize>| val.push(pin.gate_id))
                    .or_insert_with(|| vec![pin.gate_id]);
            }
        }
        dag
    }

    pub fn pin_dag(&self) -> HashMap<usize, Vec<usize>> {
        let mut gate_dag = self.gate_dag();
        let mut dag = HashMap::new();
        for (id, children) in gate_dag {
            // dag.entry()
            //     .and_modify(|val: &mut Vec<usize>| val.push(pin.gate_id))
            //     .or_insert_with(|| vec![pin.gate_id]);
        }
        dag
    }

    fn simulate_helper(
        &mut self,
        gate: usize,
        visited: &mut HashSet<usize>,
        dag: &HashMap<usize, Vec<usize>>,
    ) {
        visited.insert(gate);
        if dag.contains_key(&gate) {
            for child in dag.get(&gate).unwrap() {
                if !visited.contains(child) {
                    self.simulate_helper(*child, visited, dag);
                }
            }
            let pos = self.gates.iter().position(|g| g.id() == gate).unwrap();
            self.gates[pos].evaluate();
            println!("{}", gate);
        } else {
            println!("{}", gate);
            let pos = self.gates.iter().position(|g| g.id() == gate).unwrap();
            self.gates[pos].evaluate();
        }
    }

    pub fn simulate(&mut self) -> bool {
        let dag = self.gate_dag();
        let mut visited = HashSet::new();
        for child in dag.get(&8).unwrap() {
            if !visited.contains(child) {
                self.simulate_helper(*child, &mut visited, &dag);
            }
        }
        false
    }
}
