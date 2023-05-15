//!
//! Crate errors
//!

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("WKT parse error")]
    ParseError,
    #[error("WKT error: {0}")]
    WktError(String),
    #[error("JS parse error")]
    JsParseError,
    #[error("IO error")]
    IoError(#[from] std::io::Error),
    #[error("UTF8 error")]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error("Format error")]
    FmtError(#[from] std::fmt::Error),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
