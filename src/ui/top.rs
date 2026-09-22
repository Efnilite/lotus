use crate::app::App;
use crate::settings::keybinds::Formattable;
use crate::ui::icon::Icon;
use crate::ui::{image_from_icon, large_icon_button, ActiveWindow, MENU_ICON_SIZE};
use egui::{
    Align, Button, Frame, Image, KeyboardShortcut, Layout, Margin, MenuBar, Panel, PointerButton,
    Response, Sense, Ui, Vec2, ViewportCommand,
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
                ui.add(image_from_icon(Icon::Lotus, 25.));
                ui.add_space(8.0);

                MenuBar::new().ui(ui, |ui| {
                    file(app, ui);
                    ui.add_space(4.0);
                    edit(app, ui);
                    ui.add_space(4.0);
                    code(app, ui);
                    ui.add_space(4.0);
                    tools(app, ui);
                    ui.add_space(4.0);
                    view(app, ui);
                    ui.add_space(4.0);
                    help(app, ui);
                });

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.add_space(4.0);

                    let response = large_icon_button(
                        app,
                        ui,
                        Icon::Close,
                        app.settings.locale.close.as_str(),
                        None,
                    );
                    if response.clicked() {
                        ui.send_viewport_cmd(ViewportCommand::Close);
                    }

                    ui.add_space(4.0);

                    let response = large_icon_button(
                        app,
                        ui,
                        Icon::Maximize,
                        app.settings.locale.maximize.as_str(),
                        None,
                    );
                    if response.clicked() {
                        let is_maximized =
                            ui.ctx().input(|i| i.viewport().maximized.unwrap_or(false));

                        ui.send_viewport_cmd(ViewportCommand::Maximized(!is_maximized));
                    }

                    ui.add_space(4.0);

                    let response = large_icon_button(
                        app,
                        ui,
                        Icon::Minimize,
                        app.settings.locale.minimize.as_str(),
                        None,
                    );
                    if response.clicked() {
                        ui.send_viewport_cmd(ViewportCommand::Minimized(true));
                    }

                    ui.add_space(32.0);

                    let response = large_icon_button(
                        app,
                        ui,
                        Icon::Update,
                        app.settings.locale.update.as_str(),
                        None,
                    );
                    if response.clicked() {}

                    ui.add_space(4.0);

                    let response = large_icon_button(
                        app,
                        ui,
                        Icon::Settings,
                        app.settings.locale.settings.as_str(),
                        Some(app.settings.keybinds.settings),
                    );
                    if response.clicked() {
                        app.view_state.active_window = Some(ActiveWindow::Settings);
                    }

                    ui.add_space(4.0);

                    let response = large_icon_button(
                        app,
                        ui,
                        Icon::Search,
                        app.settings.locale.search.as_str(),
                        Some(app.settings.keybinds.search),
                    );
                    if response.clicked() {
                        app.view_state.active_window = Some(ActiveWindow::Search);
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ButtonOption {
    Icon(Icon),
    Shortcut(KeyboardShortcut),
    Menu,
    SideMenu,
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

    if options.contains(&ButtonOption::SideMenu) || options.contains(&ButtonOption::Menu) {
        ui.horizontal(|ui| {
            if options.contains(&ButtonOption::SideMenu) {
                ui.with_layout(Layout::top_down_justified(Align::LEFT), |ui| {
                    ui.menu_image_text_button(image, text, |ui| {
                        ui.set_min_width(DROPDOWN_WIDTH);
                        add_contents(ui);
                    });
                });
            } else {
                ui.menu_button(text, |ui| {
                    ui.set_min_width(DROPDOWN_WIDTH);
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

fn file(app: &App, ui: &mut Ui) {
    button(
        app,
        ui,
        vec![ButtonOption::Menu],
        app.settings.locale.file.as_str(),
        |ui| {
            ui.set_min_width(DROPDOWN_WIDTH);
            button(
                app,
                ui,
                vec![ButtonOption::SideMenu],
                app.settings.locale.new.as_str(),
                |ui| {
                    ui.set_min_width(DROPDOWN_WIDTH);
                    if let Some(response) =
                        simple_button(app, ui, app.settings.locale.skript_file.as_str())
                        && response.clicked()
                    {}
                    if let Some(response) =
                        simple_button(app, ui, app.settings.locale.file.as_str())
                        && response.clicked()
                    {}
                    if let Some(response) =
                        simple_button(app, ui, app.settings.locale.folder.as_str())
                        && response.clicked()
                    {}
                },
            );
            if let Some(response) = option_button(
                app,
                ui,
                vec![ButtonOption::Icon(Icon::Folder)],
                app.settings.locale.open.as_str(),
            ) && response.clicked()
            {}
            option_button(
                app,
                ui,
                vec![ButtonOption::SideMenu],
                app.settings.locale.open_recent.as_str(),
            );
            if let Some(response) =
                simple_button(app, ui, app.settings.locale.close_project.as_str())
                && response.clicked()
            {}
            ui.separator();
            if let Some(response) = simple_button(app, ui, app.settings.locale.exit.as_str())
                && response.clicked()
            {
                ui.send_viewport_cmd(ViewportCommand::Close);
            }
        },
    );
}

fn edit(app: &App, ui: &mut Ui) {
    button(
        app,
        ui,
        vec![ButtonOption::Menu],
        app.settings.locale.edit.as_str(),
        |ui| {
            if let Some(response) = option_button(
                app,
                ui,
                vec![ButtonOption::Shortcut(app.settings.keybinds.copy)],
                app.settings.locale.copy.as_str(),
            ) && response.clicked()
            {}

            if let Some(response) = option_button(
                app,
                ui,
                vec![ButtonOption::Shortcut(app.settings.keybinds.paste)],
                app.settings.locale.paste.as_str(),
            ) && response.clicked()
            {}

            if let Some(response) = option_button(
                app,
                ui,
                vec![ButtonOption::Shortcut(app.settings.keybinds.delete)],
                app.settings.locale.delete.as_str(),
            ) && response.clicked()
            {}

            if let Some(response) = option_button(
                app,
                ui,
                vec![ButtonOption::Shortcut(app.settings.keybinds.undo)],
                app.settings.locale.undo.as_str(),
            ) && response.clicked()
            {}

            if let Some(response) = option_button(
                app,
                ui,
                vec![ButtonOption::Shortcut(app.settings.keybinds.select_all)],
                app.settings.locale.select_all.as_str(),
            ) && response.clicked()
            {}

            ui.separator();

            if let Some(response) = option_button(
                app,
                ui,
                vec![ButtonOption::Shortcut(app.settings.keybinds.find)],
                app.settings.locale.find.as_str(),
            ) && response.clicked()
            {}

            if let Some(response) = option_button(
                app,
                ui,
                vec![ButtonOption::Shortcut(app.settings.keybinds.find_in_file)],
                app.settings.locale.find_in_file.as_str(),
            ) && response.clicked()
            {}

            ui.separator();

            if let Some(response) = option_button(
                app,
                ui,
                vec![ButtonOption::Shortcut(app.settings.keybinds.replace)],
                app.settings.locale.replace.as_str(),
            ) && response.clicked()
            {}

            if let Some(response) = option_button(
                app,
                ui,
                vec![ButtonOption::Shortcut(
                    app.settings.keybinds.replace_in_file,
                )],
                app.settings.locale.replace_in_file.as_str(),
            ) && response.clicked()
            {}
        },
    );
}

fn code(app: &App, ui: &mut Ui) {
    button(
        app,
        ui,
        vec![ButtonOption::Menu],
        app.settings.locale.code.as_str(),
        |ui| {
            button(
                app,
                ui,
                vec![ButtonOption::SideMenu],
                app.settings.locale.generate.as_str(),
                |ui| {
                    if let Some(response) =
                        simple_button(app, ui, app.settings.locale.command.as_str())
                        && response.clicked()
                    {}
                    if let Some(response) =
                        simple_button(app, ui, app.settings.locale.function.as_str())
                        && response.clicked()
                    {}
                },
            );
            if let Some(response) = simple_button(app, ui, app.settings.locale.reformat.as_str())
                && response.clicked()
            {}

            ui.separator();

            if let Some(response) = option_button(
                app,
                ui,
                vec![
                    ButtonOption::Icon(Icon::Tag),
                    ButtonOption::Shortcut(app.settings.keybinds.comment_line),
                ],
                app.settings.locale.comment_line.as_str(),
            ) && response.clicked()
            {}

            ui.separator();

            if let Some(response) = option_button(
                app,
                ui,
                vec![ButtonOption::Shortcut(app.settings.keybinds.move_up)],
                app.settings.locale.move_up.as_str(),
            ) && response.clicked()
            {}
            if let Some(response) = option_button(
                app,
                ui,
                vec![ButtonOption::Shortcut(app.settings.keybinds.move_down)],
                app.settings.locale.move_down.as_str(),
            ) && response.clicked()
            {}
            if let Some(response) = option_button(
                app,
                ui,
                vec![ButtonOption::Shortcut(
                    app.settings.keybinds.add_caret_above,
                )],
                app.settings.locale.add_caret_above.as_str(),
            ) && response.clicked()
            {}
            if let Some(response) = option_button(
                app,
                ui,
                vec![ButtonOption::Shortcut(
                    app.settings.keybinds.add_caret_below,
                )],
                app.settings.locale.add_caret_below.as_str(),
            ) && response.clicked()
            {}
        },
    );
}

fn tools(app: &App, ui: &mut Ui) {
    button(
        app,
        ui,
        vec![ButtonOption::Menu],
        app.settings.locale.tools.as_str(),
        |ui| {
            if let Some(response) = option_button(
                app,
                ui,
                vec![ButtonOption::Icon(Icon::Zip)],
                app.settings.locale.zip_project.as_str(),
            ) && response.clicked()
            {}
        },
    );
}

fn view(app: &mut App, _ui: &mut Ui) {
    let _view = app.settings.locale.view.to_owned();

    // button(app, ui, vec![ButtonOption::Menu], view.as_str(), |ui| {
    //     if let Some(response) = option_button(app,
    //                                           ui,
    //         vec![ButtonOption::Shortcut(app.settings.keybinds.zoom_in)],
    //         app.settings.locale.zoom_in.as_str(),
    //     ) {
    //         if response.clicked() {
    //             app.settings.zoom_in(ui);
    //         }
    //     }
    //     if let Some(response) = option_button(app,
    //                                           ui,
    //         vec![ButtonOption::Shortcut(app.settings.keybinds.zoom_out)],
    //         app.settings.locale.zoom_out.as_str(),
    //     ) {
    //         if response.clicked() {
    //             app.settings.zoom_out(ui);
    //         }
    //     }
    // });
}

fn help(app: &mut App, ui: &mut Ui) {
    button(
        app,
        ui,
        vec![ButtonOption::Menu],
        app.settings.locale.help.as_str(),
        |ui| {
            if let Some(response) =
                option_button(app, ui, vec![ButtonOption::Icon(Icon::Search)], "GitHub")
                && response.clicked()
            {}
        },
    );
}
