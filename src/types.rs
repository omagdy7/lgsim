use std::collections::HashMap;
use crate::gate::Gate;

pub type PinValue = u8;
pub type Gates = HashMap<usize, Gate>;
pub type Connections = HashMap<usize, Vec<usize>>;
