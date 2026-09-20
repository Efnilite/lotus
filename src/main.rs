use crate::app::App;

mod app;
mod ui;
mod settings;
mod editor;

fn main() -> eframe::Result {
    eframe::run_native(
        "Lotus",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_decorations(false)
                .with_resizable(true)
                .with_maximized(true),
            ..Default::default()
        },
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}
