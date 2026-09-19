use strum::{AsRefStr, EnumIter};

#[derive(EnumIter, AsRefStr)]
pub enum PersistentData {
    #[strum(serialize = "folder")]
    Folder,

    #[strum(serialize = "open files")]
    OpenFiles,

    #[strum(serialize = "selected file")]
    SelectedFile,

    #[strum(serialize = "skript version")]
    SkriptVersion,

    #[strum(serialize = "enabled addons")]
    EnabledAddons,
}
