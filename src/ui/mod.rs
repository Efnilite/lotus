use crate::app::App;
use eframe::emath::{Rect, Vec2};
use eframe::epaint::Color32;
use egui::{include_image, Image, ImageSource, Response, Sense, Ui};

mod search;
mod settings;
mod status;
mod tab;
mod theme;
mod top;

pub fn render(ui: &mut Ui, app: &mut App) {
    ui.add_enabled_ui(app.active_window.is_none(), |ui| {
        top::render(app, ui);
        status::render(app, ui);
        tab::render(app, ui);
    });

    if let Some(window) = &app.active_window {
        match window {
            ActiveWindow::Settings => settings::render(app, ui),
            ActiveWindow::Search => {}
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ActiveWindow {
    Settings,
    Search,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Icon {
    Empty,
    Zip,
    Close,
    Minimize,
    Add,
    Keep,
    Folder,
    Settings,
    Lotus,
    Search,
    Tag,
    Error,
    Warning,
    Terminal,
    Analytics
}

impl Icon {
    pub fn source(self) -> ImageSource<'static> {
        match self {
            Icon::Zip => include_image!("../../assets/icons/zip.svg"),
            Icon::Close => include_image!("../../assets/icons/close.svg"),
            Icon::Minimize => include_image!("../../assets/icons/minimize.svg"),
            Icon::Add => include_image!("../../assets/icons/add.svg"),
            Icon::Keep => include_image!("../../assets/icons/keep.svg"),
            Icon::Folder => include_image!("../../assets/icons/folder.svg"),
            Icon::Settings => include_image!("../../assets/icons/settings.svg"),
            Icon::Lotus => include_image!("../../assets/icons/lotus.svg"),
            Icon::Search => include_image!("../../assets/icons/search.svg"),
            Icon::Tag => include_image!("../../assets/icons/tag.svg"),
            Icon::Empty => include_image!("../../assets/icons/empty.svg"),
            Icon::Error => include_image!("../../assets/icons/error.svg"),
            Icon::Warning => include_image!("../../assets/icons/warning.svg"),
            Icon::Terminal => include_image!("../../assets/icons/terminal.svg"),
            Icon::Analytics => include_image!("../../assets/icons/analytics.svg"),
        }
    }
}

const BUTTON_NO_ICON_OFFSET: f32 = 20.;
const MENU_ICON_SIZE: f32 = 15.;
const LARGE_ICON_SIZE: f32 = 20.;

fn menu_icon_button(ui: &mut Ui, icon: Icon) -> Option<Response> {
    render_icon_button(ui, icon, MENU_ICON_SIZE)
}

fn large_icon_button(ui: &mut Ui, icon: Icon) -> Option<Response> {
    render_icon_button(ui, icon, LARGE_ICON_SIZE)
}

fn render_icon_button(ui: &mut Ui, icon: Icon, button_size: f32) -> Option<Response> {
    let size = Vec2::splat(button_size + 8.0);
    let (rect, response) = ui.allocate_exact_size(size, Sense::click());

    if response.hovered() || response.has_focus() {
        let circle_color = Color32::from_rgba_unmultiplied(255, 255, 255, 35);
        ui.painter().rect_filled(rect, 4, circle_color);
    }
    if response.is_pointer_button_down_on() {
        let circle_color = Color32::from_rgba_unmultiplied(255, 255, 255, 38);
        ui.painter().rect_filled(rect, 4, circle_color);
    }

    let image = Image::new(icon.source()).fit_to_exact_size(Vec2::splat(button_size));
    let icon_rect = Rect::from_center_size(rect.center(), Vec2::splat(button_size));
    image.paint_at(ui, icon_rect);

    Some(response)
}
