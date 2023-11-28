#![allow(dead_code)]
mod circuit;
mod gate;
mod pin;
mod types;
use crate::gate::*;
use crate::pin::*;
use crate::types::*;
use std::vec;

fn main() {
    let mut _chip = Chip::new();
    let a: u8 = 1;
    let b: u8 = 0;
    let input: Vec<PinValue> = vec![a, b];
    println!("input: {:?}", input);
    let gate_1 = Gate::new(GateType::And, input, PinType::GateInput);
    let gate_2 = Gate::new(GateType::Not, vec![], PinType::GateInput);
    let gate_3 = Gate::new(GateType::And, vec![a], PinType::GateInput);
    let gate_4 = Gate::new(GateType::Not, vec![], PinType::GateInput);
    let gate_5 = Gate::new(GateType::And, vec![b], PinType::GateInput);
    let gate_6 = Gate::new(GateType::Not, vec![], PinType::GateInput);
    let gate_7 = Gate::new(GateType::And, vec![], PinType::GateInput);
    let gate_8 = Gate::new(GateType::Not, vec![], PinType::GateInput);

    let gate_1_id = gate_1.id();
    let gate_2_id = gate_2.id();
    let gate_3_id = gate_3.id();
    let gate_4_id = gate_4.id();
    let gate_5_id = gate_5.id();
    let gate_6_id = gate_6.id();
    let gate_7_id = gate_7.id();
    let gate_8_id = gate_8.id();

    let mut xor = Chip::new();

    xor.add_gate(gate_1);
    xor.add_gate(gate_2);
    xor.add_gate(gate_3);
    xor.add_gate(gate_4);
    xor.add_gate(gate_5);
    xor.add_gate(gate_6);
    xor.add_gate(gate_7);
    xor.add_gate(gate_8);

    xor.connect_gate(gate_1_id, gate_2_id, 0);
    xor.connect_gate(gate_2_id, gate_3_id, 1);
    xor.connect_gate(gate_2_id, gate_5_id, 1);
    xor.connect_gate(gate_3_id, gate_4_id, 0);
    xor.connect_gate(gate_5_id, gate_6_id, 0);
    xor.connect_gate(gate_4_id, gate_7_id, 0);
    xor.connect_gate(gate_6_id, gate_7_id, 1);
    xor.connect_gate(gate_7_id, gate_8_id, 0);

    let dag = xor.gate_dag();
    let output = xor.simulate();
    println!("Output: {:?}", output);
    // println!("connections: {:#?}", xor.connections);
    // println!("pins: {:?}", xor.pin_dag());
    // println!("pins: {:#?}", xor.pins());
}

// #[macroquad::main("lgsim")]
// async fn main() {
// }
