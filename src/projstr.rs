//!
//! Format WKT CRS syntactic tree
//! to projstring
//!
use crate::builder::{Node, parse_number};
use crate::consts::methods;
use crate::errors::{Error, Result};
use crate::methods::{MethodMapping, find_method_mapping};
use crate::model::*;
use crate::params;

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
/// let mut buf = Vec::new();
/// Builder::new()
///    .parse(wkt_string)
///    .and_then(|node| Formatter::new(&mut buf).format(&node))
///    .unwrap();
/// let projstr = String::from_utf8(buf).unwrap();
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
    fn write_str(&mut self, s: &str) -> std::io::Result<()> {
        self.w.write_all(s.as_bytes())
    }

    fn add_geogcs(&mut self, geogcs: &Geogcs) -> Result<()> {
        self.write_str("+proj=longlat")?;
        self.add_datum(&geogcs.datum)?;
        self.add_prime_meridian(geogcs)
    }

    // Prime meridian longitude is in its own unit (WKT2),
    // otherwise in the geodetic crs unit (WKT1)
    fn add_prime_meridian(&mut self, geogcs: &Geogcs) -> Result<()> {
        if let Some(pm) = &geogcs.prime_meridian {
            let lon = parse_number(pm.longitude)?;
            if lon != 0.0 {
                let lon = match pm.unit.as_ref().or(geogcs.unit.as_ref()) {
                    // GDAL WKT1 always writes the Paris meridian in degrees,
                    // even with a grad geodetic crs unit (as corrected by PROJ)
                    _ if pm.name.eq_ignore_ascii_case("Paris")
                        && (lon - 2.33722917).abs() < 1.0e-8 =>
                    {
                        lon
                    }
                    Some(unit) if !is_degree(unit) => {
                        round_degrees((lon * unit.factor).to_degrees())
                    }
                    _ => lon,
                };
                write!(self.w, " +pm={lon}")?;
            }
        }
        Ok(())
    }

    fn add_datum(&mut self, datum: &Datum) -> Result<()> {
        self.add_ellipsoid(&datum.ellipsoid)?;
        if !datum.to_wgs84.is_empty() {
            self.write_str(" +towgs84=")?;
            datum.to_wgs84.iter().try_fold("", |sep, n| {
                write!(self.w, "{sep}{n}").map_err(Error::from).and(Ok(","))
            })?;
        } else if is_wgs84_compatible(&datum.ellipsoid)? {
            self.write_str(" +towgs84=0,0,0,0,0,0,0")?;
        }
        Ok(())
    }

    // Since we do not use database, output ellipsoid parameters
    // and get rid of ellipsoid name and authority
    fn add_ellipsoid(&mut self, ellps: &Ellipsoid) -> Result<()> {
        // Check units
        let a = match &ellps.unit {
            Some(unit) if unit.unit_type != UnitType::Linear => {
                // XXX How to handle this ?
                return Err(Error::Wkt(
                    format!("Unexpected {:?} unit for ellipsoid", unit.unit_type).into(),
                ));
            }
            // Convert to meter
            Some(unit) if unit.factor != 1.0 => {
                round_meters(parse_number(ellps.a)? * unit.factor).to_string()
            }
            _ => ellps.a.to_string(),
        };
        // Inverse flattening is unitless, and 0 defines a sphere
        if parse_number(ellps.rf)? == 0.0 {
            write!(self.w, " +R={a}")?;
        } else {
            write!(self.w, " +a={a} +rf={}", ellps.rf)?;
        }
        Ok(())
    }

    fn add_projcs(&mut self, projcs: &Projcs) -> Result<()> {
        // Check the projection
        let axis_unit = projcs.unit.as_ref();
        let geod_unit = projcs.geogcs.unit.as_ref();

        if let Some(mapping) = find_method_mapping(projcs)? {
            write!(self.w, "+proj={}", mapping.proj_name())?;

            // Polar Stereographic variant B: the pole is
            // given by the sign of the latitude of true scale
            if mapping.is(&methods::POLAR_STEREOGRAPHIC_VARIANT_B) {
                let lat_0 = polar_stereographic_lat_0(&projcs.projection.parameters, mapping)?;
                write!(self.w, " +lat_0={lat_0}")?;
            }

            self.add_parameters(&projcs.projection.parameters, mapping, axis_unit, geod_unit)?;
            self.add_datum(&projcs.geogcs.datum)?;
            self.add_prime_meridian(&projcs.geogcs)?;

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
            unit_type: &UnitType,
            ref_unit: Option<&Unit>,
        ) -> Result<()> {
            // A parameter unit of unknown type (WKT1 'UNIT' keyword)
            // has the type of the parameter
            if let Some(unit) = &p.unit
                && unit.unit_type != UnitType::Unknown
                && unit.unit_type != *unit_type
            {
                return Err(Error::Wkt(
                    format!(
                        "Unexpected {:?} unit for parameter {}",
                        unit.unit_type, p.name
                    )
                    .into(),
                ));
            }
            // See https://docs.ogc.org/is/12-063r5/12-063r5.html#66
            // for constraint on parameter's unit
            if let Some(unit) = p.unit.as_ref().or(ref_unit) {
                match unit_type {
                    UnitType::Linear if unit.factor != 1.0 => {
                        return parse_number(p.value).and_then(|value| {
                            write!(w, " +{}={}", name, round_meters(value * unit.factor))
                                .map_err(Error::from)
                        });
                    }
                    UnitType::Angular if !is_degree(unit) => {
                        return parse_number(p.value).and_then(|value| {
                            write!(
                                w,
                                " +{}={}",
                                name,
                                round_degrees((value * unit.factor).to_degrees())
                            )
                            .map_err(Error::from)
                        });
                    }
                    _ => (),
                }
            }
            write!(w, " +{}={}", name, p.value).map_err(Error::from)
        }

        // Parameters without proj mapping are irrelevant
        params.iter().try_for_each(|p| {
            mapping
                .find_proj_params(p)
                .try_for_each(|pm| match pm.unit_type {
                    UnitType::Linear => {
                        write_unit(&mut self.w, pm.proj_name, p, &pm.unit_type, axis_unit)
                    }
                    UnitType::Angular => {
                        write_unit(&mut self.w, pm.proj_name, p, &pm.unit_type, geod_unit)
                    }
                    _ => write!(self.w, " +{}={}", pm.proj_name, p.value).map_err(Error::from),
                })
        })?;

        if let Some(unit) = axis_unit && unit.factor != 1.0 {
            write!(self.w, " +to_meter={}", unit.factor)?;
        } else {
            self.write_str(" +units=m")?;
        }

        Ok(())
    }
}

// Values computed from unit conversions are rounded to remove
// floating point noise while keeping a sub-micrometric precision
// on the ground: 1e-12 degree is less than 0.06 µm, 1e-7 m is 0.1 µm.
// Values copied from the WKT are not rounded.
//
// Why 12 and not more. The noise in the outputs comes from f64 arithmetic,
// which is one or two units in the last place: 7×10⁻¹⁵° at 46.8°
// and 2.8×10⁻¹⁴° at 180°, or 1–3 nm on the ground (along the equator or a meridian).
//
// Rounding to 12 decimals removes that noise reliably for any angle up to 360°.
// With 13 decimals, two units of noise near 180° can survive.
//
fn round_degrees(value: f64) -> f64 {
    round(value, 1.0e12)
}

// For lengths in metres, the same reasoning as above gives 7 decimals (at most 0.05 µm).
// f64 noise on values up to 10⁷ m is a few nanometres, so 7 decimals removes it:
// 299999.9999999998 → 300000.
//
fn round_meters(value: f64) -> f64 {
    round(value, 1.0e7)
}

// Scale and the scaled value are exact integers in f64 (below 2^53),
// so the division gives the f64 nearest to the rounded decimal.
// Adding 0.0 turns -0.0 into 0.0.
fn round(value: f64, scale: f64) -> f64 {
    (value * scale).round() / scale + 0.0
}

// WKT unit names are free text: trust the EPSG code
// if available, otherwise the conversion factor
fn is_degree(unit: &Unit) -> bool {
    match &unit.authority {
        // degree, degree (supplier to define representation)
        Some(auth) if auth.name.eq_ignore_ascii_case("EPSG") => {
            matches!(auth.code, "9102" | "9122")
        }
        _ => {
            let degree = 1.0_f64.to_radians();
            ((unit.factor - degree) / degree).abs() < 1.0e-10
        }
    }
}

// Datum without TOWGS84 is assumed compatible with WGS84 only
// for the WGS84 and GRS80 ellipsoids (as PROJ does for WGS84,
// NAD83 or ETRS89): otherwise the datum shift is unknown.
fn is_wgs84_compatible(ellps: &Ellipsoid) -> Result<bool> {
    let factor = ellps.unit.as_ref().map_or(1.0, |unit| unit.factor);
    let a = parse_number(ellps.a)? * factor;
    let rf = parse_number(ellps.rf)?;
    Ok((a - 6378137.0).abs() < 1.0e-3
        && [298.257223563, 298.257222101]
            .iter()
            .any(|wgs84_rf| (rf - wgs84_rf).abs() < 1.0e-6))
}

fn polar_stereographic_lat_0(params: &[Parameter], mapping: &MethodMapping) -> Result<i32> {
    let lat_ts = params.iter().find(|p| {
        mapping
            .find_proj_params(p)
            .any(|pm| pm.proj_name == params::LAT_STD_PARALLEL.proj_name)
    });
    match lat_ts {
        Some(p) if parse_number(p.value)? < 0.0 => Ok(-90),
        _ => Ok(90),
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
        crate::wkt_to_projstring(i)
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

    // Writer accepting one byte at a time
    struct OneByteWriter(Vec<u8>);

    impl Write for OneByteWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            match buf.first() {
                Some(b) => {
                    self.0.push(*b);
                    Ok(1)
                }
                None => Ok(0),
            }
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn format_with_partial_writes() {
        setup();
        let node = Builder::new().parse(fixtures::WKT_GEOGCS_WGS84).unwrap();
        let mut w = OneByteWriter(vec![]);
        Formatter::new(&mut w).format(&node).unwrap();
        assert_eq!(
            String::from_utf8(w.0).unwrap(),
            "+proj=longlat +a=6378137 +rf=298.257223563 +towgs84=0,0,0,0,0,0,0",
        );
    }

    #[test]
    fn round_computed_values() {
        // Floating point noise is removed
        assert_eq!(round_degrees(46.799999999999805).to_string(), "46.8");
        assert_eq!(round_degrees(2.3372291700000005).to_string(), "2.33722917");
        assert_eq!(round_meters(299999.9999999998).to_string(), "300000");
        // Sub-micrometric precision is kept
        assert_eq!(
            round_degrees(2.999999566853654).to_string(),
            "2.999999566854"
        );
        assert_eq!(round_meters(6378350.87041).to_string(), "6378350.87041");
        // No negative zero
        assert_eq!(round_degrees(-1.0e-15).to_string(), "0");
    }
}
