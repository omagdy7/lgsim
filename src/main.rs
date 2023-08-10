use macroquad::prelude::*;
mod gate;
use gate::*;

#[macroquad::main("egui with macroquad")]
async fn main() {
    let mut gate = Gate::new(
        Rec::new(500., 500., 120., 80., RED),
        vec![Connection::new(10., GOLD), Connection::new(10., GOLD)],
        Connection::new(10., GOLD),
    );

    loop {
        clear_background(color_u8!(27, 27, 27, 255));

        // Process keys, mouse etc.

        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("egui ❤ macroquad").show(egui_ctx, |ui| {
                ui.label("Test");
            });
        });

        // Draw things before egui

        // let gate2 = Gate::new(
        //     Rec::new(700., 700., 120., 80., RED),
        //     vec![Connection::new(10., GOLD), Connection::new(10., GOLD)],
        // );
        gate.draw();
        // gate2.draw();
        egui_macroquad::draw();

        // Draw things after egui

        next_frame().await;
    }
}
