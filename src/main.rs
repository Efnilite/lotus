use crate::app::App;

mod app;
mod ui;
mod settings;

fn main() -> eframe::Result {
    eframe::run_native(
        "Lotus",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_decorations(false),
            ..Default::default()
        },
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}
