//!
//! Wasm bindgen entry point
//!
use wasm_bindgen::prelude::*;

use crate::{log, Builder, Formatter};

// Js entry point
#[wasm_bindgen(start)]
pub fn main() {
    #[cfg(feature = "logging")]
    console_log::init_with_level(log::Level::Trace).unwrap();
}

#[wasm_bindgen]
pub fn to_proj(src: &str) -> Result<String, JsError> {
    let mut buf = String::new();
    Builder::new()
        .parse(src)
        .and_then(|node| Formatter::new(&mut buf).format(&node))
        .and(Ok(buf))
        .map_err(JsError::from)
}
