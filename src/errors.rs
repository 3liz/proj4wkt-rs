//!
//! Crate errors
//!
use std::borrow::Cow;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("WKT parse error")]
    Parse,
    #[error("WKT error: {0}")]
    Wkt(Cow<'static, str>),
    #[error("JS parse error")]
    JsParse,
    #[error("IO error")]
    Io(#[from] std::io::Error),
    #[error("UTF8 error")]
    Utf8(#[from] std::str::Utf8Error),
    #[error("Format error")]
    Fmt(#[from] std::fmt::Error),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
