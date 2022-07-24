use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;

use anyhow::Error;
use anyhow::Result;
use eframe::Frame;
use egui::Context;
use egui::Ui;
use egui_extras::RetainedImage;

pub enum Content {
    Directory(Vec<String>),
    Text(Vec<String>),
    Image(RetainedImage),
    Error(Error),
    Unknown,
}

impl Content {
    fn load(path: &str, height: usize) -> Result<Self> {
        let pathbuf = PathBuf::from(&path);

        let content = if pathbuf.is_dir() {
            let files = pathbuf
                .read_dir()?
                .map(|r| {
                    r.map(|d| d.file_name().to_string_lossy().to_string())
                        .unwrap_or_default()
                })
                .collect::<Vec<String>>();

            Content::Directory(files)
        } else {
            // let ext = pathbuf.extension().and_then(|x| x.to_str());
            let mime = mime_guess::from_path(&pathbuf).first_or_text_plain();
            let type_ = mime.type_().as_str();
            let subtype = mime.type_().as_str();

            match (type_, subtype) {
                ("image", _) => {
                    let bytes = std::fs::read(pathbuf)?;
                    let image =
                        RetainedImage::from_image_bytes(path, &bytes).map_err(Error::msg)?;
                    Content::Image(image)
                }

                ("text", _) => {
                    let mut lines = vec![];
                    let reader = BufReader::new(File::open(pathbuf)?);
                    for line in reader.lines().take(height) {
                        lines.push(line?);
                    }

                    Content::Text(lines)
                }

                (_, _) => Content::Unknown,
            }
        };

        Ok(content)
    }
}

pub(crate) struct Preview {
    pub(crate) path: String,
    pub(crate) content: Content,
}

impl Preview {
    pub(crate) fn load<S>(path: S, height: usize) -> Self
    where
        S: Into<String>,
    {
        let path = path.into();
        let content = Content::load(&path, height).unwrap_or_else(Content::Error);
        Self { path, content }
    }

    pub(crate) fn show(&self, _ctx: &Context, _frame: &mut Frame, ui: &mut Ui) {
        ui.heading(&self.path);

        match &self.content {
            Content::Text(lines) | Content::Directory(lines) => {
                for line in lines {
                    ui.label(line);
                }
            }
            Content::Image(image) => {
                image.show_max_size(ui, ui.min_size());
            }
            Content::Error(err) => {
                for line in err.to_string().lines() {
                    ui.label(line);
                }
            }
            Content::Unknown => {
                ui.label("Unknown");
            }
        }
    }
}
