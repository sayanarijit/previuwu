use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;

use anyhow::Error;
use anyhow::Result;
use eframe::Frame;
use egui::Context;
use egui::Ui;
use egui::Vec2;
use egui_extras::RetainedImage;

pub enum Content {
    Directory(Vec<String>),
    Text(Vec<String>),
    Image(RetainedImage),
    Error(Error),
    Unknown,
}

impl Content {
    fn load(path: &str, size: Vec2) -> Result<Self> {
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
            let subtype = mime.subtype().as_str();

            match (type_, subtype) {
                ("image", img_ty) => {
                    let bytes = std::fs::read(pathbuf)?;
                    Content::Image(if img_ty.starts_with("svg") {
                        self::from_svg_bytes(path, &bytes, Some(size)).map_err(Error::msg)?
                    } else {
                        RetainedImage::from_image_bytes(path, &bytes).map_err(Error::msg)?
                    })
                }
                ("text", _) => {
                    let mut lines = vec![];
                    let reader = BufReader::new(File::open(pathbuf)?);
                    for line in reader.lines().take(size.y as usize) {
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

// Modified from egui_extras (this should probably be in there anyways, to allow for scaling)
pub fn from_svg_bytes(
    debug_name: impl Into<String>,
    svg_bytes: &[u8],
    render_size: Option<Vec2>,
) -> Result<RetainedImage, String> {
    let mut opt = usvg::Options::default();
    opt.fontdb.load_system_fonts();

    let rtree = usvg::Tree::from_data(svg_bytes, &opt.to_ref()).map_err(|err| err.to_string())?;

    let (w, h) = match render_size {
        Some(Vec2 { x, y }) => (x as _, y as _),
        None => rtree.svg_node().size.to_screen_size().dimensions(),
    };

    let mut pixmap = tiny_skia::Pixmap::new(w, h)
        .ok_or_else(|| format!("Failed to create SVG Pixmap of size {}x{}", w, h))?;

    let fit_to = usvg::FitTo::Size(w, h);
    resvg::render(
        &rtree,
        fit_to,
        tiny_skia::Transform::default(),
        pixmap.as_mut(),
    )
    .ok_or_else(|| "Failed to render SVG".to_owned())?;

    let image = egui::ColorImage::from_rgba_unmultiplied(
        [pixmap.width() as _, pixmap.height() as _],
        pixmap.data(),
    );

    let retained_image = RetainedImage::from_color_image(debug_name, image);

    Ok(retained_image)
}

pub(crate) struct Preview {
    pub(crate) path: String,
    pub(crate) content: Content,
}

impl Preview {
    pub(crate) fn load<S>(path: S, size: Vec2) -> Self
    where
        S: Into<String>,
    {
        let path = path.into();
        let content = Content::load(&path, size).unwrap_or_else(Content::Error);
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
