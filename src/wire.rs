use crate::gate::*;

#[derive(Clone, Debug)]
pub struct Wire {
    src: Connection,
    dst: Connection,
}
