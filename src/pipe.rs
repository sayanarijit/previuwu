use crate::message::Message;
use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::str::FromStr;
use std::sync::mpsc;

#[derive(Debug, Clone)]
pub(crate) enum Pipe {
    Fifo(String),
    Std,
}

impl FromStr for Pipe {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "-" {
            Ok(Self::Std)
        } else {
            Ok(Self::Fifo(s.to_owned()))
        }
    }
}

impl Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pipe::Fifo(path) => f.write_str(path),
            Pipe::Std => f.write_str("-"),
        }
    }
}

pub(crate) fn start(tx: mpsc::Sender<Message>, input: Pipe) {
    match input {
        Pipe::Std => {
            std::thread::spawn(move || {
                let in_stream = std::io::stdin().lock();
                for line in in_stream.lines() {
                    let path = line.unwrap();
                    tx.send(Message::Preview(path)).unwrap_or_default();
                }
                tx.send(Message::Quit).unwrap();
            });
        }

        Pipe::Fifo(in_) => {
            std::thread::spawn(move || {
                let in_stream = BufReader::new(File::open(in_).expect("failed to open input file"));
                for line in in_stream.lines() {
                    let path = line.unwrap();
                    tx.send(Message::Preview(path)).unwrap_or_default();
                }
                tx.send(Message::Quit).unwrap();
            });
        }
    };
}
