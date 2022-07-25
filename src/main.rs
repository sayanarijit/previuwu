#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release}

mod app;
mod message;
mod pipe;
mod preview;

use app::App;
use clap::Parser;
use pipe::Pipe;
use std::collections::HashSet;

#[cfg(not(wasm))]
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// The initial path to preview.
    ///
    /// Examples:
    ///
    /// previuwu path/to/preview
    #[clap(value_parser)]
    path: Option<String>,

    /// Specify multiple named pipes (fifo) to stream in the paths to preview.
    ///
    /// The preview window stay open as long as this file is kept open.
    /// Pass '-' to read from stdin instead.
    ///
    /// Example: Read from a named pipe (fifo) and stdin
    ///
    /// previuwu --pipe path/to/input.fifo --pipe -
    #[clap(short, long)]
    pipe: Vec<Pipe>,
}

#[cfg(not(wasm))]
fn main() {
    let args = Args::parse();

    let mut app = App::new("previuwu");

    if let Some(path) = args.path {
        app = app.with_preview(path);
    }

    let pipes: HashSet<Pipe> = HashSet::from_iter(args.pipe.into_iter());
    for pipe in pipes {
        app = app.with_pipe(pipe);
    }

    app.run();
}
