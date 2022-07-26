use anyhow::Result;
use eframe::Frame;
use egui::Context;
use egui::Ui;
use egui::Vec2;
use std::path::Path;

pub struct Directory(Vec<String>);

impl Directory {
    pub fn load(path: &Path, size: Vec2) -> Result<Self> {
        let files = path
            .read_dir()?
            .take(size.y as usize)
            .map(|r| {
                r.map(|d| d.file_name().to_string_lossy().to_string())
                    .unwrap_or_else(|e| e.to_string())
            })
            .collect::<Vec<String>>();

        Ok(Self(files))
    }

    pub fn show(&self, _ctx: &Context, _frame: &mut Frame, ui: &mut Ui) {
        for line in &self.0 {
            ui.label(line);
        }
    }
}
