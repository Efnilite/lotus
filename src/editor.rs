use serde::{Deserialize, Serialize};
use std::ops::Range;

pub struct TextError {
    range: Range<usize>,

}

#[derive(Default, Serialize, Deserialize)]
pub struct Editor {
    pub viewing_files: Vec<String>,
    pub opened_files: Vec<String>,

}