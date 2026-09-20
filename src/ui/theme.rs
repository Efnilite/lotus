use egui::Color32;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Theme {
    pub primary: Color32,
    pub text: Color32,

    pub header_background: Color32,

}

pub struct EditorTheme {}