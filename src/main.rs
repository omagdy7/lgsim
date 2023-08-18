use macroquad::prelude::*;
mod gate;
mod input_layout;
mod output_layout;
mod wire;
use gate::*;
use input_layout::*;
use output_layout::*;
use wire::*;

#[macroquad::main("lgsim")]
async fn main() {
    let mut gate = Gate::new(
        Rec::new(500., 500., 120., 80., RED),
        vec![Connection::new(10., (0.0, 0.0), GREEN, false); 2],
        "GATE",
        Connection::new(10., (0.0, 0.0), GREEN, false),
    );

    let input_layout = InputLayout::new(vec![Connection::new(10., (0.0, 0.0), GREEN, false); 16]);
    let output_layout = OutputLayout::new(vec![Connection::new(10., (0.0, 0.0), GREEN, false); 16]);

    loop {
        let playground_offset_x: f32 = screen_width() / 20.;
        let playground_offset_y: f32 = screen_height() / 14.;
        let playground_width: f32 = screen_width() - playground_offset_x * 2.0;
        let playground_height: f32 = screen_height() - playground_offset_y * 2.0;
        clear_background(color_u8!(27, 27, 27, 255));

        // Process keys, mouse etc.

        draw_rectangle_lines(
            playground_offset_x,
            playground_offset_y,
            playground_width,
            playground_height,
            6.,
            WHITE,
        );

        // Draw things before egui
        input_layout.draw();
        output_layout.draw();
        gate.draw();

        // egui_macroquad::draw();

        // Draw things after egui

        next_frame().await;
    }
}
