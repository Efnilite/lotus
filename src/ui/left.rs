use crate::app::App;
use crate::ui::{large_icon_button, Icon};
use egui::{Align, Frame, Layout, Margin, Panel, Ui};

const LEFT_NAME: &str = "left";

pub fn render(app: &mut App, ui: &mut Ui) {
    Panel::left(LEFT_NAME)
        .resizable(false)
        .min_size(0.)
        .frame(Frame::default().inner_margin(Margin {
            top: 8,
            bottom: 8,
            left: 8,
            right: 8,
        }))
        .show(ui, |ui| {
            ui.vertical(|ui| {
                if let Some(response) = large_icon_button(ui, Icon::Folder) {}

                ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
                    if let Some(response) = large_icon_button(ui, Icon::Error) {}
                    ui.add_space(8.);
                    if let Some(response) = large_icon_button(ui, Icon::Terminal) {}
                    ui.add_space(8.);
                    if let Some(response) = large_icon_button(ui, Icon::Analytics) {}
                });
            });
        });
}
