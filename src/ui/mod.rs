use egui::{include_image, ImageSource};

pub mod status;
pub mod top;

fn get_icon_source(icon: &str) -> Option<ImageSource<'static>> {
    match icon {
        "zip" => Some(include_image!("../../assets/icons/zip.svg")),
        "close" => Some(include_image!("../../assets/icons/close.svg")),
        "minimize" => Some(include_image!("../../assets/icons/minimize.svg")),
        _ => None,
    }
}
