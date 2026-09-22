use crate::app::App;
use eframe::epaint::Margin;
use egui::{Align, Button, Frame, Layout, Panel, Response, Ui};

const BOTTOM_NAME: &str = "bottom";

pub fn render(app: &mut App, ui: &mut Ui) {
    Panel::bottom(BOTTOM_NAME)
        .frame(Frame::default().inner_margin(Margin {
            top: 8,
            bottom: 8,
            left: 16,
            right: 16,
        }))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    status_button(
                        ui,
                        format!(
                            "{} {}, {} {}",
                            app.settings.locale.line.as_str(),
                            1,
                            app.settings.locale.column.as_str(),
                            4
                        ),
                    );
                    ui.add_space(16.);
                    status_button(ui, format!("Minecraft {}", "26.3"));
                    ui.add_space(16.);
                    status_button(ui, format!("Skript {}", "2.16.2"));
                });
            });
        });
}

fn status_button(ui: &mut Ui, text: String) -> Response {
    ui.scope(|ui| ui.add(Button::new(text).frame(false))).inner
}
