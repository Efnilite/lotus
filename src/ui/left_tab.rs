use crate::app::App;
use crate::ui::icon::Icon;
use crate::ui::large_icon_button;
use egui::{Color32, Frame, Margin, Panel, Ui};

const LEFT_TAB_NAME: &str = "left tab";

pub fn render(app: &mut App, ui: &mut Ui) {
    Panel::left(LEFT_TAB_NAME)
        .min_size(0.)
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
                let _response = large_icon_button(app, ui, Icon::Folder, app.settings.locale.project.as_str(), None);
            });
        });
}