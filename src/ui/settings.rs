use crate::app::App;
use egui::{Ui, Window};

pub fn render(app: &mut App, ui: &mut Ui) {
    Window::new("settings")
        .collapsible(false)
        .movable(true)
        .resizable(true)
        .title_bar(false)
        .show(ui.ctx(), |ui| {
            let mut dm = false;
            ui.checkbox(&mut dm, "Dark Mode");

            ui.add_space(12.0);
            ui.separator();
            ui.add_space(8.0);

            ui.horizontal(|ui| {
                if ui.button("Close").clicked() {
                    app.view_state.active_window = None;
                }
            });
        });
}
