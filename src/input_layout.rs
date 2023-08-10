use crate::gate::*;
use macroquad::prelude::*;

pub struct InputLayout {
    inputs: Vec<Connection>,
}

impl InputLayout {
    pub fn new(inputs: Vec<Connection>) -> Self {
        InputLayout { inputs }
    }
    pub fn draw(&self) {
        let playground_offset_x: f32 = screen_width() / 20.;
        let playground_offset_y: f32 = screen_height() / 14.;
        let playground_height: f32 = screen_height() - playground_offset_y * 2.0;
        for (cnt, node) in self.inputs.iter().enumerate() {
            draw_circle(
                playground_offset_x,
                playground_height / 3.0 + 25. + (30. * cnt as f32),
                node.radius,
                if node.on { node.color } else { GRAY },
            );
        }
    }
}
