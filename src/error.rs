use thiserror::Error;

#[derive(Error, Debug)]
pub enum ZicoDlError {
    #[error(transparent)]
    Network(#[from] reqwest::Error),
    #[error("Unexpected content: {msg}")]
    Content { msg: String },
    #[error("Runtime error: {msg}")]
    Runtime { msg: String },
}
