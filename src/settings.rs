use serde::{Deserialize, Serialize};

pub const ZOOM_LEVELS: [f32; 13] = [0.5, 0.75, 0.9, 1.0, 1.1, 1.25, 1.5, 1.75, 2.0, 2.25, 2.5, 2.75, 3.0];

#[derive(Deserialize, Serialize)]
pub struct Settings {

    pub zoom_index: usize,

}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            zoom_index: 3,
        }
    }
}