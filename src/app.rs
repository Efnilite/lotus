use crate::{settings, ui};
use eframe::Frame;
use egui::{FontData, FontDefinitions, FontFamily, FontId, TextStyle, Ui};
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
pub struct App {

    #[serde(skip)]
    pub active_window: Option<ui::ActiveWindow>,

    pub settings: settings::Settings

}

impl App {
    pub fn new(context: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&context.egui_ctx);

        let mut fonts = FontDefinitions::default();

        fonts.font_data.insert(
            "poppins".to_owned(),
            FontData::from_static(include_bytes!("../assets/fonts/poppins light.ttf")).into(),
        );

        fonts.font_data.insert(
            "jetbrains mono".to_owned(),
            FontData::from_static(include_bytes!("../assets/fonts/jetbrains mono light.ttf"))
                .into(),
        );

        fonts
            .families
            .entry(FontFamily::Proportional)
            .or_default()
            .insert(0, "poppins".to_owned());

        fonts
            .families
            .entry(FontFamily::Monospace)
            .or_default()
            .insert(0, "jetbrains mono".to_owned());

        context.egui_ctx.set_fonts(fonts);

        context.egui_ctx.global_style_mut(|style| {
            style.animation_time = 0.0;

            style
                .text_styles
                .insert(TextStyle::Body, FontId::new(12.0, FontFamily::Proportional));

            style.text_styles.insert(
                TextStyle::Monospace,
                FontId::new(14.0, FontFamily::Monospace),
            );
        });

        if let Some(storage) = context.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut Frame) {
        ui::render(ui, self);
    }
}
