use crate::app::App;
use crate::ui::{large_icon_button, Icon};
use eframe::emath::Align;
use eframe::epaint::Margin;
use egui::{Frame, Layout, Panel, Ui};

const TAB_NAME: &str = "tab";

pub fn render(app: &mut App, ui: &mut Ui) {
    Panel::left(TAB_NAME)
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
                if let Some(response) = large_icon_button(ui, Icon::Folder) {

                }
    
                ui.add_space(4.);

                ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
                    if let Some(response) = large_icon_button(ui, Icon::Error) {

                    }
                    ui.add_space(4.);
                    if let Some(response) = large_icon_button(ui, Icon::Terminal) {

                    }
                    ui.add_space(4.);
                    if let Some(response) = large_icon_button(ui, Icon::Analytics) {

                    }
                });
            });
        });
}