use crate::message::Message;
use crate::pipe;
use crate::preview::Content;
use crate::preview::Preview;
use anyhow::Error;
use anyhow::Result;
use eframe::{CreationContext, NativeOptions};
use egui::Vec2;
use pipe::Pipe;
use std::sync::mpsc;

pub(crate) struct App {
    title: String,
    preview: Option<Preview>,
    last_preview: Option<Preview>,
    sender: mpsc::Sender<Message>,
    receiver: mpsc::Receiver<Message>,
    options: NativeOptions,
    active_inputs: usize,
}

impl App {
    pub(crate) fn new<S>(title: S) -> Self
    where
        S: Into<String>,
    {
        let (sender, receiver) = std::sync::mpsc::channel();

        Self {
            title: title.into(),
            preview: None,
            last_preview: None,
            sender,
            receiver,
            options: NativeOptions::default(),
            active_inputs: 0,
        }
    }

    pub(crate) fn with_preview<S>(mut self, path: S) -> Self
    where
        S: Into<String>,
    {
        let path = path.into();
        if let Err(err) = self.sender.send(Message::Preview(path.clone())) {
            self.preview = Some(Preview::new(path, Content::Error(err.into())));
        }
        self
    }

    pub(crate) fn with_pipe(mut self, pipe: Pipe) -> Result<Self> {
        self.active_inputs += 1;
        pipe::start(self.sender.clone(), pipe)?;
        Ok(self)
    }

    pub(crate) fn with_pipes<I>(mut self, pipes: I) -> Result<Self>
    where
        I: IntoIterator<Item = Pipe>,
    {
        for pipe in pipes {
            self = self.with_pipe(pipe)?;
        }
        Ok(self)
    }

    pub(crate) fn run(self) {
        eframe::run_native(
            &self.title.clone(),
            self.options.clone(),
            Box::new(|_cc: &CreationContext| Box::new(self)),
        );
    }

    fn get_last_msg(&mut self) -> Option<Message> {
        let mut res = None;
        while let Ok(msg) = self.receiver.try_recv() {
            res = Some(msg);
            if matches!(res, Some(Message::Quit)) {
                // Don't skip counting quit.
                break;
            }
        }
        res
    }

    fn load(&mut self, path: String, size: Vec2) {
        let preview = Preview::load(path, size);
        std::mem::swap(&mut self.preview, &mut self.last_preview);
        self.preview = Some(preview);
    }

    fn render_preview(
        &mut self,
        ctx: &egui::Context,
        frame: &mut eframe::Frame,
        newpath: Option<String>,
    ) -> Result<()> {
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(path) = newpath {
                self.load(path, ui.available_size().floor())
            }

            if let Some(preview) = self.preview.as_ref() {
                preview.show(ctx, frame, ui)
            }
        });

        Ok(())
    }

    fn render_err(&mut self, ctx: &egui::Context, _f: &mut eframe::Frame, err: Error) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Error");
            ui.label(err.to_string());
        });
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.request_repaint();

        let mut newpath = None;

        match self.get_last_msg() {
            Some(Message::Preview(p)) => {
                if Some(&p) != self.preview.as_ref().map(|p| &p.path) {
                    newpath = Some(p)
                }
            }
            Some(Message::Error(err)) => {
                self.preview = Some(Preview::new("Error", Content::Error(err)));
            }
            Some(Message::Quit) => {
                if self.active_inputs <= 1 {
                    frame.quit()
                } else {
                    self.active_inputs -= 1;
                }
            }
            None => {}
        }

        if let Err(err) = self.render_preview(ctx, frame, newpath) {
            self.render_err(ctx, frame, err);
            return;
        }
    }
}
