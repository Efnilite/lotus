use crate::app::App;
use eframe::epaint::Margin;
use egui::{CentralPanel, Frame, Ui};

const EDITOR_NAME: &str = "editor";

pub fn render(app: &mut App, ui: &mut Ui) {
    CentralPanel::default()
        .frame(Frame::default().inner_margin(Margin {
            top: 8,
            bottom: 8,
            left: 8,
            right: 8,
        }))
        .show(ui, |ui| {
            if app.editor.opened_files.is_empty() {
                ui.centered_and_justified(|ui| {
                    ui.heading("Create new file Ctrl + N\nSearch Double Shift\nDrop files here to open them");
                });
            }
        });
}