use crate::app::App;
use crate::settings::keybinds::Formattable;
use crate::ui::icon::Icon;
use eframe::emath::{Rect, Vec2};
use eframe::epaint::Color32;
use egui::{Button, Image, KeyboardShortcut, Layout, Response, Sense, Ui};
use emath::Align;

mod bottom;
mod bottom_tab;
mod editor;
mod icon;
mod left;
mod left_tab;
mod search;
mod settings;
mod top;

#[derive(Default)]
pub struct ViewState {
    active_window: Option<ActiveWindow>,
    active_bottom_tab: Option<ActiveBottomTab>,
    active_left_tab: Option<ActiveLeftTab>,

    tree_state: egui_ltreeview::TreeViewState<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActiveWindow {
    Settings,
    Search,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActiveLeftTab {
    Project,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActiveBottomTab {
    Problems,
    Terminal,
    Analytics,
}

pub fn render(app: &mut App, ui: &mut Ui) {
    ui.add_enabled_ui(app.view_state.active_window.is_none(), |ui| {
        top::render(app, ui);
        bottom::render(app, ui);
        left::render(app, ui);
        // bottom_tab::render(app, ui);

        if let Some(window) = &app.view_state.active_bottom_tab {
            match window {
                ActiveBottomTab::Problems => bottom_tab::render(app, ui),
                ActiveBottomTab::Analytics => bottom_tab::render(app, ui),
                ActiveBottomTab::Terminal => bottom_tab::render(app, ui),
            }
        }

        if let Some(window) = &app.view_state.active_left_tab {
            match window {
                ActiveLeftTab::Project => left_tab::render(app, ui),
            }
        }

        // editor::render(app, ui);
    });

    if let Some(window) = &app.view_state.active_window {
        match window {
            ActiveWindow::Settings => settings::render(app, ui),
            ActiveWindow::Search => {}
        }
    }
}

const BUTTON_NO_ICON_OFFSET: f32 = 20.;
const MENU_ICON_SIZE: f32 = 15.;
const PROJECT_ICON_SIZE: f32 = 14.;
const LARGE_ICON_SIZE: f32 = 20.;

fn menu_icon_button(
    app: &App,
    ui: &mut Ui,
    icon: Icon,
    tooltip: &str,
    shortcut: Option<KeyboardShortcut>,
) -> Response {
    render_icon_button(app, ui, icon, MENU_ICON_SIZE, tooltip, shortcut)
}

fn large_icon_button(
    app: &App,
    ui: &mut Ui,
    icon: Icon,
    tooltip: &str,
    shortcut: Option<KeyboardShortcut>,
) -> Response {
    render_icon_button(app, ui, icon, LARGE_ICON_SIZE, tooltip, shortcut)
}

fn render_icon_button(
    app: &App,
    ui: &mut Ui,
    icon: Icon,
    button_size: f32,
    tooltip: &str,
    shortcut: Option<KeyboardShortcut>,
) -> Response {
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

    let image = image_from_icon(icon, button_size);
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

fn image_from_icon<'a>(icon: Icon, size: f32) -> Image<'a> {
    Image::new(icon.source()).fit_to_exact_size(Vec2::splat(size))
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ButtonOption {
    Icon(Icon),
    Shortcut(KeyboardShortcut),
    Menu(f32),
    SideMenu(f32),
}

fn simple_button(app: &App, ui: &mut Ui, text: &str) -> Option<Response> {
    button(app, ui, vec![], text, |_| {})
}

fn option_button(
    app: &App,
    ui: &mut Ui,
    options: Vec<ButtonOption>,
    text: &str,
) -> Option<Response> {
    button(app, ui, options, text, |_| {})
}

fn button<'a, R>(
    app: &App,
    ui: &mut Ui,
    options: Vec<ButtonOption>,
    text: &str,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> Option<Response> {
    let icon = options
        .iter()
        .find_map(|opt| match opt {
            ButtonOption::Icon(icon) => Some(*icon),
            _ => None,
        })
        .unwrap_or(Icon::Empty);
    let image = image_from_icon(icon, MENU_ICON_SIZE);

    let width = *options.iter().find_map(|opt| match opt {
        ButtonOption::Menu(w) => Some(w),
        ButtonOption::SideMenu(w) => Some(w),
        _ => None,
    }).unwrap_or(&100.);

    if options.iter().any(|it| matches!(it, ButtonOption::SideMenu(_))) || options.iter().any(|it| matches!(it, ButtonOption::Menu(_))) {
        ui.horizontal(|ui| {
            if options.iter().any(|it| matches!(it, ButtonOption::SideMenu(_))) {
                ui.with_layout(Layout::top_down_justified(Align::LEFT), |ui| {
                    ui.menu_image_text_button(image, text, |ui| {
                        ui.set_min_width(width);
                        add_contents(ui);
                    });
                });
            } else {
                ui.menu_button(text, |ui| {
                    ui.set_min_width(width);
                    add_contents(ui);
                });
            }
        });

        return None;
    }

    let shortcut = options.iter().find_map(|opt| match opt {
        ButtonOption::Shortcut(s) => Some(s.to_formatted_string(app)),
        _ => None,
    });

    let button = Button::image_and_text(image, text);
    if let Some(sc) = shortcut {
        Some(ui.add(button.shortcut_text(sc)))
    } else {
        Some(ui.add(button))
    }
}