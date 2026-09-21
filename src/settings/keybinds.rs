use crate::app::App;
use egui::{Key, KeyboardShortcut, Modifiers, Ui};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Keybinds {
    copy: KeyboardShortcut,
    paste: KeyboardShortcut,
    delete: KeyboardShortcut,
    undo: KeyboardShortcut,
    select_all: KeyboardShortcut,

    replace: KeyboardShortcut,
    replace_in_file: KeyboardShortcut,

    find: KeyboardShortcut,
    find_in_file: KeyboardShortcut,

    zoom_in: KeyboardShortcut,
    zoom_out: KeyboardShortcut,

    comment: KeyboardShortcut,
    reformat: KeyboardShortcut,

    move_up: KeyboardShortcut,
    move_down: KeyboardShortcut,
    add_caret_above: KeyboardShortcut,
    add_caret_below: KeyboardShortcut,

    project: KeyboardShortcut,
    problems: KeyboardShortcut,
    terminal: KeyboardShortcut,
    analytics: KeyboardShortcut,
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
            zoom_in: KeyboardShortcut::new(Modifiers::COMMAND.plus(Modifiers::SHIFT), Key::Plus),
            zoom_out: KeyboardShortcut::new(Modifiers::COMMAND.plus(Modifiers::SHIFT), Key::Minus),
            comment: KeyboardShortcut::new(Modifiers::COMMAND, Key::Slash),
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
            ("Find", &self.find),
            ("Find in File", &self.find_in_file),
            ("Zoom In", &self.zoom_in),
            ("Zoom Out", &self.zoom_out),
            ("Comment", &self.comment),
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
            ("Find", &mut self.find),
            ("Find in File", &mut self.find_in_file),
            ("Zoom In", &mut self.zoom_in),
            ("Zoom Out", &mut self.zoom_out),
            ("Comment", &mut self.comment),
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

trait Formattable {

    fn format(&self, app: &App) -> String;

}

impl Formattable for KeyboardShortcut {

    fn format(&self, app: &App) -> String {
        let mut builder = String::with_capacity(32);

        if self.modifiers.ctrl {
            builder.push_str(app.settings.locale.ctrl.as_str());
        }
        if self.modifiers.mac_cmd {
            builder.push_str(app.settings.locale.cmd.as_str());
        }
        if self.modifiers.alt {
            builder.push_str(app.settings.locale.alt.as_str());
        }
        if self.modifiers.shift {
            builder.push_str(app.settings.locale.shift.as_str());
        }

        builder
    }

}

impl Keybinds {
    pub fn activate(&self, app: &mut App, ui: &mut Ui) {
        let mut should_zoom_in = false;
        let mut should_zoom_out = false;

        ui.input_mut(|i| {
            for (action, shortcut) in self.into_iter() {
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