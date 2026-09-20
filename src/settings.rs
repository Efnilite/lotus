use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
pub struct Settings {

    zoom: f32,

}