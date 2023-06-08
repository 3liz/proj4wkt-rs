//!
//! Wasm bindgen entry point
//!
use wasm_bindgen::prelude::*;

use crate::wkt_to_projstring;

// Js entry point
#[wasm_bindgen(start)]
pub fn main() {
    #[cfg(feature = "logging")]
    console_log::init_with_level(log::Level::Trace).unwrap();
}

#[wasm_bindgen(js_name = toProjstring)]
pub fn to_projstring(src: &str) -> Result<String, JsError> {
    wkt_to_projstring(src).map_err(JsError::from)
}
