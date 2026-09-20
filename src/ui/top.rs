use crate::ui::{find_icon_source, BUTTON_ICON_SIZE, BUTTON_NO_ICON_OFFSET};
use egui::{
    Align, Button, Image, Layout, MenuBar, Panel, Response, TextBuffer, Ui, Vec2, ViewportCommand,
};

const HEADER_NAME: &str = "header";
const DROPDOWN_WIDTH: f32 = 280.;

pub fn render(ui: &mut Ui) {
    Panel::top(HEADER_NAME).show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.add_space(4.0);

            if let Some(lotus) = find_icon_source("lotus") {
                ui.add(Image::new(lotus).fit_to_exact_size(Vec2::splat(25.)));
                ui.add_space(8.0);
            }

            MenuBar::new().ui(ui, |ui| {
                file(ui);
                ui.add_space(4.0);
                code(ui);
                ui.add_space(4.0);
                tools(ui);
            });

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.add_space(8.0);

                if let Some(response) = render_icon_only_button(ui, "close") {
                    if response.clicked() {
                        ui.send_viewport_cmd(ViewportCommand::Close);
                    }
                }

                ui.add_space(4.0);

                if let Some(response) = render_icon_only_button(ui, "minimize") {
                    if response.clicked() {
                        ui.send_viewport_cmd(ViewportCommand::Minimized(true));
                    }
                }

                ui.add_space(32.0);

                if let Some(response) = render_icon_only_button(ui, "settings") {
                    if response.clicked() {}
                }

                ui.add_space(4.0);

                if let Some(response) = render_icon_only_button(ui, "search") {
                    if response.clicked() {}
                }
            });
        });
    });
}

#[derive(PartialEq)]
enum ButtonOption<'a> {
    Icon(&'a str),
    Shortcut(&'a str),
    Menu,
    SideMenu,
}

fn render_button<'a, R>(
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
        "empty"
    };

    if let Some(source) = find_icon_source(icon) {
        let mut button = Button::image_and_text(
            Image::new(source).fit_to_exact_size(Vec2::splat(BUTTON_ICON_SIZE)),
            text,
        );

        if let Some(sc) = shortcut {
            button = button.shortcut_text(sc);
        }

        Some(ui.add(button))
    } else {
        None
    }
}

pub fn render_icon_only_button(ui: &mut Ui, icon: &str) -> Option<Response> {
    let source = find_icon_source(icon)?;
    let button = Button::image(Image::new(source).fit_to_exact_size(Vec2::splat(BUTTON_ICON_SIZE)))
        .frame(false);

    Some(ui.add(button))
}

fn file(ui: &mut Ui) {
    render_button(ui, vec![ButtonOption::Menu], "File", |ui| {
        ui.set_min_width(DROPDOWN_WIDTH);
        render_button(ui, vec![ButtonOption::SideMenu], "New", |ui| {
            ui.set_min_width(DROPDOWN_WIDTH);
            if let Some(response) = render_button(ui, vec![], "Skript File", |_| {}) {
                if response.clicked() {}
            }
            if let Some(response) = render_button(ui, vec![], "File", |_| {}) {
                if response.clicked() {}
            }
            if let Some(response) = render_button(ui, vec![], "Directory", |_| {}) {
                if response.clicked() {}
            }
        });
        if let Some(response) =
            render_button(ui, vec![ButtonOption::Icon("folder")], "Open", |_| {})
        {
            if response.clicked() {}
        }
        render_button(ui, vec![ButtonOption::SideMenu], "Open Recent", |_| {});
        if let Some(response) = render_button(ui, vec![], "Close Project", |_| {}) {
            if response.clicked() {}
        }
        ui.separator();
        if let Some(response) = render_button(ui, vec![], "Exit", |_| {}) {
            if response.clicked() {
                ui.send_viewport_cmd(ViewportCommand::Close);
            }
        }
    });
}

fn code(ui: &mut Ui) {
    render_button(ui, vec![ButtonOption::Menu], "Code", |ui| {
        render_button(ui, vec![ButtonOption::SideMenu], "Generate", |ui| {
            if let Some(response) = render_button(ui, vec![], "Command", |_| {}) {
                if response.clicked() {}
            }
            if let Some(response) = render_button(ui, vec![], "Function", |_| {}) {
                if response.clicked() {}
            }
        });
        if let Some(response) = render_button(ui, vec![], "Reformat", |_| {}) {
            if response.clicked() {}
        }

        ui.separator();

        if let Some(response) = render_button(
            ui,
            vec![
                ButtonOption::Icon("tag"),
                ButtonOption::Shortcut("Ctrl + /"),
            ],
            "Comment Line",
            |_| {},
        ) {
            if response.clicked() {}
        }

        ui.separator();

        if let Some(response) = render_button(
            ui,
            vec![ButtonOption::Shortcut("Ctrl + Up")],
            "Move Line Up",
            |_| {},
        ) {
            if response.clicked() {}
        }
        if let Some(response) = render_button(
            ui,
            vec![ButtonOption::Shortcut("Ctrl + Down")],
            "Move Line Down",
            |_| {},
        ) {
            if response.clicked() {}
        }
        if let Some(response) = render_button(
            ui,
            vec![ButtonOption::Shortcut("Ctrl + Shift + Up")],
            "Add Caret Above",
            |_| {},
        ) {
            if response.clicked() {}
        }
        if let Some(response) = render_button(
            ui,
            vec![ButtonOption::Shortcut("Ctrl + Shift + Down")],
            "Add Caret Below",
            |_| {},
        ) {
            if response.clicked() {}
        }
    });
}

fn tools(ui: &mut Ui) {
    render_button(ui, vec![ButtonOption::Menu], "Tools", |ui| {
        if let Some(response) =
            render_button(ui, vec![ButtonOption::Icon("zip")], "Zip Project", |_| {})
        {
            if response.clicked() {}
        }
    });
}
