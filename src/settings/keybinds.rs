use crate::app::App;
use egui::{Key, KeyboardShortcut, Modifiers, Ui};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Keybinds {
    pub copy: KeyboardShortcut,
    pub paste: KeyboardShortcut,
    pub delete: KeyboardShortcut,
    pub undo: KeyboardShortcut,
    pub select_all: KeyboardShortcut,

    pub replace: KeyboardShortcut,
    pub replace_in_file: KeyboardShortcut,

    pub find: KeyboardShortcut,
    pub find_in_file: KeyboardShortcut,

    pub zoom_in: KeyboardShortcut,
    pub zoom_out: KeyboardShortcut,
    pub reset_zoom: KeyboardShortcut,

    pub comment_line: KeyboardShortcut,
    pub reformat: KeyboardShortcut,

    pub move_up: KeyboardShortcut,
    pub move_down: KeyboardShortcut,
    pub add_caret_above: KeyboardShortcut,
    pub add_caret_below: KeyboardShortcut,

    pub project: KeyboardShortcut,
    pub problems: KeyboardShortcut,
    pub terminal: KeyboardShortcut,
    pub analytics: KeyboardShortcut,

    pub search: KeyboardShortcut,
    pub settings: KeyboardShortcut,
}

impl Default for Keybinds {
    fn default() -> Self {
        Keybinds {
            copy: KeyboardShortcut::new(Modifiers::COMMAND, Key::C),
            paste: KeyboardShortcut::new(Modifiers::COMMAND, Key::V),
            delete: KeyboardShortcut::new(Modifiers::COMMAND, Key::X),
            undo: KeyboardShortcut::new(Modifiers::COMMAND, Key::Z),
            select_all: KeyboardShortcut::new(Modifiers::COMMAND, Key::A),
            replace: KeyboardShortcut::new(Modifiers::COMMAND.plus(Modifiers::SHIFT), Key::R),
            replace_in_file: KeyboardShortcut::new(Modifiers::COMMAND, Key::R),
            find: KeyboardShortcut::new(Modifiers::COMMAND.plus(Modifiers::SHIFT), Key::F),
            find_in_file: KeyboardShortcut::new(Modifiers::COMMAND, Key::F),
            zoom_in: KeyboardShortcut::new(Modifiers::COMMAND, Key::Plus),
            zoom_out: KeyboardShortcut::new(Modifiers::COMMAND, Key::Minus),
            reset_zoom: KeyboardShortcut::new(Modifiers::COMMAND, Key::Num0),
            comment_line: KeyboardShortcut::new(Modifiers::COMMAND, Key::Slash),
            reformat: KeyboardShortcut::new(Modifiers::COMMAND.plus(Modifiers::SHIFT), Key::Plus),
            move_up: KeyboardShortcut::new(Modifiers::COMMAND, Key::ArrowUp),
            move_down: KeyboardShortcut::new(Modifiers::COMMAND, Key::ArrowDown),
            add_caret_above: KeyboardShortcut::new(
                Modifiers::COMMAND.plus(Modifiers::SHIFT),
                Key::ArrowUp,
            ),
            add_caret_below: KeyboardShortcut::new(
                Modifiers::COMMAND.plus(Modifiers::SHIFT),
                Key::ArrowDown,
            ),
            project: KeyboardShortcut::new(Modifiers::ALT, Key::Num1),
            problems: KeyboardShortcut::new(Modifiers::ALT, Key::Num2),
            terminal: KeyboardShortcut::new(Modifiers::ALT, Key::Num3),
            analytics: KeyboardShortcut::new(Modifiers::ALT, Key::Num4),

            search: KeyboardShortcut::new(Modifiers::COMMAND.plus(Modifiers::ALT), Key::F),
            settings: KeyboardShortcut::new(Modifiers::COMMAND.plus(Modifiers::ALT), Key::S),
        }
    }
}

impl<'a> IntoIterator for &'a Keybinds {
    type Item = (&'static str, &'a KeyboardShortcut);
    type IntoIter = std::array::IntoIter<Self::Item, 21>;

    fn into_iter(self) -> Self::IntoIter {
        [
            ("Copy", &self.copy),
            ("Paste", &self.paste),
            ("Delete", &self.delete),
            ("Undo", &self.undo),
            ("Select All", &self.select_all),
            ("Replace", &self.replace),
            ("Replace in File", &self.replace_in_file),
            ("Find", &self.search),
            ("Find in File", &self.find_in_file),
            ("Zoom In", &self.zoom_in),
            ("Zoom Out", &self.zoom_out),
            ("Comment", &self.comment_line),
            ("Reformat", &self.reformat),
            ("Move Up", &self.move_up),
            ("Move Down", &self.move_down),
            ("Add Caret Above", &self.add_caret_above),
            ("Add Caret Below", &self.add_caret_below),
            ("Project", &self.project),
            ("Problems", &self.problems),
            ("Terminal", &self.terminal),
            ("Analytics", &self.analytics),
        ]
        .into_iter()
    }
}

impl<'a> IntoIterator for &'a mut Keybinds {
    type Item = (&'static str, &'a mut KeyboardShortcut);
    type IntoIter = std::array::IntoIter<Self::Item, 21>;

    fn into_iter(self) -> Self::IntoIter {
        [
            ("Copy", &mut self.copy),
            ("Paste", &mut self.paste),
            ("Delete", &mut self.delete),
            ("Undo", &mut self.undo),
            ("Select All", &mut self.select_all),
            ("Replace", &mut self.replace),
            ("Replace in File", &mut self.replace_in_file),
            ("Find", &mut self.search),
            ("Find in File", &mut self.find_in_file),
            ("Zoom In", &mut self.zoom_in),
            ("Zoom Out", &mut self.zoom_out),
            ("Comment", &mut self.comment_line),
            ("Reformat", &mut self.reformat),
            ("Move Up", &mut self.move_up),
            ("Move Down", &mut self.move_down),
            ("Add Caret Above", &mut self.add_caret_above),
            ("Add Caret Below", &mut self.add_caret_below),
            ("Project", &mut self.project),
            ("Problems", &mut self.problems),
            ("Terminal", &mut self.terminal),
            ("Analytics", &mut self.analytics),
        ]
        .into_iter()
    }
}

pub trait Formattable {
    fn to_formatted_string(&self, app: &App) -> String;
}

impl Formattable for KeyboardShortcut {
    fn to_formatted_string(&self, app: &App) -> String {
        let mut builder = Vec::with_capacity(3);

        if self.modifiers.command {
            builder.push(app.settings.locale.ctrl.as_str());
        }
        #[cfg(target_os = "macos")]
        if self.modifiers.command {
            builder.push_str(app.settings.locale.cmd.as_str());
        }
        if self.modifiers.alt {
            builder.push(app.settings.locale.alt.as_str());
        }
        if self.modifiers.shift {
            builder.push(app.settings.locale.shift.as_str());
        }

        let key = match self.logical_key {
            Key::ArrowUp => app.settings.locale.arrow_up.as_str(),
            Key::ArrowDown => app.settings.locale.arrow_down.as_str(),
            Key::ArrowLeft => app.settings.locale.arrow_left.as_str(),
            Key::ArrowRight => app.settings.locale.arrow_right.as_str(),

            Key::Escape => app.settings.locale.escape.as_str(),
            Key::Tab => app.settings.locale.tab.as_str(),
            Key::Backspace => app.settings.locale.backspace.as_str(),
            Key::Enter => app.settings.locale.enter.as_str(),
            Key::Space => app.settings.locale.space.as_str(),

            Key::Colon => app.settings.locale.colon.as_str(),
            Key::Comma => app.settings.locale.comma.as_str(),
            Key::Backslash => app.settings.locale.backslash.as_str(),
            Key::Slash => app.settings.locale.slash.as_str(),
            Key::Pipe => app.settings.locale.pipe.as_str(),
            Key::Questionmark => app.settings.locale.question_mark.as_str(),
            Key::Exclamationmark => app.settings.locale.exclamation_mark.as_str(),
            Key::OpenBracket => app.settings.locale.open_bracket.as_str(),
            Key::CloseBracket => app.settings.locale.close_bracket.as_str(),
            Key::OpenCurlyBracket => app.settings.locale.open_curly_bracket.as_str(),
            Key::CloseCurlyBracket => app.settings.locale.close_curly_bracket.as_str(),
            Key::Backtick => app.settings.locale.backtick.as_str(),
            Key::Minus => app.settings.locale.minus.as_str(),
            Key::Period => app.settings.locale.period.as_str(),
            Key::Plus => app.settings.locale.plus.as_str(),
            Key::Equals => app.settings.locale.equals.as_str(),
            Key::Semicolon => app.settings.locale.semicolon.as_str(),
            Key::Quote => app.settings.locale.quote.as_str(),

            _ => self.logical_key.name(),
        };

        builder.push(key);

        builder.join(" + ")
    }
}

impl Keybinds {
    pub fn activate(&self, app: &mut App, ui: &mut Ui) {
        let mut should_zoom_in = false;
        let mut should_zoom_out = false;

        ui.input_mut(|i| {
            for (_action, _shortcut) in self.into_iter() {
                if i.consume_shortcut(&self.copy) {
                    // app.editor.copy_selection();
                }
                if i.consume_shortcut(&self.paste) {
                    // app.editor.paste_from_clipboard();
                }
                if i.consume_shortcut(&self.undo) {
                    // app.editor.undo();
                }
                if i.consume_shortcut(&self.zoom_in) {
                    should_zoom_in = true;
                }
                if i.consume_shortcut(&self.zoom_out) {
                    should_zoom_out = true;
                }
            }
        });

        if should_zoom_in {
            app.settings.zoom_in(ui);
        }
        if should_zoom_out {
            app.settings.zoom_out(ui);
        }
    }
}
