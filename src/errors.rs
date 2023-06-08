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
    #[error("Format error")]
    Fmt(#[from] std::io::Error),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
