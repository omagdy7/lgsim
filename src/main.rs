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
    let a: u8 = 0;
    let b: u8 = 1;
    let input: Vec<PinValue> = vec![a, b];
    println!("input: {:?}", input);
    let mut gate_1 = Gate::new(GateType::And, input, PinType::ChipInput);
    let mut gate_2 = Gate::new(GateType::Not, vec![], PinType::GateInput);
    let mut gate_3 = Gate::new(GateType::And, vec![a], PinType::GateInput);
    let mut gate_4 = Gate::new(GateType::Not, vec![], PinType::GateInput);
    let mut gate_5 = Gate::new(GateType::And, vec![b], PinType::GateInput);
    let mut gate_6 = Gate::new(GateType::Not, vec![], PinType::GateInput);
    let mut gate_7 = Gate::new(GateType::And, vec![], PinType::GateInput);
    let mut gate_8 = Gate::new(GateType::Not, vec![], PinType::GateInput);

    let mut xor = Chip::new();

    xor.connect_gate(&mut gate_1, &mut gate_2, 0);
    xor.connect_gate(&mut gate_2, &mut gate_3, 1);
    xor.connect_gate(&mut gate_2, &mut gate_5, 1);
    xor.connect_gate(&mut gate_3, &mut gate_4, 0);
    xor.connect_gate(&mut gate_5, &mut gate_6, 0);
    xor.connect_gate(&mut gate_4, &mut gate_7, 0);
    xor.connect_gate(&mut gate_6, &mut gate_7, 1);
    xor.connect_gate(&mut gate_7, &mut gate_8, 0);

    let dag = xor.gate_dag();
    // println!("pins: {:#?}", xor.pins());
    println!("dag: {:#?}", dag);
    // println!("dag: {:#?}", xor.gates);
    // xor.simulate();
    // println!(
    //     "Output: {:?}\n Gate = {:?}",
    //     &mut gate_8.evaluate(),
    //     gate_8.clone(),
    // );
}

// #[macroquad::main("lgsim")]
// async fn main() {
// }
