use crate::app::App;
use crate::ui::icon::Icon;
use crate::ui::large_icon_button;
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
                if let Some(response) = large_icon_button(ui, Icon::Folder, Some(app.settings.locale.project.as_str())) {}

                ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
                    if let Some(response) = large_icon_button(ui, Icon::Error, Some(app.settings.locale.problems.as_str())) {}
                    ui.add_space(6.);
                    if let Some(response) = large_icon_button(ui, Icon::Terminal, Some(app.settings.locale.terminal.as_str())) {}
                    ui.add_space(6.);
                    if let Some(response) = large_icon_button(ui, Icon::Analytics, Some(app.settings.locale.analytics.as_str())) {}
                });
            });
        });
}
