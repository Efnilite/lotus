use crate::app::App;
use crate::ui::icon::Icon;
use eframe::emath::{Rect, Vec2};
use eframe::epaint::Color32;
use egui::{Image, KeyboardShortcut, Response, Sense, Ui};
use egui::Event::Text;
use crate::settings::keybinds::Formattable;

mod bottom;
mod editor;
mod icon;
mod left;
mod left_tab;
mod search;
mod settings;
mod top;
mod bottom_tab;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ActiveWindow {
    Settings,
    Search,
}

pub fn render(app: &mut App, ui: &mut Ui) {
    ui.add_enabled_ui(app.active_window.is_none(), |ui| {
        top::render(app, ui);
        bottom::render(app, ui);
        left::render(app, ui);
        bottom_tab::render(app, ui);
        left_tab::render(app, ui);
        // editor::render(app, ui);
    });

    if let Some(window) = &app.active_window {
        match window {
            ActiveWindow::Settings => settings::render(app, ui),
            ActiveWindow::Search => {}
        }
    }
}

const BUTTON_NO_ICON_OFFSET: f32 = 20.;
const MENU_ICON_SIZE: f32 = 15.;
const LARGE_ICON_SIZE: f32 = 20.;

fn menu_icon_button(app: &App,ui: &mut Ui, icon: Icon, tooltip: &str, shortcut: Option<KeyboardShortcut>) -> Response {
    render_icon_button(app, ui, icon, MENU_ICON_SIZE, tooltip, shortcut)
}

fn large_icon_button(app: &App,ui: &mut Ui, icon: Icon, tooltip: &str, shortcut: Option<KeyboardShortcut>) -> Response {
    render_icon_button(app, ui, icon, LARGE_ICON_SIZE, tooltip, shortcut)
}

fn render_icon_button(app: &App, ui: &mut Ui, icon: Icon, button_size: f32, tooltip: &str, shortcut: Option<KeyboardShortcut>) -> Response {
    ui.style_mut().spacing.item_spacing = Vec2::splat(5.0);

    let padding = 4.0;
    let total_side = button_size + (padding * 2.0);
    let size = Vec2::splat(total_side);

    let (rect, response) = ui.allocate_exact_size(size, Sense::click());

    if response.hovered() || response.has_focus() {
        let circle_color = Color32::from_white_alpha(35);
        ui.painter().rect_filled(rect, 4, circle_color);
    }
    if response.is_pointer_button_down_on() {
        let circle_color = Color32::from_white_alpha(38);
        ui.painter().rect_filled(rect, 4, circle_color);
    }

    let image = Image::new(icon.source()).fit_to_exact_size(Vec2::splat(button_size));
    let icon_rect = Rect::from_center_size(rect.center(), Vec2::splat(button_size));
    image.paint_at(ui, icon_rect);

    if let Some(shortcut) = shortcut {
        response.on_hover_ui(|ui| {
            ui.horizontal(|ui| {
                ui.label(tooltip);
                ui.add_space(4.0);
                ui.weak(shortcut.to_formatted_string(app));
            });
        })
    } else {
        response.on_hover_text(tooltip)
    }
}
