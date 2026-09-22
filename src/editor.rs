use serde::{Deserialize, Serialize};
use std::ops::Range;

pub struct TextError {
    range: Range<usize>,
}

#[derive(Serialize, Deserialize)]
pub struct Editor {
    pub root_folder: String,
    pub viewing_files: Vec<String>,
    pub opened_files: Vec<String>,
}

impl Default for Editor {
    fn default() -> Self {
        Editor {
            root_folder: "/home/efy/Projects/lotus".to_string(),
            viewing_files: vec![],
            opened_files: vec![],
        }
    }
}
