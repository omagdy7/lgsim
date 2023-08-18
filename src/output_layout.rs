use crate::gate::*;
use macroquad::prelude::*;

pub struct OutputLayout {
    outputs: Vec<Connection>,
}

impl OutputLayout {
    pub fn new(outputs: Vec<Connection>) -> Self {
        OutputLayout { outputs }
    }
    pub fn draw(&self) {
        let playground_offset_x: f32 = screen_width() / 20.;
        let playground_offset_y: f32 = screen_height() / 14.;
        let playground_height: f32 = screen_height() - playground_offset_y * 2.0;
        for (cnt, node) in self.outputs.iter().enumerate() {
            draw_circle(
                screen_width() - playground_offset_x,
                playground_height / 3.0 + 25. + (30. * cnt as f32),
                node.radius,
                if node.on { node.color } else { GRAY },
            );
        }
    }
}
