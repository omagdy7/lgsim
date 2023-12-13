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
    // TODO: Find a way to have unique ids without having this unsafe block
    pub fn new(gate_type: GateType, input: Vec<PinValue>) -> Self {
        static mut ID: usize = 0;
        unsafe {
            ID += 1;
            match gate_type {
                GateType::And => Gate::And(AndGate::new(input, ID)),
                GateType::Not => Gate::Not(NotGate::new(input, ID)),
                GateType::Chip => Gate::Chip(Chip::new(ID)),
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
        let id = self.id().clone();
        match self {
            Gate::And(gate) => gate.add_input(val, connections),
            Gate::Not(gate) => gate.add_input(val, connections),
            Gate::Chip(chip) => chip.add_input(Pin::new(PinType::ChipInput, id, val)),
        }
    }

    fn pins(&self) -> &HashMap<usize, Pin> {
        match self {
            Gate::And(gate) => &gate.pins,
            Gate::Not(gate) => &gate.pins,
            Gate::Chip(chip) => &chip.pins,
        }
    }

    fn pins_mut(&mut self) -> &mut HashMap<usize, Pin> {
        match self {
            Gate::And(gate) => &mut gate.pins,
            Gate::Not(gate) => &mut gate.pins,
            Gate::Chip(chip) => &mut chip.pins,
        }
    }

    fn set_pin(&mut self, id: &usize, val: Option<PinValue>) {
        match self {
            Gate::And(gate) => gate.set_pin(id, val),
            Gate::Not(gate) => gate.set_pin(id, val),
            Gate::Chip(chip) => chip.set_pin(id, val),
        }
    }

    fn input(&self) -> &[usize] {
        match self {
            Gate::And(gate) => &gate.input,
            Gate::Not(gate) => &gate.input,
            Gate::Chip(chip) => &chip.input,
        }
    }

    pub fn add_gate(&mut self, gate: Gate) -> usize {
        match self {
            Gate::Chip(chip) => chip.add_gate(gate),
            _ => {
                unreachable!("You shouldn't be calling this function");
            }
        }
    }

    pub fn connect_gate(&mut self, from: usize, to: usize) {
        match self {
            Gate::Chip(chip) => chip.connect_gate(from, to),
            _ => {
                unreachable!("You shouldn't be calling this function");
            }
        }
    }

    pub fn pin(&self, id: usize) -> &Pin {
        match self {
            Gate::And(gate) => gate.pin(id),
            Gate::Not(gate) => gate.pin(id),
            Gate::Chip(chip) => chip.pin(id),
        }
    }

    fn pin_mut(&mut self, id: usize) -> &Pin {
        match self {
            Gate::And(gate) => gate.pin_mut(id),
            Gate::Not(gate) => gate.pin_mut(id),
            Gate::Chip(chip) => chip.pin_mut(id),
        }
    }

    fn output(&self) -> &[usize] {
        match self {
            Gate::And(gate) => &gate.output,
            Gate::Not(gate) => &gate.output,
            Gate::Chip(chip) => &chip.output,
        }
    }

    pub fn id(&self) -> usize {
        match self {
            Gate::And(gate) => gate.id,
            Gate::Not(gate) => gate.id,
            Gate::Chip(chip) => chip.id,
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
    id: usize,                 // unique id for every gate
    pins: HashMap<usize, Pin>, // contains all the input and output pins of that gate
    input: Vec<usize>,         // contains only the ids of input pins
    output: Vec<usize>,        // contains only th ids of output pins
}

impl AndGate {
    fn new(input: Vec<PinValue>, id: usize) -> Self {
        /* if we are here it means the gate input doesn't depend on input
        known when running the circuit aka these inputs will be evaluated at runtime */
        if input.is_empty() {
            let pin_out = Pin::new(PinType::GateOutput, id, 42);
            let mut pins = HashMap::new();
            pins.insert(pin_out.id, pin_out);
            Self {
                id,
                pins,
                input: vec![],
                output: vec![pin_out.id],
            }
        }
        // only one input is known before running the circuit
        else if input.len() == 1 {
            let pin_1 = Pin::new(PinType::GateInput, id, input[0]);
            let pin_out = Pin::new(PinType::GateOutput, id, 42);
            let mut pins = HashMap::new();
            pins.insert(pin_1.id, pin_1);
            pins.insert(pin_out.id, pin_out);
            Self {
                id,
                pins,
                input: vec![pin_1.id],
                output: vec![pin_out.id],
            }
        }
        // The two inputs are known before evaluating the circuit
        else {
            let pin_1 = Pin::new(PinType::GateInput, id, input[0]);
            let pin_2 = Pin::new(PinType::GateInput, id, input[1]);
            let pin_out = Pin::new(PinType::GateOutput, id, 42);
            let mut pins = HashMap::new();
            pins.insert(pin_1.id, pin_1);
            pins.insert(pin_2.id, pin_2);
            pins.insert(pin_out.id, pin_out);
            Self {
                id,
                pins,
                input: vec![pin_1.id, pin_2.id],
                output: vec![pin_out.id],
            }
        }
    }

    pub fn pin(&self, id: usize) -> &Pin {
        // TODO: Should return a Result and propagate the error up the stack
        self.pins.get(&id).expect("pin should exist")
    }

    fn pin_mut(&mut self, id: usize) -> &mut Pin {
        // TODO: Should return a Result and propagate the error up the stack
        self.pins.get_mut(&id).expect("pin should exist")
    }

    fn set_pin(&mut self, id: &usize, val: Option<PinValue>) {
        self.pin_mut(*id).val = val;
    }

    // Adds an input pin and returns the id of that pin
    fn add_input(&mut self, val: PinValue, connections: &mut Connections) -> usize {
        let new_pin = Pin::new(PinType::GateInput, self.id, val);
        self.pins.insert(new_pin.id, new_pin);
        self.input.push(new_pin.id);
        connections
            .entry(self.output[0])
            .and_modify(|val| val.push(new_pin.id))
            .or_insert_with(|| vec![new_pin.id]);
        new_pin.id
    }

    // Evaluating the And gate by literally anding it's two inputs
    fn evaluate(&mut self) -> bool {
        let res =
            (self.pin(self.input[0]).val.unwrap() & self.pin(self.input[1]).val.unwrap()) == 1;
        // println!(
        //     "input: {:?}",
        //     self.input
        //         .iter()
        //         .map(|&x| (self.pin(x).val, self.pin(x).id))
        //         .collect::<Vec<(Option<PinValue>, usize)>>()
        // );
        // println!("Evaluating gate_{} with res = {}", self.id, res);
        self.pin_mut(self.output[0]).val = Some(res as u8);
        self.pin_mut(self.output[0]).kind = PinType::GateOutput;
        res
    }
}

#[derive(Debug, Clone)]
pub struct NotGate {
    id: usize,
    pins: HashMap<usize, Pin>,
    input: Vec<usize>,
    output: Vec<usize>,
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
                output: vec![pin_out.id],
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
                output: vec![pin_out.id],
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
            .entry(self.output[0])
            .and_modify(|val| val.push(new_pin.id))
            .or_insert_with(|| vec![new_pin.id]);
        new_pin.id
    }

    fn evaluate(&mut self) -> bool {
        let res = !(self.pin(self.input[0]).val.unwrap() == 1);
        // println!(
        //     "input: {:?}",
        //     self.input
        //         .iter()
        //         .map(|&x| (self.pin(x).val, self.pin(x).id))
        //         .collect::<Vec<(Option<PinValue>, usize)>>()
        // );
        // println!("Evaluating gate_{} with res = {}", self.id, res);
        self.pin_mut(self.output[0]).val = Some(res as u8);
        self.pin_mut(self.output[0]).kind = PinType::GateOutput;
        res
    }
}

/*
*  A Chip is collection of smaller gates connected together it's essentially a complex gate
*  A Chip very much like a gate have some input pins and output pins, but they also have
*  connections map which defines the edges between gates and how they are connected
*/
#[derive(Debug, Clone, PartialEq)]
pub struct Chip {
    id: usize,
    pub gates: Gates,
    pub connections: Connections,
    pins: HashMap<usize, Pin>,
    input: Vec<usize>,
    output: Vec<usize>,
}

impl Chip {
    pub fn new(id: usize) -> Chip {
        Chip {
            id,
            gates: Gates::new(),         // Gates in the current chip
            connections: HashMap::new(), // Essentially represents how the pins are connected
            pins: HashMap::new(),        // Pins in the current chip(Not including gates pins)
            input: vec![],               // Chip input pins
            output: vec![],              // Chip output pins
        }
    }

    fn set_pin(&mut self, id: &usize, val: Option<PinValue>) {
        /*
        * TODO: This should return a Result and deals with faliure more properly instead of
                crashing the program
        */
        self.pins.get_mut(id).expect("Pin should exsist").val = val;
    }

    pub fn add_input(&mut self, pin: Pin) -> usize {
        self.pins.insert(pin.id, pin);
        self.input.push(pin.id);
        pin.id
    }

    pub fn add_output(&mut self, pin: Pin) {
        self.pins.insert(pin.id, pin);
        self.output.push(pin.id);
    }

    fn set_output(&mut self) {
        let g_pins = self.gate_dag();
        let gates = g_pins.iter().map(|x| x.0).collect::<Vec<_>>();
        for gate_id in gates {
            let mut found = false;
            for (_, children) in &g_pins {
                if children.contains(gate_id) {
                    found = true;
                    break;
                }
            }
            if !found {
                self.output.push(*gate_id);
            }
        }
    }

    pub fn add_gate(&mut self, gate: Gate) -> usize {
        /*
         * The output of a gate depends on its inputs
         * so we have to add all of input pins as a dependency
         * to the output pin in the conncections graph
         */
        for pin_in in gate.input() {
            for output in gate.output() {
                self.connections
                    .entry(*output)
                    .and_modify(|val| val.push(*pin_in))
                    .or_insert_with(|| vec![*pin_in]);
            }
        }
        let id = gate.id();
        self.gates.insert(id, gate);
        id
    }

    /*
     *  This function takes two gate ids and connects them
     */
    pub fn connect_gate(&mut self, from: usize, to: usize) {
        let from_pins = self.gates.get(&from).unwrap().clone();
        let from_pins = from_pins.output();
        let to_gate = self.gates.get_mut(&to).unwrap();
        let mut _id = 42;
        if to_gate.input().len() < 2 {
            _id = to_gate.add_input(0, &mut self.connections)
        }

        // the inputs of some gate depends on the output of some other gate
        for output in from_pins {
            self.connections
                .entry(to_gate.input()[to_gate.input().len() - 1])
                .and_modify(|val| val.push(*output))
                .or_insert_with(|| vec![*output]);
        }
    }

    // Returns a HashMap containing all pins in the current chip
    pub fn gates_pins(&self) -> HashMap<usize, &Pin> {
        let mut pins = HashMap::new();
        for (_, gate) in &self.gates {
            for (id, pin) in gate.pins() {
                pins.insert(*id, pin);
            }
        }
        pins
    }

    fn pin(&self, id: usize) -> &Pin {
        // TODO: Do peoper error handling and propagate the error up the stack instead of crashing
        self.pins.get(&id).expect("Pin should exist")
    }

    pub fn pin_mut(&mut self, id: usize) -> &mut Pin {
        // TODO: Do peoper error handling and propagate the error up the stack instead of crashing
        self.pins.get_mut(&id).expect("Pin should exist")
    }

    // Constructs the gate dag(Directed Acyclic Graph) from the connections map
    pub fn gate_dag(&self) -> HashMap<usize, Vec<usize>> {
        let pins = self.gates_pins();
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
        // println!(
        //     "Propagating: gate_{} with pin_{} to val = {:?}",
        //     gate_id, pin_id, val
        // );
        self.gates.get_mut(&gate_id).unwrap().set_pin(pin_id, val);
    }

    /*
     * The function that does all the magic of evaluating gates outputs recusively
     * by utilizing toplogical sort(kind of) it just traverses the graph in post-order
     * and it makes sure that all the gates output are propagated properly to their connected pins
     */
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

    /*
     * Takes a gate id and propagates its output evaluation to its other conncted pins in the pin
     * dag, this function should only be called when we are sure that the output has already been
     * evaluated which should be the case because we are traversting the graph in post-order
     */
    fn propagate(&mut self, gate_id: usize) {
        // TODO: Optimize this function especially the unecessary calling of self.pins()
        // TODO: Add proper error handling of when the output value hasn't been evaluated yet
        let pin_dag = self.pin_dag().clone();
        let pins_out = self.gates.get(&gate_id).unwrap().output();
        let mut propagated_pins = HashMap::new();
        let pin_out_val = {
            let pins = self.gates_pins();
            pins.get(&pins_out[0]).unwrap().val
        };
        for pin in pins_out {
            propagated_pins.extend(pin_dag.iter().filter(|(_, pins)| pins.contains(&pin)));
        }
        /* set the values of the pins conncted to the output pin of that gate_id to the value of
         *  that output pin
         */
        for (pin_id, _) in propagated_pins {
            let pins = self.gates_pins();
            if let Some(pin) = pins.get(&pin_id).cloned() {
                let gate_id = pin.gate_id;
                self.set_gate_pin(gate_id, &pin_id, pin_out_val);
            }
        }
    }

    /*
     * The function that kickstarts simulate_helper which does all the job of evaluating gates
     * recusively
     */
    pub fn simulate(&mut self) -> bool {
        let dag = self.gate_dag();
        let mut visited = HashSet::new();
        self.set_output();
        println!("output: {:?}", self.output);
        for out in self.output.clone() {
            for child in dag.get(&out).unwrap() {
                if !visited.contains(child) {
                    self.simulate_helper(*child, &mut visited, &dag);
                }
            }
            return self.gates.get_mut(&out).unwrap().evaluate();
        }
        false
    }
}

mod tests {
    use crate::{
        gate::{Gate, GateType},
        types::PinValue,
    };

    fn test_or_helper(a: u8, b: u8) -> bool {
        let input: Vec<PinValue> = vec![a, b];
        println!("input: {:?}", input);
        let gate_1 = Gate::new(GateType::And, vec![a, a]);
        let gate_2 = Gate::new(GateType::Not, vec![]);
        let gate_3 = Gate::new(GateType::And, vec![b, b]);
        let gate_4 = Gate::new(GateType::Not, vec![]);
        let gate_5 = Gate::new(GateType::And, vec![]);
        let gate_6 = Gate::new(GateType::Not, vec![]);

        let gate_1_id = gate_1.id();
        let gate_2_id = gate_2.id();
        let gate_3_id = gate_3.id();
        let gate_4_id = gate_4.id();
        let gate_5_id = gate_5.id();
        let gate_6_id = gate_6.id();

        let mut or = Gate::new(GateType::Chip, vec![]);

        or.add_gate(gate_1);
        or.add_gate(gate_2);
        or.add_gate(gate_3);
        or.add_gate(gate_4);
        or.add_gate(gate_5);
        or.add_gate(gate_6);

        or.connect_gate(gate_1_id, gate_2_id);
        or.connect_gate(gate_2_id, gate_5_id);
        or.connect_gate(gate_3_id, gate_4_id);
        or.connect_gate(gate_4_id, gate_5_id);
        or.connect_gate(gate_5_id, gate_6_id);

        println!("Output: {:?}", or.evaluate());
        or.evaluate()
    }

    fn test_xor_helper(a: u8, b: u8) -> bool {
        let input: Vec<PinValue> = vec![a, b];
        println!("input: {:?}", input);
        let gate_1 = Gate::new(GateType::And, input);
        let gate_2 = Gate::new(GateType::Not, vec![]);
        let gate_3 = Gate::new(GateType::And, vec![a]);
        let gate_4 = Gate::new(GateType::Not, vec![]);
        let gate_5 = Gate::new(GateType::And, vec![b]);
        let gate_6 = Gate::new(GateType::Not, vec![]);
        let gate_7 = Gate::new(GateType::And, vec![]);
        let gate_8 = Gate::new(GateType::Not, vec![]);

        let gate_1_id = gate_1.id();
        let gate_2_id = gate_2.id();
        let gate_3_id = gate_3.id();
        let gate_4_id = gate_4.id();
        let gate_5_id = gate_5.id();
        let gate_6_id = gate_6.id();
        let gate_7_id = gate_7.id();
        let gate_8_id = gate_8.id();

        let mut xor = Gate::new(GateType::Chip, vec![]);

        xor.add_gate(gate_1);
        xor.add_gate(gate_2);
        xor.add_gate(gate_3);
        xor.add_gate(gate_4);
        xor.add_gate(gate_5);
        xor.add_gate(gate_6);
        xor.add_gate(gate_7);
        xor.add_gate(gate_8);

        xor.connect_gate(gate_1_id, gate_2_id);
        xor.connect_gate(gate_2_id, gate_3_id);
        xor.connect_gate(gate_2_id, gate_5_id);
        xor.connect_gate(gate_3_id, gate_4_id);
        xor.connect_gate(gate_5_id, gate_6_id);
        xor.connect_gate(gate_4_id, gate_7_id);
        xor.connect_gate(gate_6_id, gate_7_id);
        xor.connect_gate(gate_7_id, gate_8_id);

        println!("Output: {:?}", xor.evaluate());
        xor.evaluate()
    }

    #[test]
    fn test_xor() {
        assert!(!test_xor_helper(0, 0));
        assert!(test_xor_helper(0, 1));
        assert!(test_xor_helper(1, 0));
        assert!(!test_xor_helper(1, 1));
    }

    #[test]
    fn test_or() {
        assert!(!test_or_helper(0, 0));
        assert!(test_or_helper(0, 1));
        assert!(test_or_helper(1, 0));
        assert!(test_or_helper(1, 1));
    }
}
