//!
//! Output to projstring
//!
use crate::builder::Node;
use crate::errors::{Error, Result};
use crate::methods::{find_method_mapping, MethodMapping};
use crate::model::*;

use std::borrow::Cow;
use std::fmt::{self, Write};

#[derive(Default)]
pub struct ProjStringFormatter<T: Write> {
    w: T,
}

impl<T: Write> ProjStringFormatter<T> {
    pub fn new(w: T) -> Self {
        Self { w }
    }

    pub fn format(&mut self, node: &Node) -> Result<()> {
        match node {
            Node::GEOGCS(cs) => self.from_geogcs(cs),
            Node::PROJCS(cs) => self.from_projcs(cs),
            Node::COMPOUNDCRS(crs) => match &crs.h_crs {
                Horizontalcrs::Projcs(cs) => self.from_projcs(cs),
                Horizontalcrs::Geogcs(cs) => self.from_geogcs(cs),
                _ => Err(Error::WktError(format!(
                    "Cannot create proj string from {node:?}"
                ))),
            },
            _ => Err(Error::WktError(format!(
                "Cannot create projstring from {node:?}"
            ))),
        }
    }

    fn from_geogcs(&mut self, geogcs: &Geogcs) -> Result<()> {
        self.w.write_str("+proj=longlat")?;
        self.add_datum(&geogcs.datum)
    }

    fn add_datum(&mut self, datum: &Datum) -> Result<()> {
        self.add_ellipsoid(&datum.ellipsoid)?;
        if datum.to_wgs84.is_empty() {
            // Assume WGS84 or GRS80 compatible
            self.w.write_str(" +towgs84=0,0,0,0,0,0,0")?;
        } else {
            self.w.write_str(" +towgs84=")?;
            datum.to_wgs84.iter().try_fold("", |sep, n| {
                write!(self.w, "{sep}{n}").map_err(Error::from).and(Ok(","))
            })?;
        }
        Ok(())
    }

    // Since we do not use database, output ellipsoid parameters
    // and get rid of ellipsoid name and authority
    fn add_ellipsoid(&mut self, ellps: &Ellipsoid) -> Result<()> {
        // TODO Convert to meter if unit are specified
        let a = ellps.a;
        let rf = ellps.rf;
        write!(self.w, " +a={a} +rf={rf}")?;
        Ok(())
    }

    fn from_projcs(&mut self, projcs: &Projcs) -> Result<()> {
        // Check the projection
        if let Some(mapping) = find_method_mapping(&projcs.projection.method) {
            write!(self.w, "+proj={}", mapping.proj_name())?;
            self.add_parameters(&projcs.projection.parameters, mapping)?;
            self.add_datum(&projcs.geogcs.datum)?;
            let proj_aux = mapping.proj_aux();
            if !proj_aux.is_empty() {
                write!(self.w, " {proj_aux}")?;
            }
            Ok(())
        } else {
            Err(Error::WktError(format!(
                "No projection mapping found for {:?}",
                projcs.projection.method
            )))
        }
    }

    fn add_parameters(&mut self, params: &[Parameter], mapping: &MethodMapping) -> Result<()> {
        params.iter().try_for_each(|p| {
            if let Some(pp) = mapping.find_proj_param(p) {
                // TODO convert to correct units
                write!(self.w, " +{}={}", pp.proj_name, p.value)
            } else {
                Ok(())
            }
        })?;
        Ok(())
    }
}

// ==============================
//  Tests
// ==============================
#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::Builder;
    use crate::parser::parse;
    use crate::tests::{fixtures, setup};

    fn to_projstring(i: &str) -> Result<String> {
        let mut buf = String::new();
        Builder::new()
            .parse(i)
            .and_then(|node| ProjStringFormatter::new(&mut buf).format(&node))
            .and(Ok(buf))
    }

    #[test]
    fn convert_projcs_nad83() {
        setup();
        let wkt = to_projstring(fixtures::WKT_PROJCS_NAD83).unwrap();
        assert_eq!(
            wkt,
            concat!(
                "+proj=lcc +lat_1=42.68333333333333 +lat_2=41.71666666666667",
                " +lat_0=-41 +lon_0=-71.5 +x_0=200000 +y_0=750000 +a=6378137",
                " +rf=298.257222101 +towgs84=0,0,0,0,0,0,0",
            )
        );
    }
}
