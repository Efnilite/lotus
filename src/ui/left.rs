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
                large_icon_button(app, ui, Icon::Folder, app.settings.locale.project.as_str(), Some(app.settings.keybinds.project));

                ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
                    large_icon_button(app, ui, Icon::Analytics, app.settings.locale.analytics.as_str(), Some(app.settings.keybinds.analytics));
                    ui.add_space(6.);
                    large_icon_button(app, ui, Icon::Terminal, app.settings.locale.terminal.as_str(), Some(app.settings.keybinds.terminal));
                    ui.add_space(6.);
                    large_icon_button(app, ui, Icon::Error, app.settings.locale.problems.as_str(), Some(app.settings.keybinds.problems));
                });
            });
        });
}
