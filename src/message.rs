use anyhow::Error;

pub(crate) enum Message {
    Preview(String),
    Error(Error),
    Quit,
}
