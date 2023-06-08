//!
//! # WKT CRS parser
//!
//! Convert WKT CRS to proj string
//!
//! Support both WKT1 and WKT2 formats.
//!
//! This is a companion crate for `proj4rs` because of this conversions are limited
//! to projection supported by `proj4rs`. As more projection will be supported
//! in `proj4rs`, more conversions will be supported in `proj4wt`.
//!
//! This crate may be built as WASM package
//!
//! Example:
//! ```
//! use proj4wkt::wkt_to_projstring;
//!
//! const nad83: &str = concat!(
//!    r#"PROJCS["NAD83 / Massachusetts Mainland",GEOGCS["NAD83","#,
//!    r#"DATUM["North_American_Datum_1983",SPHEROID["GRS 1980",6378137,298.257222101,"#,
//!    r#"AUTHORITY["EPSG","7019"]],AUTHORITY["EPSG","6269"]],PRIMEM["Greenwich",0,"#,
//!    r#"AUTHORITY["EPSG","8901"]],UNIT["degree",0.01745329251994328,"#,
//!    r#"AUTHORITY["EPSG","9122"]],AUTHORITY["EPSG","4269"]],UNIT["metre",1,"#,
//!    r#"AUTHORITY["EPSG","9001"]],PROJECTION["Lambert_Conformal_Conic_2SP"],"#,
//!    r#"PARAMETER["standard_parallel_1",42.68333333333333],"#,
//!    r#"PARAMETER["standard_parallel_2",41.71666666666667],"#,
//!    r#"PARAMETER["latitude_of_origin", -41],PARAMETER["central_meridian",-71.5],"#,
//!    r#"PARAMETER["false_easting",200000],PARAMETER["false_northing",750000],"#,
//!    r#"AUTHORITY["EPSG","26986"],AXIS["X",EAST],AXIS["Y",NORTH]]"#,
//! );
//!
//! let projstr = wkt_to_projstring(nad83).unwrap();
//! assert_eq!(
//!     projstr,
//!     concat!(
//!         "+proj=lcc +lat_1=42.68333333333333 +lat_2=41.71666666666667",
//!         " +lat_0=-41 +lon_0=-71.5 +x_0=200000 +y_0=750000 +units=m +a=6378137",
//!         " +rf=298.257222101 +towgs84=0,0,0,0,0,0,0",
//!     )
//! );
//! ```
//!
mod builder;
mod consts;
mod errors;
mod methods;
mod model;
mod params;
mod parse;
mod projstr;

pub mod parser;

pub use builder::Builder;
pub use projstr::Formatter;

use errors::Result;

/// Convert a wkt string to a projstring
pub fn wkt_to_projstring(i: &str) -> Result<String> {
    let mut buf = String::new();
    Builder::new()
        .parse(i)
        .and_then(|node| Formatter::new(unsafe { buf.as_mut_vec() }).format(&node))
        .and(Ok(buf))
}

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
