use crate::gate::*;
use crate::pin::*;
use std::collections::HashMap;

pub type PinValue = u8;
pub type Pins = Vec<Pin>;
pub type Gates = Vec<Gate>;
pub type Chips = Vec<Chip>;
pub type Connections = HashMap<Pin, Vec<Pin>>;
