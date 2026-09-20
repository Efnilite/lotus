use crate::app::App;
use crate::ui::{large_icon_button, Icon};
use egui::{Color32, Frame, Margin, Panel, Ui};

const LEFT_TAB_NAME: &str = "left tab";

pub fn render(app: &mut App, ui: &mut Ui) {
    let available_width = ui.ctx().viewport_rect().width();

    Panel::left(LEFT_TAB_NAME)
        .min_size(0.)
        .default_size(available_width * 0.3)
        .max_size(available_width * 0.8)
        .resizable(true)
        .frame(Frame::default().inner_margin(Margin {
            top: 8,
            bottom: 8,
            left: 8,
            right: 8,
        })
            .fill(Color32::from_white_alpha(50)))
        .show(ui, |ui| {
            ui.vertical(|ui| {
                if let Some(response) = large_icon_button(ui, Icon::Folder) {}
            });
        });
}