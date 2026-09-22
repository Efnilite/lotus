use crate::app::App;
use crate::locale;
use crate::ui::icon::Icon;
use crate::ui::{image_from_icon, PROJECT_ICON_SIZE};
use egui::{Frame, Label, Margin, Panel, ScrollArea, Ui};
use egui_ltreeview::{Action, IndentHintStyle, NodeBuilder, RowLayout, TreeView, TreeViewBuilder};
use std::fs;
use std::fs::DirEntry;
use std::path::Path;

const IGNORED_FOLDERS: [&str; 1] = [".git"];
const LEFT_TAB_NAME: &str = "left tab";
const CONTEXT_WIDTH: f32 = 180.;

pub fn render(app: &mut App, ui: &mut Ui) {
    Panel::left(LEFT_TAB_NAME)
        .min_size(100.)
        .max_size(f32::INFINITY)
        .default_size(250.0)
        .resizable(true)
        .frame(Frame::default().inner_margin(Margin {
            top: 8,
            bottom: 8,
            left: 0,
            right: 4,
        }))
        .show(ui, |ui| {
            ScrollArea::both()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    let folder = &app.editor.root_folder.clone();

                    let tree: TreeView<String> = TreeView::new(ui.make_persistent_id("Tree view"))
                        .indent_hint_style(IndentHintStyle::None)
                        .row_layout(RowLayout::AlignedIconsAndLabels)
                        .min_width(0.);

                    if !folder.is_empty() {
                        let (_r, actions) = tree.show_state(ui, &mut app.view_state.tree_state, |builder| {
                            render_dir(&app.settings.locale, builder, Path::new(folder), Some(folder));
                        });

                        for action in actions {
                            match action {
                                Action::Activate(activation) => {
                                    for activated_path in activation.selected {
                                        let path = Path::new(&activated_path);

                                        if path.is_dir() {
                                            let is_expanded = app.view_state.tree_state.is_open(&activated_path).unwrap_or(false);
                                            app.view_state.tree_state.set_openness(activated_path, !is_expanded);
                                        } else {
                                            app.editor.opened_files.insert(activated_path.clone());
                                            app.editor.viewing_files.insert(activated_path.clone());
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                });
        });
}

fn render_dir(locale: &locale::Locale, builder: &mut TreeViewBuilder<String>, path: &Path, root: Option<&String>) {
    let folder_name = path
        .file_name()
        .map(|it| it.to_string_lossy().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| path.to_string_lossy().to_string());

    let mut folders: Vec<DirEntry> = Vec::with_capacity(32);
    let mut files: Vec<DirEntry> = Vec::with_capacity(32);

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_dir() {
                    if IGNORED_FOLDERS
                        .iter()
                        .any(|x| *x == entry.file_name().to_string_lossy().to_string())
                    {
                        continue;
                    }

                    folders.push(entry);
                } else {
                    files.push(entry);
                }
            }
        }
    } else {
        return;
    }

    let comp = |a: &DirEntry, b: &DirEntry| {
        a.file_name()
            .to_string_lossy()
            .to_lowercase()
            .cmp(&b.file_name().to_string_lossy().to_lowercase())
    };

    folders.sort_by(comp);
    files.sort_by(comp);

    let node = NodeBuilder::dir(path.to_string_lossy().to_string())
        .default_open(root.is_some())
        .label_ui(|ui| {
            ui.add_space(2.);
            ui.add(Label::new(&folder_name).selectable(false));
        })
        .icon(|ui| {
            image_from_icon(Icon::Folder, PROJECT_ICON_SIZE).paint_at(ui, ui.max_rect());
        })
        .activatable(true)
        .context_menu(|ui| {
            ui.set_min_width(CONTEXT_WIDTH);
            ui.label(&folder_name);
            ui.separator();
            if ui.button("New File").clicked() {
                // TODO: Handle file creation action
                ui.close();
            }
            if ui.button(locale.new_folder.to_string()).clicked() {
                ui.close();
            }
            ui.separator();
            if ui.button(locale.delete.to_string()).clicked() {
                // TODO: Handle deletion action
                ui.close();
            }
        });

    builder.node(node);

    for folder in folders {
        render_dir(locale, builder, folder.path().as_path(), None);
    }

    for file in files {
        let file_name = file.file_name().to_string_lossy().to_string();
        let file_id = file.path().to_string_lossy().to_string();

        let leaf_node = NodeBuilder::leaf(file_id)
            .label_ui(|ui| {
                ui.add_space(2.);
                ui.add(Label::new(&file_name).selectable(false));
            })
            .icon(|ui| {
                image_from_icon(Icon::Folder, PROJECT_ICON_SIZE).paint_at(ui, ui.max_rect());
            })
            .activatable(true)
            .context_menu(|ui| {
                ui.set_min_width(CONTEXT_WIDTH);
                ui.label(&file_name);
                ui.separator();
                if ui.button(locale.open.to_string()).clicked() {
                    // TODO: Handle open action
                    ui.close();
                }
                if ui.button(locale.rename.to_string()).clicked() {
                    // TODO: Handle rename action
                    ui.close();
                }
                ui.separator();
                if ui.button(locale.delete.to_string()).clicked() {
                    // TODO: Handle deletion action
                    ui.close();
                }
            });

        builder.node(leaf_node);
    }

    builder.close_dir();
}
