use crate::app::App;
use eframe::{run_native, NativeOptions};
use egui::ViewportBuilder;

mod app;
mod ui;
mod settings;
mod editor;
mod locale;

fn main() -> eframe::Result {
    run_native(
        "Lotus",
        NativeOptions {
            viewport: ViewportBuilder::default()
                .with_decorations(false)
                .with_resizable(true)
                .with_maximized(true),
            ..Default::default()
        },
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}
