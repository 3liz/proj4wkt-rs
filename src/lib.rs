//!
//! # WKT parser
//!
//! Convert WKT CRS to proj string
//!
mod builder;
mod consts;
mod errors;
mod methods;
mod model;
mod params;
mod parse;
mod parser;
mod projstr;

pub use builder::Builder;
pub use projstr::Formatter;

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// log for logging (optional).
#[cfg(feature = "logging")]
use log;

#[cfg(not(feature = "logging"))]
mod log {
    // Use __XXX__ to prevent 'ambiguous name' error
    // when exporting
    macro_rules! __trace__    ( ($($tt:tt)*) => {{}} );
    macro_rules! __debug__    ( ($($tt:tt)*) => {{}} );
    macro_rules! __error__    ( ($($tt:tt)*) => {{}} );
    macro_rules! __info__     ( ($($tt:tt)*) => {{}} );
    macro_rules! __warn__     ( ($($tt:tt)*) => {{}} );

    #[allow(unused_imports)]
    pub(crate) use {
        __debug__ as debug, __error__ as error, __info__ as info, __trace__ as trace,
        __warn__ as warn,
    };
}

// Include wasm entry point for wasm32-unknown-unknown
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
mod wasm;

#[cfg(test)]
mod tests;
