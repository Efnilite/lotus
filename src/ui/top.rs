use crate::ui::{get_icon_source, BUTTON_ICON_SIZE, BUTTON_NO_ICON_OFFSET};
use egui::{
    Align, Button, Image, IntoAtoms, Layout, MenuBar, Panel, Response, Ui, Vec2,
    ViewportCommand,
};

const HEADER_NAME: &str = "header";
const DROPDOWN_WIDTH: f32 = 180.;

pub fn render(ui: &mut Ui) {
    Panel::top(HEADER_NAME).show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.add_space(4.0);

            if let Some(lotus) = get_icon_source("lotus") {
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
                    if response.clicked() {

                    }
                }

                ui.add_space(4.0);

                if let Some(response) = render_icon_only_button(ui, "search") {
                    if response.clicked() {

                    }
                }
            });
        });
    });
}

fn render_menu_button<'a, R>(
    ui: &mut Ui,
    atoms: impl IntoAtoms<'a>,
    add_contents: impl FnOnce(&mut Ui) -> R,
) {
    ui.scope(|ui| {
        ui.menu_button(atoms, |ui| {
            ui.set_min_width(DROPDOWN_WIDTH);
            add_contents(ui);
        });
    })
    .inner
}

fn render_sub_menu_button<'a, R>(
    ui: &mut Ui,
    atoms: impl IntoAtoms<'a>,
    add_contents: impl FnOnce(&mut Ui) -> R,
) {
    ui.scope(|ui| {
        ui.style_mut().spacing.button_padding.x += BUTTON_NO_ICON_OFFSET;
        ui.menu_button(atoms, |ui| {
            ui.set_min_width(DROPDOWN_WIDTH);
            add_contents(ui)
        });
    })
    .inner
}

pub fn render_icon_only_button(ui: &mut Ui, icon: &str) -> Option<Response> {
    let source = get_icon_source(icon)?;
    let button =
        Button::image(Image::new(source).fit_to_exact_size(Vec2::splat(BUTTON_ICON_SIZE))).frame(false);

    Some(ui.add(button))
}

fn render_icon_button<'a>(ui: &mut Ui, icon: &str, text: &str) -> Option<Response> {
    let source = get_icon_source(icon)?;
    let button = Button::image_and_text(Image::new(source).fit_to_exact_size(Vec2::splat(BUTTON_ICON_SIZE)), text);

    Some(ui.add(button))
}

fn render_button(ui: &mut Ui, text: String) -> Response {
    ui.scope(|ui| {
        ui.style_mut().spacing.button_padding.x += BUTTON_NO_ICON_OFFSET;
        ui.button(text)
    })
    .inner
}

fn file(ui: &mut Ui) {
    render_menu_button(ui, "File", |ui| {
        ui.set_min_width(DROPDOWN_WIDTH);
        render_sub_menu_button(ui, "New", |ui| {
            ui.set_min_width(DROPDOWN_WIDTH);
            if render_button(ui, "Skript File".to_owned()).clicked() {}
            if render_button(ui, "File".to_owned()).clicked() {}
            if render_button(ui, "Directory".to_owned()).clicked() {}
        });
        if let Some(response) = render_icon_button(ui, "folder", "Open") {
            if response.clicked() {}
        }
        render_sub_menu_button(ui, "Open Recent", |ui| {});
        if render_button(ui, "Close Project".to_owned()).clicked() {}
        ui.separator();
        if render_button(ui, "Exit".to_owned()).clicked() {
            ui.send_viewport_cmd(ViewportCommand::Close);
        }
    });
}

fn code(ui: &mut Ui) {
    render_menu_button(ui, "Code", |ui| {
        if render_button(ui, "Reformat".to_owned()).clicked() {}
        ui.separator();
        if let Some(response) = render_icon_button(ui, "tag", "Comment Line") {
            if response.clicked() {}
        }
    });
}

fn tools(ui: &mut Ui) {
    render_menu_button(ui, "Tools", |ui| {
        if let Some(response) = render_icon_button(ui, "zip", "Zip Project") {
            if response.clicked() {}
        }
    });
}
