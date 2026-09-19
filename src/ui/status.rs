use egui::{Align, Button, Layout, Panel, Response, Ui};

const STATUS_NAME: &str = "status";

pub fn render(ui: &mut Ui) {
    Panel::bottom(STATUS_NAME).show(ui, |ui| {
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            render_button(ui, format!("Line {}, Column {}", 1, 4));
            ui.add_space(16.);
            render_button(ui, format!("Minecraft {}", "26.3"));
            ui.add_space(16.);
            render_button(ui, format!("Skript {}", "2.16.2"));
        });
    });
}

fn render_button(ui: &mut Ui, text: String) -> Response {
    ui.scope(|ui| {
        ui.style_mut().spacing.button_padding.x += 16.0;
        ui.add(Button::new(text).frame(false))
    })
    .inner
}
