use anyhow::Result;
use eframe::Frame;
use egui::Context;
use egui::Ui;
use std::fs::Metadata;
use std::path::Path;

pub struct Binary(Metadata);

impl Binary {
    pub fn load(path: &Path) -> Result<Self> {
        Ok(Self(path.metadata()?))
    }

    pub fn show(&self, _ctx: &Context, _frame: &mut Frame, ui: &mut Ui) {
        // TODO: show more info
        ui.label("Binary");
        ui.label(format!("Readonly: {}", self.0.permissions().readonly()));
    }
}
