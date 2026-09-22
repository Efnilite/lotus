use egui::{include_image, ImageSource};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Icon {
    Empty,
    Zip,
    Close,
    Minimize,
    Add,
    Keep,
    Folder,
    Settings,
    Lotus,
    Search,
    Tag,
    Error,
    Warning,
    Terminal,
    Analytics,
    Maximize,
    Update,
}

impl Icon {
    pub fn source(&self) -> ImageSource<'static> {
        match self {
            Icon::Zip => include_image!("../../assets/icons/zip.svg"),
            Icon::Close => include_image!("../../assets/icons/close.svg"),
            Icon::Minimize => include_image!("../../assets/icons/minimize.svg"),
            Icon::Add => include_image!("../../assets/icons/add.svg"),
            Icon::Keep => include_image!("../../assets/icons/keep.svg"),
            Icon::Folder => include_image!("../../assets/icons/folder.svg"),
            Icon::Settings => include_image!("../../assets/icons/settings.svg"),
            Icon::Lotus => include_image!("../../assets/icons/lotus.svg"),
            Icon::Search => include_image!("../../assets/icons/search.svg"),
            Icon::Tag => include_image!("../../assets/icons/tag.svg"),
            Icon::Empty => include_image!("../../assets/icons/empty.svg"),
            Icon::Error => include_image!("../../assets/icons/error.svg"),
            Icon::Warning => include_image!("../../assets/icons/warning.svg"),
            Icon::Terminal => include_image!("../../assets/icons/terminal.svg"),
            Icon::Analytics => include_image!("../../assets/icons/analytics.svg"),
            Icon::Maximize => include_image!("../../assets/icons/maximize.svg"),
            Icon::Update => include_image!("../../assets/icons/update.svg"),
        }
    }
}
