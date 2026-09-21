use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Locale {
    pub name: String,
    pub authors: Vec<String>,

    pub file: String,
    pub edit: String,
    pub code: String,
    pub tools: String,
    pub view: String,
    pub help: String,

    pub new: String,
    pub skript_file: String,
    pub folder: String,
    pub open: String,
    pub open_recent: String,
    pub close_project: String,
    pub exit: String,

    pub copy: String,
    pub paste: String,
    pub delete: String,
    pub undo: String,
    pub select_all: String,
    pub replace: String,
    pub replace_in_file: String,
    pub find: String,
    pub find_in_file: String,

    pub generate: String,
    pub command: String,
    pub function: String,
    pub reformat: String,
    pub comment_line: String,
    pub move_line_up: String,
    pub move_line_down: String,
    pub add_caret_above: String,
    pub add_caret_below: String,

    pub zip_project: String,

    pub zoom_in: String,
    pub zoom_out: String,

    pub search: String,
    pub settings: String,
    pub minimize: String,
    pub maximize: String,
    pub close: String,

    pub project: String,
    pub analytics: String,
    pub terminal: String,
    pub problems: String,

    pub line: String,
    pub column: String,

    pub arrow_up: String,
    pub arrow_down: String,
    pub arrow_left: String,
    pub arrow_right: String,

    pub ctrl: String,
    pub shift: String,
    pub alt: String,
    pub cmd: String,

    pub escape: String,
    pub tab: String,
    pub backspace: String,
    pub enter: String,
    pub space: String,
    pub plus: String,
    pub minus: String,
    pub colon: String,
    pub comma: String,
    pub backslash: String,
    pub slash: String,
    pub pipe: String,
    pub question_mark: String,
    pub exclamation_mark: String,
    pub open_bracket: String,
    pub close_bracket: String,
    pub open_curly_bracket: String,
    pub close_curly_bracket: String,
    pub backtick: String,
    pub period: String,
    pub equals: String,
    pub semicolon: String,
    pub quote: String,
}

impl Locale {

    pub const ALL: [&str; 1] = ["en_US"];

    pub fn load_language(lang_code: &str) -> Self {
        let json_data = match lang_code {
            _ => include_str!("../assets/locales/en_US.json"),
        };

        serde_json::from_str(json_data).unwrap() // if locale is invalid, panic
    }
}

#[test]
fn test_all_locales_valid() {
    for locale in Locale::ALL {
        Locale::load_language(locale);
    }
}