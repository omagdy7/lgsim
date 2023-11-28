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
    pins: HashMap<usize, Pin>,
    input: Vec<usize>,
    output: usize,
}

impl AndGate {
    fn new(input: Vec<PinValue>, id: usize, kind: PinType) -> Self {
        if input.is_empty() {
            let pin_out = Pin::new(PinType::GateOutput, id, 42);
            let mut pins = HashMap::new();
            pins.insert(pin_out.id, pin_out);
            Self {
                id,
                pins,
                input: vec![],
                output: pin_out.id,
            }
        } else if input.len() == 1 {
            let pin_1 = Pin::new(kind, id, input[0]);
            let pin_out = Pin::new(PinType::GateOutput, id, 42);
            let mut pins = HashMap::new();
            pins.insert(pin_1.id, pin_1);
            pins.insert(pin_out.id, pin_out);
            Self {
                id,
                pins,
                input: vec![pin_1.id],
                output: pin_out.id,
            }
        } else {
            let pin_1 = Pin::new(kind, id, input[0]);
            let pin_2 = Pin::new(kind, id, input[1]);
            let pin_out = Pin::new(PinType::GateOutput, id, 42);
            let mut pins = HashMap::new();
            pins.insert(pin_1.id, pin_1);
            pins.insert(pin_2.id, pin_2);
            pins.insert(pin_out.id, pin_out);
            Self {
                id,
                pins,
                input: vec![pin_1.id, pin_2.id],
                output: pin_out.id,
            }
        }
    }

    pub fn pin(&self, id: usize) -> &Pin {
        self.pins.get(&id).expect("pin should exist")
    }

    fn pin_mut(&mut self, id: usize) -> &mut Pin {
        self.pins.get_mut(&id).expect("pin should exist")
    }

    fn set_pin(&mut self, id: &usize, val: Option<PinValue>) {
        self.pin_mut(*id).val = val;
    }

    fn add_input(&mut self, val: PinValue, connections: &mut Connections) -> usize {
        let new_pin = Pin::new(PinType::GateInput, self.id, val);
        self.pins.insert(new_pin.id, new_pin);
        self.input.push(new_pin.id);
        connections
            .entry(self.output)
            .and_modify(|val| val.push(new_pin.id))
            .or_insert_with(|| vec![new_pin.id]);
        new_pin.id
    }

    fn evaluate(&mut self) -> bool {
        let res =
            (self.pin(self.input[0]).val.unwrap() & self.pin(self.input[1]).val.unwrap()) == 1;
        println!(
            "input: {:?}",
            self.input
                .iter()
                .map(|&x| (self.pin(x).val, self.pin(x).id))
                .collect::<Vec<(Option<PinValue>, usize)>>()
        );
        println!("Evaluating gate_{} with res = {}", self.id, res);
        self.pin_mut(self.output).val = Some(res as u8);
        self.pin_mut(self.output).kind = PinType::GateOutput;
        res
    }
}

#[derive(Debug, Clone)]
pub struct NotGate {
    id: usize,
    pins: HashMap<usize, Pin>,
    input: Vec<usize>,
    output: usize,
}

impl NotGate {
    fn new(input: Vec<PinValue>, id: usize) -> Self {
        if input.is_empty() {
            let pin_out = Pin::new(PinType::GateOutput, id, 42);
            let mut pins = HashMap::new();
            pins.insert(pin_out.id, pin_out);
            Self {
                id,
                pins,
                input: vec![],
                output: pin_out.id,
            }
        } else {
            let pin_1 = Pin::new(PinType::GateInput, id, input[0]);
            let pin_out = Pin::new(PinType::GateOutput, id, 42);
            let mut pins = HashMap::new();
            pins.insert(pin_1.id, pin_1);
            pins.insert(pin_out.id, pin_out);
            Self {
                id,
                pins,
                input: vec![pin_1.id],
                output: pin_out.id,
            }
        }
    }

    fn pin(&self, id: usize) -> &Pin {
        self.pins.get(&id).expect("pin should exist")
    }

    fn pin_mut(&mut self, id: usize) -> &mut Pin {
        self.pins.get_mut(&id).expect("pin should exist")
    }

    fn set_pin(&mut self, id: &usize, val: Option<PinValue>) {
        self.pin_mut(*id).val = val;
    }

    fn add_input(&mut self, val: PinValue, connections: &mut Connections) -> usize {
        let new_pin = Pin::new(PinType::GateInput, self.id, val);
        self.pins.insert(new_pin.id, new_pin);
        self.input.push(new_pin.id);
        connections
            .entry(self.output)
            .and_modify(|val| val.push(new_pin.id))
            .or_insert_with(|| vec![new_pin.id]);
        new_pin.id
    }

    fn evaluate(&mut self) -> bool {
        let res = !(self.pin(self.input[0]).val.unwrap() == 1);
        println!(
            "input: {:?}",
            self.input
                .iter()
                .map(|&x| (self.pin(x).val, self.pin(x).id))
                .collect::<Vec<(Option<PinValue>, usize)>>()
        );
        println!("Evaluating gate_{} with res = {}", self.id, res);
        self.pin_mut(self.output).val = Some(res as u8);
        self.pin_mut(self.output).kind = PinType::GateOutput;
        res
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Chip {
    pub gates: Gates,
    pub connections: Connections,
    pins: HashMap<usize, Pin>,
    input: Vec<usize>,
    output: Vec<usize>,
}

impl Chip {
    pub fn new() -> Chip {
        Chip {
            gates: Gates::new(),
            connections: HashMap::new(),
            pins: HashMap::new(),
            input: vec![],
            output: vec![],
        }
    }

    pub fn add_input(&mut self, pin: Pin) {
        self.pins.insert(pin.id, pin);
        self.input.push(pin.id);
    }

    pub fn add_output(&mut self, pin: Pin) {
        self.pins.insert(pin.id, pin);
        self.output.push(pin.id);
    }

    pub fn add_gate(&mut self, gate: Gate) -> usize {
        // the output of a gate depends on its inputs
        for pin_in in gate.input() {
            self.connections
                .entry(*gate.output())
                .and_modify(|val| val.push(*pin_in))
                .or_insert_with(|| vec![*pin_in]);
        }
        let id = gate.id();
        self.gates.insert(id, gate);
        id
    }

    pub fn connect_gate(&mut self, from: usize, to: usize, to_pin_idx: usize) {
        let from_pin_id = self.gates.get(&from).unwrap().output().clone();
        let to_gate = self.gates.get_mut(&to).unwrap();
        let mut id = 42;
        if to_gate.input().len() < 2 {
            id = to_gate.add_input(0, &mut self.connections)
        }

        // the inputs of some gate depends on the output of some other gate
        self.connections
            .entry(to_gate.input()[to_pin_idx])
            .and_modify(|val| val.push(from_pin_id))
            .or_insert_with(|| vec![from_pin_id]);
    }

    pub fn pins(&self) -> HashMap<usize, &Pin> {
        let mut pins = HashMap::new();
        for (_, gate) in &self.gates {
            for (id, pin) in gate.pins() {
                pins.insert(*id, pin);
            }
        }
        pins
    }

    pub fn pins_mut(&mut self) -> HashMap<usize, &mut Pin> {
        let mut pins = HashMap::new();
        for (_, gate) in &mut self.gates {
            for (id, pin) in gate.pins_mut() {
                pins.entry(*id).or_insert_with(|| pin);
            }
        }
        pins
    }

    pub fn gate_dag(&self) -> HashMap<usize, Vec<usize>> {
        let pins = self.pins();
        let mut dag = HashMap::new();
        for connection in &self.connections {
            for pin in connection.1 {
                match pins.get(connection.0).unwrap().kind {
                    PinType::GateInput => {
                        dag.entry(pins.get(connection.0).unwrap().gate_id)
                            .and_modify(|val: &mut Vec<usize>| {
                                val.push(pins.get(pin).unwrap().gate_id)
                            })
                            .or_insert_with(|| vec![pins.get(pin).unwrap().gate_id]);
                    }
                    _ => {}
                }
            }
        }
        dag
    }

    pub fn pin_dag(&self) -> &HashMap<usize, Vec<usize>> {
        &self.connections
    }

    fn set_gate_pin(&mut self, gate_id: usize, pin_id: &usize, val: Option<PinValue>) {
        println!(
            "Propagating: gate_{} with pin_{} to val = {:?}",
            gate_id, pin_id, val
        );
        self.gates.get_mut(&gate_id).unwrap().set_pin(pin_id, val);
    }

    fn simulate_helper(
        &mut self,
        gate_id: usize,
        visited: &mut HashSet<usize>,
        dag: &HashMap<usize, Vec<usize>>,
    ) {
        visited.insert(gate_id);
        if dag.contains_key(&gate_id) {
            for child in dag.get(&gate_id).unwrap() {
                if !visited.contains(child) {
                    self.simulate_helper(*child, visited, dag);
                }
            }
            self.gates.get_mut(&gate_id).unwrap().evaluate();
            self.propagate(gate_id);
        } else {
            self.gates.get_mut(&gate_id).unwrap().evaluate();
            self.propagate(gate_id);
        }
    }

    // TODO: Optimize this function specially the unecessary calling of self.pins()
    fn propagate(&mut self, gate_id: usize) {
        let pin_dag = self.pin_dag().clone();
        let pin_out_id = self.gates.get(&gate_id).unwrap().output().clone();
        let pin_out_val = {
            let pins = self.pins();
            pins.get(&pin_out_id).unwrap().val
        };
        let propagated_pins = pin_dag
            .iter()
            .filter(|(_, pins)| pins.contains(&pin_out_id));
        for (pin_id, _) in propagated_pins {
            let pins = self.pins();
            if let Some(pin) = pins.get(&pin_id).cloned() {
                let gate_id = pin.gate_id;
                self.set_gate_pin(gate_id, pin_id, pin_out_val);
            }
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
        self.gates.get_mut(&8).unwrap().evaluate()
    }
}
