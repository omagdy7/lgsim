fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };
    eframe::run_native(
        "Rust Logic Simulator",
        options,
        Box::new(|_cc| Box::new(lgsim::gate_ui::LogicApp::new())),
    )
}

