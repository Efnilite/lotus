use crate::app::App;
use crate::ui::icon::Icon;
use crate::ui::{image_from_icon, PROJECT_ICON_SIZE};
use egui::{Frame, Margin, Panel, Ui};
use egui_ltreeview::{
    Action, DirPosition, IndentHintStyle, NodeBuilder, RowLayout, TreeView, TreeViewBuilder,
    TreeViewState,
};
use std::fs;
use std::fs::DirEntry;
use std::path::Path;
use uuid::Uuid;

const IGNORED_FOLDERS: [&str; 1] = [".git"];

const LEFT_TAB_NAME: &str = "left tab";

pub fn render(app: &mut App, ui: &mut Ui) {
    Panel::left(LEFT_TAB_NAME)
        .min_size(200.)
        .max_size(f32::INFINITY)
        .default_size(200.0)
        .resizable(true)
        .frame(Frame::default().inner_margin(Margin {
            top: 8,
            bottom: 8,
            left: 0,
            right: 8,
        }))
        .show(ui, |ui| {
            ui.vertical(|ui| {
                let folder = &app.editor.root_folder.clone();
                let tree: TreeView<Uuid> = TreeView::new(ui.make_persistent_id("Tree view"))
                    .indent_hint_style(IndentHintStyle::None)
                    .row_layout(RowLayout::CompactAlignedLabels)
                    .min_width(0.);

                if !folder.is_empty() {
                    tree.show(ui, |builder| {
                        render_dir(builder, Path::new(folder), Some(folder));
                    });
                }
            });
        });
}

fn render_dir(builder: &mut TreeViewBuilder<Uuid>, path: &Path, root: Option<&String>) {
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

    let node = NodeBuilder::dir(Uuid::new_v4())
        .default_open(root.is_some())
        .icon(|ui| {
            image_from_icon(Icon::Folder, PROJECT_ICON_SIZE).paint_at(ui, ui.max_rect());
        })
        .label(folder_name);

    builder.node(node);

    for folder in folders {
        render_dir(builder, folder.path().as_path(), None);
    }

    for file in files {
        builder.leaf(Uuid::new_v4(), file.file_name().to_string_lossy().to_string());
    }

    builder.close_dir();
}
