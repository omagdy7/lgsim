#![allow(dead_code)]
mod circuit;
mod gate;
mod pin;
mod types;
use crate::gate::*;
use crate::types::*;
use std::vec;

fn main() {
    {
        let a: u8 = 1;
        let b: u8 = 1;
        let input: Vec<PinValue> = vec![a, b];
        println!("input: {:?}", input);
        let gate_1 = Gate::new(GateType::And, vec![a, a]);
        let gate_2 = Gate::new(GateType::Not, vec![]);
        let gate_1_id = gate_1.id();
        let gate_2_id = gate_2.id();

        let mut nand_1 = Gate::new(GateType::Chip, vec![]);

        nand_1.add_gate(gate_1);
        nand_1.add_gate(gate_2);
        nand_1.connect_gate(gate_1_id, gate_2_id);

        let nand_1_id = nand_1.id();
        let mut double_nand = Gate::new(GateType::Chip, vec![]);

        let gate_3 = Gate::new(GateType::Not, vec![]);
        let gate_3_id = gate_3.id();
        double_nand.add_gate(nand_1);
        double_nand.add_gate(gate_3);

        double_nand.connect_gate(nand_1_id, gate_3_id);

        println!("chip: {:#?}", double_nand);
    }
}
