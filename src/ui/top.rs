use crate::app::App;
use crate::settings;
use crate::ui::{large_icon_button, ActiveWindow, Icon, BUTTON_NO_ICON_OFFSET, MENU_ICON_SIZE};
use egui::{
    Align, Button, Frame, Image, Layout, Margin, MenuBar, Panel, PointerButton, Response, Sense,
    TextBuffer, Ui, Vec2, ViewportCommand,
};

const HEADER_NAME: &str = "header";
const DROPDOWN_WIDTH: f32 = 280.;

pub fn render(app: &mut App, ui: &mut Ui) {
    Panel::top(HEADER_NAME)
        .frame(Frame::default().inner_margin(Margin {
            top: 8,
            bottom: 8,
            left: 8,
            right: 16,
        }))
        .show(ui, |ui| {
            let header_rect = ui.max_rect();
            let header_response = ui.interact(
                header_rect,
                ui.make_persistent_id("header_drag_area"),
                Sense::click_and_drag(),
            );

            ui.horizontal(|ui| {
                ui.add_space(2.0);
                ui.add(Image::new(Icon::Lotus.source()).fit_to_exact_size(Vec2::splat(25.)));
                ui.add_space(8.0);

                MenuBar::new().ui(ui, |ui| {
                    file(ui);
                    ui.add_space(4.0);
                    code(ui);
                    ui.add_space(4.0);
                    tools(ui);
                    ui.add_space(4.0);
                    view(app, ui);
                    ui.add_space(4.0);
                    help(ui);
                });

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.add_space(4.0);

                    if let Some(response) = large_icon_button(ui, Icon::Close) {
                        if response.clicked() {
                            ui.send_viewport_cmd(ViewportCommand::Close);
                        }
                    }

                    ui.add_space(2.0);

                    if let Some(response) = large_icon_button(ui, Icon::Maximize) {
                        if response.clicked() {
                            let is_maximized =
                                ui.ctx().input(|i| i.viewport().maximized.unwrap_or(false));

                            ui.send_viewport_cmd(ViewportCommand::Maximized(!is_maximized));
                        }
                    }

                    ui.add_space(2.0);

                    if let Some(response) = large_icon_button(ui, Icon::Minimize) {
                        if response.clicked() {
                            ui.send_viewport_cmd(ViewportCommand::Minimized(true));
                        }
                    }

                    ui.add_space(32.0);

                    if let Some(response) = large_icon_button(ui, Icon::Settings) {
                        if response.clicked() {
                            app.active_window = Some(ActiveWindow::Settings);
                        }
                    }

                    ui.add_space(2.0);

                    if let Some(response) = large_icon_button(ui, Icon::Search) {
                        if response.clicked() {
                            app.active_window = Some(ActiveWindow::Search);
                        }
                    }
                });
            });

            if header_response.double_clicked_by(PointerButton::Primary) {
                let is_maximized = ui.ctx().input(|i| i.viewport().maximized.unwrap_or(false));
                ui.ctx()
                    .send_viewport_cmd(ViewportCommand::Maximized(!is_maximized));
            } else if header_response.drag_started_by(PointerButton::Primary) {
                let is_maximized = ui.ctx().input(|i| i.viewport().maximized.unwrap_or(false));

                if is_maximized {
                    ui.ctx()
                        .send_viewport_cmd(ViewportCommand::Maximized(false));
                }

                ui.ctx().send_viewport_cmd(ViewportCommand::StartDrag);
            }
        });
}

#[derive(PartialEq)]
enum ButtonOption<'a> {
    Icon(Icon),
    Shortcut(&'a str),
    Menu,
    SideMenu,
}

fn simple_button(ui: &mut Ui, text: &str) -> Option<Response> {
    button(ui, vec![], text, |_| {})
}

fn option_button(ui: &mut Ui, options: Vec<ButtonOption>, text: &str) -> Option<Response> {
    button(ui, options, text, |_| {})
}

fn button<'a, R>(
    ui: &mut Ui,
    options: Vec<ButtonOption>,
    text: &str,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> Option<Response> {
    if options.contains(&ButtonOption::SideMenu) || options.contains(&ButtonOption::Menu) {
        ui.horizontal(|ui| {
            if options.contains(&ButtonOption::SideMenu) {
                ui.add_space(BUTTON_NO_ICON_OFFSET);
            }

            ui.menu_button(text, |ui| {
                ui.set_min_width(DROPDOWN_WIDTH);
                add_contents(ui);
            });
        });

        return None;
    }

    let shortcut = options.iter().find_map(|opt| {
        if let ButtonOption::Shortcut(s) = opt {
            Some(s.as_str())
        } else {
            None
        }
    });

    let icon = if let Some(ButtonOption::Icon(icon)) = options
        .iter()
        .find(|opt| matches!(opt, ButtonOption::Icon(_)))
    {
        icon
    } else {
        &Icon::Empty
    };

    let mut button = Button::image_and_text(
        Image::new(icon.source()).fit_to_exact_size(Vec2::splat(MENU_ICON_SIZE)),
        text,
    );

    if let Some(sc) = shortcut {
        button = button.shortcut_text(sc);
    }

    Some(ui.add(button))
}

fn file(ui: &mut Ui) {
    button(ui, vec![ButtonOption::Menu], "File", |ui| {
        ui.set_min_width(DROPDOWN_WIDTH);
        button(ui, vec![ButtonOption::SideMenu], "New", |ui| {
            ui.set_min_width(DROPDOWN_WIDTH);
            if let Some(response) = simple_button(ui, "Skript File") {
                if response.clicked() {}
            }
            if let Some(response) = simple_button(ui, "File") {
                if response.clicked() {}
            }
            if let Some(response) = simple_button(ui, "Directory") {
                if response.clicked() {}
            }
        });
        if let Some(response) = option_button(ui, vec![ButtonOption::Icon(Icon::Folder)], "Open") {
            if response.clicked() {}
        }
        option_button(ui, vec![ButtonOption::SideMenu], "Open Recent");
        if let Some(response) = simple_button(ui, "Close Project") {
            if response.clicked() {}
        }
        ui.separator();
        if let Some(response) = simple_button(ui, "Exit") {
            if response.clicked() {
                ui.send_viewport_cmd(ViewportCommand::Close);
            }
        }
    });
}

fn code(ui: &mut Ui) {
    button(ui, vec![ButtonOption::Menu], "Code", |ui| {
        button(ui, vec![ButtonOption::SideMenu], "Generate", |ui| {
            if let Some(response) = simple_button(ui, "Command") {
                if response.clicked() {}
            }
            if let Some(response) = simple_button(ui, "Function") {
                if response.clicked() {}
            }
        });
        if let Some(response) = simple_button(ui, "Reformat") {
            if response.clicked() {}
        }

        ui.separator();

        if let Some(response) = option_button(
            ui,
            vec![
                ButtonOption::Icon(Icon::Tag),
                ButtonOption::Shortcut("Ctrl + /"),
            ],
            "Comment Line",
        ) {
            if response.clicked() {}
        }

        ui.separator();

        if let Some(response) = option_button(
            ui,
            vec![ButtonOption::Shortcut("Ctrl + Up")],
            "Move Line Up",
        ) {
            if response.clicked() {}
        }
        if let Some(response) = option_button(
            ui,
            vec![ButtonOption::Shortcut("Ctrl + Down")],
            "Move Line Down",
        ) {
            if response.clicked() {}
        }
        if let Some(response) = option_button(
            ui,
            vec![ButtonOption::Shortcut("Ctrl + Shift + Up")],
            "Add Caret Above",
        ) {
            if response.clicked() {}
        }
        if let Some(response) = option_button(
            ui,
            vec![ButtonOption::Shortcut("Ctrl + Shift + Down")],
            "Add Caret Below",
        ) {
            if response.clicked() {}
        }
    });
}

fn tools(ui: &mut Ui) {
    button(ui, vec![ButtonOption::Menu], "Tools", |ui| {
        if let Some(response) =
            option_button(ui, vec![ButtonOption::Icon(Icon::Zip)], "Zip Project")
        {
            if response.clicked() {}
        }
    });
}

fn view(app: &mut App, ui: &mut Ui) {
    button(ui, vec![ButtonOption::Menu], "View", |ui| {
        if let Some(response) =
            option_button(ui, vec![ButtonOption::Shortcut("Ctrl + Plus")], "Zoom In")
        {
            if response.clicked() {
                app.settings.zoom_index = (app.settings.zoom_index + 1).clamp(0, settings::ZOOM_LEVELS.len() - 1);
                ui.set_zoom_factor(settings::ZOOM_LEVELS[app.settings.zoom_index])
            }
        }
        if let Some(response) =
            option_button(ui, vec![ButtonOption::Shortcut("Ctrl + Minus")], "Zoom Out")
        {
            if response.clicked() {
                if let Some(new) = app.settings.zoom_index.checked_sub_signed(1) {
                    app.settings.zoom_index = new.clamp(0, settings::ZOOM_LEVELS.len() - 1);
                    ui.set_zoom_factor(settings::ZOOM_LEVELS[app.settings.zoom_index])
                }
            }
        }
    });
}

fn help(ui: &mut Ui) {
    button(ui, vec![ButtonOption::Menu], "Help", |ui| {
        if let Some(response) = option_button(ui, vec![ButtonOption::Icon(Icon::Search)], "GitHub")
        {
            if response.clicked() {}
        }
    });
}
