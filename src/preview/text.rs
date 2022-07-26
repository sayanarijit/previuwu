use anyhow::Result;
use eframe::Frame;
use egui::Context;
use egui::Ui;
use egui::Vec2;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

pub struct Text(Vec<String>);

impl Text {
    pub fn load(path: &Path, size: Vec2) -> Result<Self> {
        let mut lines = vec![];
        let reader = BufReader::new(File::open(path)?);
        for line in reader.lines().take(size.y as usize) {
            lines.push(line?);
        }

        Ok(Self(lines))
    }

    pub fn show(&self, _ctx: &Context, _frame: &mut Frame, ui: &mut Ui) {
        for line in &self.0 {
            ui.label(line);
        }
    }
}
