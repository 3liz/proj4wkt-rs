//!
//! Output to projstring
//!
use crate::builder::{parse_number, Node};
use crate::errors::{Error, Result};
use crate::methods::{find_method_mapping, MethodMapping};
use crate::model::*;
use crate::params::ParamMapping;

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
            Node::GEOGCS(cs) => self.add_geogcs(cs),
            Node::PROJCS(cs) => self.add_projcs(cs),
            Node::COMPOUNDCRS(crs) => match &crs.h_crs {
                Horizontalcrs::Projcs(cs) => self.add_projcs(cs),
                Horizontalcrs::Geogcs(cs) => self.add_geogcs(cs),
                _ => Err(Error::WktError(format!(
                    "Cannot create proj string from {node:?}"
                ))),
            },
            _ => Err(Error::WktError(format!(
                "Cannot create projstring from {node:?}"
            ))),
        }
    }

    fn add_geogcs(&mut self, geogcs: &Geogcs) -> Result<()> {
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
        let mut a = ellps.a;
        let mut rf = ellps.rf;
        // Check units
        if let Some(unit) = &ellps.unit {
            match unit.unit_type {
                UnitType::Linear => {
                    if !unit.is_metre() {
                        // Convert to meter
                        let a = parse_number(a)? * unit.factor;
                        let rf = parse_number(rf)? * unit.factor;
                        write!(self.w, " +a={a} +rf={rf}")?;
                    } else {
                        write!(self.w, " +a={a} +rf={rf}")?;
                    }
                }
                _ => {
                    // XXX How to handle this ?
                    return Err(Error::WktError(format!(
                        "Unexpected {:?} unit for ellipsoid",
                        unit.unit_type
                    )));
                }
            }
        } else {
            write!(self.w, " +a={a} +rf={rf}")?;
        }
        Ok(())
    }

    fn add_projcs(&mut self, projcs: &Projcs) -> Result<()> {
        // Check the projection
        if let Some(mapping) = find_method_mapping(&projcs.projection.method) {
            write!(self.w, "+proj={}", mapping.proj_name())?;
            self.add_parameters(&projcs.projection.parameters, mapping, projcs.unit.as_ref())?;
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

    fn add_parameters(
        &mut self,
        params: &[Parameter],
        mapping: &MethodMapping,
        unit: Option<&Unit>,
    ) -> Result<()> {
        fn convert_to_metre(value: &str, pm: &ParamMapping, unit: &Unit) -> Result<f64> {
            let value = parse_number(value)?;
            match unit.unit_type {
                UnitType::Linear => Ok(value * unit.factor),
                _ => {
                    // XXX How to handle this ?
                    Err(Error::WktError(format!(
                        "Unexpected unit '{unit:?}' for parameter '{pm:?}'"
                    )))
                }
            }
        }

        fn convert_to_degree(value: &str, pm: &ParamMapping, unit: &Unit) -> Result<f64> {
            let value = parse_number(value)?;
            match unit.unit_type {
                UnitType::Angular => {
                    // Factor convert to radians
                    Ok((value * unit.factor).to_degrees())
                }
                _ => {
                    // XXX How to handle this ?
                    Err(Error::WktError(format!(
                        "Unexpected unit '{unit:?}' for parameter '{pm:?}'"
                    )))
                }
            }
        }

        params.iter().try_for_each(|p| {
            if let Some(pm) = mapping.find_proj_param(p) {
                if let Some(unit) = p.unit.as_ref().or(unit) {
                    match (&pm.unit_type, &unit.unit_type) {
                        (UnitType::Linear, UnitType::Linear) if !unit.is_metre() => {
                            convert_to_metre(p.value, pm, unit).and_then(|value| {
                                write!(self.w, " +{}={}", pm.proj_name, value).map_err(Error::from)
                            })
                        }
                        (UnitType::Angular, UnitType::Angular) if !unit.is_degree() => {
                            convert_to_degree(p.value, pm, unit).and_then(|value| {
                                write!(self.w, " +{}={}", pm.proj_name, value).map_err(Error::from)
                            })
                        }
                        (UnitType::Scale, UnitType::Scale) if unit.factor != 1.0 => {
                            parse_number(p.value).and_then(|value| {
                                write!(self.w, " +{}={}", pm.proj_name, value * unit.factor)
                                    .map_err(Error::from)
                            })
                        }
                        _ => write!(self.w, " +{}={}", pm.proj_name, p.value).map_err(Error::from),
                    }
                } else {
                    // No units defined, assume default
                    write!(self.w, " +{}={}", pm.proj_name, p.value).map_err(Error::from)
                }
            } else {
                // Irrelevant proj mapping
                Ok(())
            }
        })?;

        match unit {
            Some(unit) => {
                if unit.is_metre() {
                    // WKT1, behave like proj
                    self.w.write_str(" +units=m")?;
                } else {
                    write!(self.w, " +to_meter={}", unit.factor)?;
                }
            }
            None => self.w.write_str(" +units=m")?,
        }

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
                " +lat_0=-41 +lon_0=-71.5 +x_0=200000 +y_0=750000 +units=m +a=6378137",
                " +rf=298.257222101 +towgs84=0,0,0,0,0,0,0",
            )
        );
    }
}
