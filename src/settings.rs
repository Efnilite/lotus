use crate::locale;
use egui::{Context, Ui};
use serde::{Deserialize, Serialize};

mod keybinds;
mod theme;

const ZOOM_LEVELS: [f32; 13] = [
    0.5, 0.75, 0.9, 1.0, 1.1, 1.25, 1.5, 1.75, 2.0, 2.25, 2.5, 2.75, 3.0,
];

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Settings {
    zoom_index: usize,

    pub keybinds: keybinds::Keybinds,
    pub locale: locale::Locale,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            zoom_index: 3,
            keybinds: Default::default(),
            locale: locale::Locale::load_language("en_US"),
        }
    }
}

impl Settings {
    pub fn init(&mut self, ui: &Context) {
        ui.set_zoom_factor(ZOOM_LEVELS[self.zoom_index]);
    }

    pub fn zoom_in(&mut self, ui: &mut Ui) {
        self.zoom_index = (self.zoom_index + 1).clamp(0, ZOOM_LEVELS.len() - 1);
        ui.set_zoom_factor(ZOOM_LEVELS[self.zoom_index])
    }

    pub fn zoom_out(&mut self, ui: &mut Ui) {
        if let Some(new) = self.zoom_index.checked_sub_signed(1) {
            self.zoom_index = new.clamp(0, ZOOM_LEVELS.len() - 1);
            ui.set_zoom_factor(ZOOM_LEVELS[self.zoom_index])
        }
    }
}
