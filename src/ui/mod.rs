use egui::{include_image, ImageSource};

pub mod status;
pub mod top;

const BUTTON_NO_ICON_OFFSET: f32 = 20.;
const BUTTON_ICON_SIZE: f32 = 15.;

fn get_icon_source(icon: &str) -> Option<ImageSource<'static>> {
    match icon {
        "zip" => Some(include_image!("../../assets/icons/zip.svg")),
        "close" => Some(include_image!("../../assets/icons/close.svg")),
        "minimize" => Some(include_image!("../../assets/icons/minimize.svg")),
        "add" => Some(include_image!("../../assets/icons/add.svg")),
        "keep" => Some(include_image!("../../assets/icons/keep.svg")),
        "folder" => Some(include_image!("../../assets/icons/folder.svg")),
        "settings" => Some(include_image!("../../assets/icons/settings.svg")),
        "lotus" => Some(include_image!("../../assets/icons/lotus.svg")),
        "search" => Some(include_image!("../../assets/icons/search.svg")),
        "tag" => Some(include_image!("../../assets/icons/tag.svg")),
        _ => None,
    }
}
