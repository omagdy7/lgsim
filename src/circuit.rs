use crate::{gate::Chip, types::*};

#[derive(Debug, Clone)]
struct Circuit {
    chips: Chips,
}

impl Circuit {
    fn new() -> Circuit {
        Circuit { chips: Vec::new() }
    }

    fn add_chip(&mut self, chip: Chip) -> usize {
        self.chips.push(chip);
        0
    }

    fn connect_chip(&mut self, from: usize, to: usize) {
        todo!()
    }

    fn simulate(&mut self) {
        todo!();
    }
}
