use crate::preview::Previewable;
use anyhow::Error;
use anyhow::Result;
use eframe::Frame;
use egui::Context;
use egui::Ui;
use egui::Vec2;
use egui_extras::RetainedImage;
use std::path::Path;

pub struct Image(RetainedImage);

impl Previewable for Image {
    fn load(path: &Path, _size: Vec2, (_, subtype): (&str, &str)) -> Result<Self> {
        let bytes = std::fs::read(path)?;

        let img = if subtype.starts_with("svg") {
            RetainedImage::from_svg_bytes(path.to_string_lossy(), &bytes).map_err(Error::msg)?
        } else {
            RetainedImage::from_image_bytes(path.to_string_lossy(), &bytes).map_err(Error::msg)?
        };

        Ok(Self(img))
    }

    fn show(&self, _ctx: &Context, _frame: &mut Frame, ui: &mut Ui) {
        self.0.show_max_size(ui, ui.max_rect().size());
    }
}
