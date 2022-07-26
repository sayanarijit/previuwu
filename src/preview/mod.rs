mod directory;
mod image;
mod text;

use self::directory::Directory;
use self::image::Image;
use self::text::Text;
use anyhow::Error;
use anyhow::Result;
use eframe::Frame;
use egui::Context;
use egui::Ui;
use egui::Vec2;
use std::path::PathBuf;

pub enum Content {
    Directory(Directory),
    Text(Text),
    Image(Image),
    Error(Error),
}

pub struct Meta {
    pub mime: (String, String),
}

impl Content {
    fn load(path: &str, size: Vec2) -> Result<Self> {
        let path = PathBuf::from(&path);

        let (type_, subtype) = if path.is_dir() {
            ("inode".to_string(), "directory".to_string())
        } else {
            let m = mime_guess::from_path(&path).first_or_text_plain();
            (m.type_().to_string(), m.subtype().to_string())
        };

        let meta = Meta {
            mime: (type_, subtype),
        };

        let content = match (meta.mime.0.as_str(), meta.mime.1.as_str()) {
            ("inode", "directory") => Self::Directory(Directory::load(&path, size)?),
            ("image", _) => Self::Image(Image::load(&path, size, meta)?),
            ("text", _) => Self::Text(Text::load(&path, size)?),
            (_, _) => Content::Error(Error::msg("Unknown")),
        };

        Ok(content)
    }

    pub(crate) fn show(&self, ctx: &Context, frame: &mut Frame, ui: &mut Ui) {
        match self {
            Content::Directory(p) => p.show(ctx, frame, ui),
            Content::Text(p) => p.show(ctx, frame, ui),
            Content::Image(p) => p.show(ctx, frame, ui),
            Content::Error(err) => {
                for line in err.to_string().lines() {
                    ui.label(line);
                }
            }
        }
    }
}

pub(crate) struct Preview {
    pub(crate) path: String,
    pub(crate) content: Content,
}

impl Preview {
    pub(crate) fn new<S>(path: S, content: Content) -> Self
    where
        S: Into<String>,
    {
        Self {
            path: path.into(),
            content,
        }
    }

    pub(crate) fn load<S>(path: S, size: Vec2) -> Self
    where
        S: Into<String>,
    {
        let path = path.into();
        let content = Content::load(&path, size).unwrap_or_else(Content::Error);
        Self::new(path, content)
    }

    pub(crate) fn show(&self, ctx: &Context, frame: &mut Frame, ui: &mut Ui) {
        ui.heading(&self.path);
        self.content.show(ctx, frame, ui);
    }
}
