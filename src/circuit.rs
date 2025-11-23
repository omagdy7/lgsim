use crate::{gate::*, pin::*, types::*};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Chip {
    pub id: usize,
    pub gates: Gates,
    pub connections: Connections,
    pub pins: HashMap<usize, Pin>,
    pub input: Vec<usize>,
    pub output: Vec<usize>,
}

impl Chip {
    pub fn new(id: usize) -> Chip {
        Chip {
            id,
            gates: Gates::new(),
            connections: HashMap::new(),
            pins: HashMap::new(),
            input: vec![],
            output: vec![],
        }
    }

    pub fn add_gate(&mut self, gate: Gate) -> usize {
        let id = gate.id();
        self.gates.insert(id, gate);
        id
    }

    pub fn add_shell_pin(&mut self, kind: PinType) -> usize {
        let pin = Pin::new(kind, self.id, 0);
        self.pins.insert(pin.id, pin);
        if kind == PinType::ChipInput {
            self.input.push(pin.id);
        } else {
            self.output.push(pin.id);
        }
        pin.id
    }

    pub fn connect_pins(&mut self, from_pin: usize, to_pin: usize) {
        for connections in self.connections.values_mut() {
            connections.retain(|&x| x != to_pin);
        }
        self.connections
            .entry(from_pin)
            .or_insert(vec![])
            .push(to_pin);
    }

    pub fn set_pin(&mut self, id: &usize, val: Option<PinValue>) {
        if let Some(p) = self.pins.get_mut(id) {
            p.val = val;
        }
    }

    pub fn simulate(&mut self) -> bool {
        for &in_pin in &self.input {
            let val = self.pins.get(&in_pin).unwrap().val;
            if let Some(targets) = self.connections.get(&in_pin) {
                for target in targets {
                    for gate in self.gates.values_mut() {
                        if gate.input().contains(target) {
                            gate.set_pin(target, val);
                        }
                    }
                }
            }
        }

        let loops = self.gates.len() + 2;
        for _ in 0..loops {
            let ids: Vec<usize> = self.gates.keys().cloned().collect();
            for gid in ids {
                self.gates.get_mut(&gid).unwrap().evaluate();
                self.propagate_internal(gid);
            }
        }
        true
    }

    fn propagate_internal(&mut self, gate_id: usize) {
        let outputs = self.gates.get(&gate_id).unwrap().output().to_vec();
        if outputs.is_empty() {
            return;
        }
        let val = self
            .gates
            .get(&gate_id)
            .unwrap()
            .pins()
            .get(&outputs[0])
            .unwrap()
            .val;

        for out_pin in outputs {
            if let Some(targets) = self.connections.get(&out_pin).cloned() {
                for target_pin in targets {
                    let mut found_gate = false;
                    for gate in self.gates.values_mut() {
                        if gate.input().contains(&target_pin) {
                            gate.set_pin(&target_pin, val);
                            found_gate = true;
                        }
                    }
                    if !found_gate {
                        if self.output.contains(&target_pin) {
                            self.set_pin(&target_pin, val);
                        }
                    }
                }
            }
        }
    }

    pub fn deep_copy(&self) -> Chip {
        // FIX: Use global counter, not a local static!
        let new_chip_id = crate::pin::next_uuid();
        let mut new_chip = Chip::new(new_chip_id);
        let mut id_map: HashMap<usize, usize> = HashMap::new();

        for &old_id in &self.input {
            let new_id = new_chip.add_shell_pin(PinType::ChipInput);
            id_map.insert(old_id, new_id);
        }
        for &old_id in &self.output {
            let new_id = new_chip.add_shell_pin(PinType::ChipOutput);
            id_map.insert(old_id, new_id);
        }

        for gate in self.gates.values() {
            let new_gate = gate.clone_with_new_ids(&mut id_map);
            new_chip.add_gate(new_gate);
        }

        for (src, dests) in &self.connections {
            if let Some(&new_src) = id_map.get(src) {
                for dst in dests {
                    if let Some(&new_dest) = id_map.get(dst) {
                        new_chip.connect_pins(new_src, new_dest);
                    }
                }
            }
        }
        new_chip
    }
}
