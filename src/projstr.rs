//!
//! Format WKT CRS syntactic tree
//! to projstring
//!
use crate::builder::{parse_number, Node};
use crate::errors::{Error, Result};
use crate::methods::{find_method_mapping, MethodMapping};
use crate::model::*;

use std::io::Write;

/// WKT Formatter that output to [`Write`]
///
/// A formatter will transform a WKT CRS syntactic
/// tree to a proj4 string
///
/// Example:
///
/// ```
/// # const wkt_string: &str = concat!(
/// #    r#"PROJCS["NAD83 / Massachusetts Mainland",GEOGCS["NAD83","#,
/// #    r#"DATUM["North_American_Datum_1983",SPHEROID["GRS 1980",6378137,298.257222101,"#,
/// #    r#"AUTHORITY["EPSG","7019"]],AUTHORITY["EPSG","6269"]],PRIMEM["Greenwich",0,"#,
/// #    r#"AUTHORITY["EPSG","8901"]],UNIT["degree",0.01745329251994328,"#,
/// #    r#"AUTHORITY["EPSG","9122"]],AUTHORITY["EPSG","4269"]],UNIT["metre",1,"#,
/// #    r#"AUTHORITY["EPSG","9001"]],PROJECTION["Lambert_Conformal_Conic_2SP"],"#,
/// #    r#"PARAMETER["standard_parallel_1",42.68333333333333],"#,
/// #    r#"PARAMETER["standard_parallel_2",41.71666666666667],"#,
/// #    r#"PARAMETER["latitude_of_origin", -41],PARAMETER["central_meridian",-71.5],"#,
/// #    r#"PARAMETER["false_easting",200000],PARAMETER["false_northing",750000],"#,
/// #   r#"AUTHORITY["EPSG","26986"],AXIS["X",EAST],AXIS["Y",NORTH]]"#,
/// # );
/// /// Format a WKT CRS str to a String
/// use proj4wkt::{Builder, Formatter};
///
/// let mut buf = String::new();
/// Builder::new()
///    .parse(wkt_string)
///    .and_then(|node| Formatter::new(unsafe { buf.as_mut_vec() }).format(&node))
///    .unwrap()
/// ```
///
pub struct Formatter<T: Write> {
    w: T,
}

impl<T: Write> Formatter<T> {
    /// Create a new Formatter
    pub fn new(w: T) -> Self {
        Self { w }
    }

    /// Format a `Processor` root node output to
    /// a proj4 string
    pub fn format(&mut self, node: &Node) -> Result<()> {
        match node {
            Node::GEOGCRS(cs) => self.add_geogcs(cs),
            Node::PROJCRS(cs) => self.add_projcs(cs),
            Node::COMPOUNDCRS(crs) => match &crs.h_crs {
                Horizontalcrs::Projcs(cs) => self.add_projcs(cs),
                Horizontalcrs::Geogcs(cs) => self.add_geogcs(cs),
            },
            _ => Err(Error::Wkt(
                format!("Cannot create projstring from {node:?}").into(),
            )),
        }
    }

    #[inline]
    fn write_str(&mut self, s: &str) -> std::io::Result<usize> {
        self.w.write(s.as_bytes())
    }

    fn add_geogcs(&mut self, geogcs: &Geogcs) -> Result<()> {
        self.write_str("+proj=longlat")?;
        self.add_datum(&geogcs.datum)
    }

    fn add_datum(&mut self, datum: &Datum) -> Result<()> {
        self.add_ellipsoid(&datum.ellipsoid)?;
        if datum.to_wgs84.is_empty() {
            // Assume WGS84 or GRS80 compatible
            self.write_str(" +towgs84=0,0,0,0,0,0,0")?;
        } else {
            self.write_str(" +towgs84=")?;
            datum.to_wgs84.iter().try_fold("", |sep, n| {
                write!(self.w, "{sep}{n}").map_err(Error::from).and(Ok(","))
            })?;
        }
        Ok(())
    }

    // Since we do not use database, output ellipsoid parameters
    // and get rid of ellipsoid name and authority
    fn add_ellipsoid(&mut self, ellps: &Ellipsoid) -> Result<()> {
        let a = ellps.a;
        let rf = ellps.rf;
        // Check units
        if let Some(unit) = &ellps.unit {
            match unit.unit_type {
                UnitType::Linear => {
                    if unit.factor != 1.0 {
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
                    return Err(Error::Wkt(
                        format!("Unexpected {:?} unit for ellipsoid", unit.unit_type).into(),
                    ));
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

            // TODO check how to get relevant axis units on wkt2

            let axis_unit = projcs.unit.as_ref();
            let geod_unit = projcs.geogcs.unit.as_ref();

            self.add_parameters(&projcs.projection.parameters, mapping, axis_unit, geod_unit)?;
            self.add_datum(&projcs.geogcs.datum)?;

            let proj_aux = mapping.proj_aux();
            if !proj_aux.is_empty() {
                write!(self.w, " {proj_aux}")?;
            }
            Ok(())
        } else {
            Err(Error::Wkt(
                format!(
                    "No projection mapping found for {:?}",
                    projcs.projection.method
                )
                .into(),
            ))
        }
    }

    fn add_parameters(
        &mut self,
        params: &[Parameter],
        mapping: &MethodMapping,
        axis_unit: Option<&Unit>,
        geod_unit: Option<&Unit>,
    ) -> Result<()> {
        fn write_unit<W: Write>(
            w: &mut W,
            name: &str,
            p: &Parameter,
            ref_unit: Option<&Unit>,
        ) -> Result<()> {
            // See https://docs.ogc.org/is/12-063r5/12-063r5.html#66
            // for constraint on parameter's unit
            if let Some(unit) = p.unit.as_ref().or(ref_unit) {
                if unit.unit_type == UnitType::Linear {
                    if unit.factor != 1.0 {
                        return parse_number(p.value).and_then(|value| {
                            write!(w, " +{}={}", name, value * unit.factor).map_err(Error::from)
                        });
                    }
                } else if !unit.name.eq_ignore_ascii_case("degree") {
                    return parse_number(p.value).and_then(|value| {
                        write!(w, " +{}={}", name, (value * unit.factor).to_degrees())
                            .map_err(Error::from)
                    });
                }
            }
            write!(w, " +{}={}", name, p.value).map_err(Error::from)
        }

        params.iter().try_for_each(|p| {
            if let Some(pm) = mapping.find_proj_param(p) {
                match pm.unit_type {
                    UnitType::Linear => write_unit(&mut self.w, pm.proj_name, p, axis_unit),
                    UnitType::Angular => write_unit(&mut self.w, pm.proj_name, p, geod_unit),
                    _ => write!(self.w, " +{}={}", pm.proj_name, p.value).map_err(Error::from),
                }
            } else {
                // Irrelevant proj mapping
                Ok(())
            }
        })?;

        match axis_unit {
            Some(unit) => {
                if unit.factor != 1.0 {
                    write!(self.w, " +to_meter={}", unit.factor)?;
                } else {
                    self.write_str(" +units=m")?;
                }
            }
            None => {
                self.write_str(" +units=m")?;
            }
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
    use crate::tests::{fixtures, setup};

    fn to_projstring(i: &str) -> Result<String> {
        let mut buf = String::new();
        Builder::new()
            .parse(i)
            .and_then(|node| Formatter::new(unsafe { buf.as_mut_vec() }).format(&node))
            .and(Ok(buf))
    }

    #[test]
    fn convert_projcs_nad83() {
        setup();
        let projstr = to_projstring(fixtures::WKT_PROJCS_NAD83).unwrap();
        assert_eq!(
            projstr,
            concat!(
                "+proj=lcc +lat_1=42.68333333333333 +lat_2=41.71666666666667",
                " +lat_0=-41 +lon_0=-71.5 +x_0=200000 +y_0=750000 +units=m +a=6378137",
                " +rf=298.257222101 +towgs84=0,0,0,0,0,0,0",
            )
        );
    }
}
