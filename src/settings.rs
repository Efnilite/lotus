use std::iter::Iterator;
use std::ops::Index;
use crate::locale;
use egui::{Context, Ui};
use serde::{Deserialize, Serialize};

pub mod keybinds;
mod theme;

const ZOOM_LEVELS: [f32; 13] = [0.5, 0.6, 0.75, 0.8, 0.9, 1.0, 1.1, 1.25, 1.5, 1.75, 2.0, 2.25, 2.5];
const DEFAULT_ZOOM_LEVEL: usize = 5;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Settings {
    zoom_index: usize,
    max_double_click_delay: f64,

    pub keybinds: keybinds::Keybinds,
    pub locale: locale::Locale,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            zoom_index: DEFAULT_ZOOM_LEVEL,
            max_double_click_delay: 0.75,
            keybinds: Default::default(),
            locale: locale::Locale::load_language("en_US"),
        }
    }
}

impl Settings {
    pub fn init(&mut self, ui: &Context) {
        ui.set_zoom_factor(ZOOM_LEVELS[self.zoom_index]);
        ui.options_mut(|options| {
            options.input_options.max_double_click_delay = self.max_double_click_delay;
        });
    }

    pub fn reset_zoom(&mut self, ui: &mut Ui) {
        self.zoom_index = DEFAULT_ZOOM_LEVEL;
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

    pub fn set_max_double_click_delay(&mut self, new: f64) {
        self.max_double_click_delay = new.clamp(0., 5.);
    }
}
