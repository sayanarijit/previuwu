use crate::message::Message;
use anyhow::Result;
use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::str::FromStr;
use std::sync::mpsc;
use std::thread::JoinHandle;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
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

pub(crate) fn start(tx: mpsc::Sender<Message>, input: Pipe) -> Result<JoinHandle<()>> {
    let t = match input {
        Pipe::Std => std::thread::spawn(move || {
            let in_stream = std::io::stdin().lock();
            for line in in_stream.lines() {
                match line {
                    Ok(path) => {
                        tx.send(Message::Preview(path)).unwrap_or_default();
                    }
                    Err(err) => {
                        tx.send(Message::Error(err.into())).unwrap_or_default();
                    }
                }
            }
            tx.send(Message::Quit).unwrap();
        }),

        Pipe::Fifo(in_) => {
            let file = File::open(in_)?;
            std::thread::spawn(move || {
                let in_stream = BufReader::new(file);
                for line in in_stream.lines() {
                    match line {
                        Ok(path) => {
                            tx.send(Message::Preview(path)).unwrap_or_default();
                        }
                        Err(err) => {
                            tx.send(Message::Error(err.into())).unwrap_or_default();
                        }
                    }
                }
                tx.send(Message::Quit).unwrap();
            })
        }
    };

    Ok(t)
}
