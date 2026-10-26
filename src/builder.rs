//!
//! Build proj string
//!
//! ## Specifications
//!
//! * [WKT CRS standards](https://www.ogc.org/standard/wkt-crs/)
//! * [WKT2015 specs](https://docs.ogc.org/is/12-063r5/12-063r5.html)
//! * [WKT2019 specs](https://docs.ogc.org/is/18-010r7/18-010r7.html)
//!
//!
use crate::errors::{Error, Result};
use crate::model::*;
use crate::parser::{Attribute, Processor, parse};

#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq)]
pub enum Node<'a> {
    AUTHORITY(Authority<'a>),
    UNIT(Unit<'a>),
    METHOD(Method<'a>),
    PARAMETER(Parameter<'a>),
    DATUM(Datum<'a>),
    PRIMEM(PrimeMeridian<'a>),
    PROJCRS(Projcs<'a>),
    GEOGCRS(Geogcs<'a>),
    PROJECTION(Projection<'a>),
    ELLIPSOID(Ellipsoid<'a>),
    COMPOUNDCRS(Compoundcrs<'a>),
    VERTICALCRS(Verticalcrs<'a>),
    TOWGS84(Vec<&'a str>),
    AXIS(Axis<'a>),
    CS(&'a str),
    OTHER(&'a str),
}

/// A WKT CRS builder
///
/// A builder implement the WKT CRS grammar and create a syntactic
/// representation of the WKT.
///
#[derive(Debug, Default)]
pub struct Builder;

impl Builder {
    /// Create a new Builder
    pub fn new() -> Self {
        Builder
    }

    /// Parse a WKT string and return the root Node
    pub fn parse<'a>(&self, s: &'a str) -> Result<Node<'a>> {
        parse(s, self)
    }
}

impl<'a> Processor<'a> for Builder {
    type Err = Error;
    type Output = Node<'a>;

    fn process<I>(&self, key: &'a str, _depth: usize, attrs: I) -> Result<Self::Output, Self::Err>
    where
        I: Iterator<Item = Attribute<'a, Self::Output>>,
    {
        match key {
            "AUTHORITY" | "ID" => self.authority(attrs).map(Node::AUTHORITY),
            "PROJCS" | "PROJCRS" | "PROJECTEDCRS" => self.projcs(attrs).map(Node::PROJCRS),
            "GEOGCS" | "GEOGCRS" | "GEOGRAPHICCRS" | "GEODCRS" | "GEODETICCRS" | "BASEGEODCRS"
            | "BASEGEOGCRS" => self.geogcs(attrs).map(Node::GEOGCRS),
            "ELLIPSOID" | "SPHEROID" => self.ellipsoid(attrs).map(Node::ELLIPSOID),
            "CONVERSION" => self.projection(attrs).map(Node::PROJECTION),
            "PROJECTION" | "METHOD" => self.method(attrs).map(Node::METHOD),
            "PARAMETER" => self.parameter(attrs).map(Node::PARAMETER),
            "DATUM" | "GEODETICDATUM" | "TRF" => self.datum(attrs).map(Node::DATUM),
            "ENSEMBLE" => self.ensemble(key, attrs),
            "CS" => self.cs(attrs).map(Node::CS),
            "PRIMEM" | "PRIMEMERIDIAN" => self.primem(attrs).map(Node::PRIMEM),
            "UNIT" | "ANGLEUNIT" | "SCALEUNIT" | "LENGTHUNIT" => {
                self.unit(key, attrs).map(Node::UNIT)
            }
            "COMPD_CS" | "COMPOUNDCRS" => self.compoundcrs(attrs).map(Node::COMPOUNDCRS),
            "VERT_CS" | "VERTCRS" | "VERTICALCRS" => self.verticalcrs(attrs).map(Node::VERTICALCRS),
            "TOWGS84" => self.towgs84(attrs).map(Node::TOWGS84),
            "AXIS" => self.axis(attrs).map(Node::AXIS),
            _ => {
                // Consume tokens
                for _ in attrs {}
                Ok(Node::OTHER(key))
            }
        }
    }
}

impl Builder {
    fn projcs<'a>(
        &self,
        attrs: impl Iterator<Item = Attribute<'a, Node<'a>>>,
    ) -> Result<Projcs<'a>> {
        let mut name = None;
        let mut geogcs = None;
        let mut projection = None;
        let mut method = None;
        let mut unit = None;
        let mut axis_unit = None;
        let mut axis_directions = vec![];
        let mut authority = None;

        let mut parameters: Vec<Parameter<'a>> = vec![];

        for (i, a) in attrs.enumerate() {
            match a {
                Attribute::Quoted(s) if i == 0 => name = Some(s),
                Attribute::Keyword(_, n) => match n {
                    Node::GEOGCRS(cs) => geogcs = Some(cs),
                    Node::PROJECTION(p) => projection = Some(p),
                    // Handle WKT1
                    Node::AUTHORITY(auth) => authority = Some(auth),
                    Node::UNIT(u) => unit = Some(u),
                    Node::METHOD(m) => method = Some(m),
                    Node::PARAMETER(p) => parameters.push(p),
                    Node::AXIS(axis) => {
                        axis_directions.push(axis.direction);
                        // WKT2: keep the first axis unit
                        if axis_unit.is_none() {
                            axis_unit = axis.unit;
                        }
                    }
                    _ => (),
                },
                _ => (),
            }
        }

        // WKT2: a unit common to all axes follows the AXIS list,
        // otherwise the unit is defined on each axis
        if unit.is_none() {
            unit = axis_unit;
        }

        // On pre WKT2 parameters for projection are at the root level
        if projection.is_none() {
            let me = method.ok_or(Error::Wkt("No projection method defined".into()))?;
            projection = Some(Projection {
                name: "Unknown",
                method: me,
                parameters,
                authority,
            });
        }

        if let Some(u) = unit.as_mut() {
            match u.unit_type {
                // Projcs unit should be linear
                UnitType::Unknown => u.unit_type = UnitType::Linear,
                UnitType::Angular => {
                    // Hu ?
                    return Err(Error::Wkt(
                        "Expecting linear unit for projcted crs axis".into(),
                    ));
                }
                _ => (),
            }
        }

        Ok(Projcs {
            name: name.unwrap_or("Unknown"),
            geogcs: geogcs.ok_or(Error::Wkt("Missing PROJCRS geodetic crs".into()))?,
            projection: projection.ok_or(Error::Wkt("Missing PROJCS projection".into()))?,
            unit,
            axis_directions,
        })
    }

    fn projection<'a>(
        &self,
        attrs: impl Iterator<Item = Attribute<'a, Node<'a>>>,
    ) -> Result<Projection<'a>> {
        let mut name = None;
        let mut method = None;
        let mut authority = None;

        let mut parameters: Vec<Parameter<'a>> = vec![];

        for (i, a) in attrs.enumerate() {
            match a {
                Attribute::Quoted(s) if i == 0 => name = Some(s),
                Attribute::Keyword(_, n) => match n {
                    Node::METHOD(m) => method = Some(m),
                    Node::PARAMETER(p) => parameters.push(p),
                    Node::AUTHORITY(auth) => authority = Some(auth),
                    _ => (),
                },
                _ => (),
            }
        }

        Ok(Projection {
            name: name.unwrap_or(""),
            method: method.ok_or(Error::Wkt("Missing METHOD in projection definition".into()))?,
            parameters,
            authority,
        })
    }

    fn method<'a>(
        &self,
        attrs: impl Iterator<Item = Attribute<'a, Node<'a>>>,
    ) -> Result<Method<'a>> {
        let mut name = None;
        let mut authority = None;

        for (i, a) in attrs.enumerate() {
            match a {
                Attribute::Quoted(s) if i == 0 => name = Some(s),
                Attribute::Keyword(_, Node::AUTHORITY(auth)) => authority = Some(auth),
                _ => (),
            }
        }

        Ok(Method {
            name: name.ok_or(Error::Wkt("Missing METHOD or PROJECTION name".into()))?,
            authority,
        })
    }

    fn parameter<'a>(
        &self,
        attrs: impl Iterator<Item = Attribute<'a, Node<'a>>>,
    ) -> Result<Parameter<'a>> {
        let mut name = None;
        let mut value = None;
        let mut unit = None;
        let mut authority = None;

        for (i, a) in attrs.enumerate() {
            match a {
                Attribute::Quoted(s) if i == 0 => name = Some(s),
                Attribute::Number(s) if i == 1 => value = Some(s),
                Attribute::Keyword(_, n) => match n {
                    Node::AUTHORITY(auth) => authority = Some(auth),
                    Node::UNIT(u) => unit = Some(u),
                    _ => (),
                },
                _ => (),
            }
        }

        Ok(Parameter {
            name: name.ok_or(Error::Wkt("Missing PARAMETER name".into()))?,
            value: value.ok_or(Error::Wkt("Missing PARAMETER value".into()))?,
            unit,
            authority,
        })
    }

    fn geogcs<'a>(
        &self,
        attrs: impl Iterator<Item = Attribute<'a, Node<'a>>>,
    ) -> Result<Geogcs<'a>> {
        let mut name = None;
        let mut datum = None;
        let mut prime_meridian = None;
        let mut unit = None;
        let mut axis_unit = None;
        let mut authority = None;
        let mut cs_type = None;

        for (i, a) in attrs.enumerate() {
            match a {
                Attribute::Quoted(s) if i == 0 => name = Some(s),
                Attribute::Keyword(_, n) => match n {
                    Node::DATUM(d) => datum = Some(d),
                    Node::PRIMEM(pm) => prime_meridian = Some(pm),
                    Node::CS(cs) => cs_type = Some(cs),
                    Node::UNIT(u) => unit = Some(u),
                    Node::AUTHORITY(a) => authority = Some(a),
                    // WKT2: keep the first axis angular unit,
                    // skip the ellipsoidal height unit of 3D crs
                    Node::AXIS(Axis { unit: Some(u), .. })
                        if axis_unit.is_none() && u.unit_type != UnitType::Linear =>
                    {
                        axis_unit = Some(u)
                    }
                    _ => (),
                },
                _ => (),
            }
        }

        // WKT2: GEODCRS may be a geocentric crs
        if cs_type.is_some_and(|cs| cs.eq_ignore_ascii_case("Cartesian")) {
            return Err(Error::Wkt("Geocentric crs is not supported".into()));
        }

        // WKT2: a unit common to all axes follows the AXIS list,
        // otherwise the unit is defined on each axis
        if unit.is_none() {
            unit = axis_unit;
        }

        if let Some(u) = unit.as_mut() {
            match u.unit_type {
                // Geogcs unit should be angular
                UnitType::Unknown => u.unit_type = UnitType::Angular,
                UnitType::Linear => {
                    // Hu ?
                    return Err(Error::Wkt("Expecting angular unit for geodetic crs".into()));
                }
                _ => (),
            }
        }

        Ok(Geogcs {
            name: name.unwrap_or(""),
            datum: datum.ok_or(Error::Wkt("Missing DATUM for geodetic crs".into()))?,
            prime_meridian,
            unit,
            authority,
        })
    }

    fn primem<'a>(
        &self,
        attrs: impl Iterator<Item = Attribute<'a, Node<'a>>>,
    ) -> Result<PrimeMeridian<'a>> {
        let mut name = None;
        let mut longitude = None;
        let mut unit = None;

        for (i, a) in attrs.enumerate() {
            match a {
                Attribute::Quoted(s) if i == 0 => name = Some(s),
                Attribute::Number(s) if i == 1 => longitude = Some(s),
                Attribute::Keyword(_, Node::UNIT(u)) => unit = Some(u),
                _ => (),
            }
        }

        if let Some(u) = unit.as_mut() {
            // Prime meridian unit should be angular
            if u.unit_type == UnitType::Unknown {
                u.unit_type = UnitType::Angular;
            }
        }

        Ok(PrimeMeridian {
            name: name.ok_or(Error::Wkt("Missing PRIMEM name".into()))?,
            longitude: longitude.ok_or(Error::Wkt("Missing PRIMEM longitude".into()))?,
            unit,
        })
    }

    fn datum<'a>(&self, attrs: impl Iterator<Item = Attribute<'a, Node<'a>>>) -> Result<Datum<'a>> {
        let mut name = None;
        let mut ellipsoid = None;
        let mut to_wgs84 = vec![];

        for (i, a) in attrs.enumerate() {
            match a {
                Attribute::Quoted(s) if i == 0 => name = Some(s),
                Attribute::Keyword(_, n) => match n {
                    Node::ELLIPSOID(e) => ellipsoid = Some(e),
                    Node::TOWGS84(v) => to_wgs84 = v,
                    _ => (),
                },
                _ => (),
            }
        }

        Ok(Datum {
            name: name.unwrap_or("Unknown"),
            ellipsoid: ellipsoid.ok_or(Error::Wkt("Missing ellipsoid for DATUM".into()))?,
            to_wgs84,
        })
    }

    // WKT2 datum ensemble: a geodetic ensemble is used as datum,
    // a vertical ensemble has no ellipsoid
    fn ensemble<'a>(
        &self,
        key: &'a str,
        attrs: impl Iterator<Item = Attribute<'a, Node<'a>>>,
    ) -> Result<Node<'a>> {
        let mut name = None;
        let mut ellipsoid = None;

        for (i, a) in attrs.enumerate() {
            match a {
                Attribute::Quoted(s) if i == 0 => name = Some(s),
                Attribute::Keyword(_, Node::ELLIPSOID(e)) => ellipsoid = Some(e),
                _ => (),
            }
        }

        Ok(match ellipsoid {
            Some(ellipsoid) => Node::DATUM(Datum {
                name: name.unwrap_or("Unknown"),
                ellipsoid,
                to_wgs84: vec![],
            }),
            None => Node::OTHER(key),
        })
    }

    fn cs<'a>(&self, attrs: impl Iterator<Item = Attribute<'a, Node<'a>>>) -> Result<&'a str> {
        let mut cs_type = None;

        for (i, a) in attrs.enumerate() {
            match a {
                Attribute::Label(s) if i == 0 => cs_type = Some(s),
                _ => (),
            }
        }

        cs_type.ok_or(Error::Wkt("Missing CS type".into()))
    }

    fn authority<'a>(
        &self,
        attrs: impl Iterator<Item = Attribute<'a, Node<'a>>>,
    ) -> Result<Authority<'a>> {
        let mut name = None;
        let mut code = None;

        for (i, a) in attrs.enumerate() {
            match a {
                Attribute::Quoted(s) if i == 0 => name = Some(s),
                Attribute::Number(s) | Attribute::Quoted(s) if i == 1 => code = Some(s),
                _ => (),
            }
        }

        Ok(Authority {
            name: name.ok_or(Error::Wkt("Missing AUTHORITY name".into()))?,
            code: code.ok_or(Error::Wkt("Missing AUTHORITY code".into()))?,
        })
    }

    fn unit<'a>(
        &self,
        key: &'a str,
        attrs: impl Iterator<Item = Attribute<'a, Node<'a>>>,
    ) -> Result<Unit<'a>> {
        let mut name = None;
        let mut factor = None;
        let mut _authority = None;

        for (i, a) in attrs.enumerate() {
            match a {
                Attribute::Quoted(s) if i == 0 => name = Some(s),
                Attribute::Number(s) if i == 1 => factor = Some(parse_number(s)?),
                Attribute::Keyword(_, Node::AUTHORITY(auth)) => _authority = Some(auth),
                _ => (),
            }
        }

        Ok(Unit {
            name: name.ok_or(Error::Wkt("Missing UNIT name".into()))?,
            factor: factor.ok_or(Error::Wkt("Missing UNIT factor".into()))?,
            unit_type: match key {
                "ANGLEUNIT" => UnitType::Angular,
                "SCALEUNIT" => UnitType::Scale,
                "LENGTHUNIT" => UnitType::Linear,
                _ => UnitType::Unknown,
            },
        })
    }

    fn compoundcrs<'a>(
        &self,
        attrs: impl Iterator<Item = Attribute<'a, Node<'a>>>,
    ) -> Result<Compoundcrs<'a>> {
        let mut name = None;
        let mut h_crs = None;
        let mut v_crs = None;

        for (i, a) in attrs.enumerate() {
            match a {
                Attribute::Quoted(s) if i == 0 => name = Some(s),
                Attribute::Keyword(_, n) => match n {
                    Node::PROJCRS(cs) => h_crs = Some(Horizontalcrs::Projcs(cs)),
                    Node::GEOGCRS(cs) => h_crs = Some(Horizontalcrs::Geogcs(cs)),
                    Node::VERTICALCRS(cs) => v_crs = Some(cs),
                    _ => (),
                },
                _ => (),
            }
        }

        Ok(Compoundcrs {
            name: name.ok_or(Error::Wkt("Missing Compound Crs name".into()))?,
            h_crs: h_crs.ok_or(Error::Wkt(
                "Missing Horzontal CRS for compound crs name".into(),
            ))?,
            v_crs: v_crs.ok_or(Error::Wkt("Missing Vertical crs for compound".into()))?,
        })
    }

    fn verticalcrs<'a>(
        &self,
        attrs: impl Iterator<Item = Attribute<'a, Node<'a>>>,
    ) -> Result<Verticalcrs<'a>> {
        let mut name = None;

        for (i, a) in attrs.enumerate() {
            match a {
                Attribute::Quoted(s) if i == 0 => name = Some(s),
                _ => (),
            }
        }

        Ok(Verticalcrs {
            name: name.unwrap_or(""),
        })
    }

    fn ellipsoid<'a>(
        &self,
        attrs: impl Iterator<Item = Attribute<'a, Node<'a>>>,
    ) -> Result<Ellipsoid<'a>> {
        let mut name = None;
        let mut semi_major = None;
        let mut rf = None;
        let mut unit = None;

        for (i, a) in attrs.enumerate() {
            match a {
                Attribute::Quoted(s) if i == 0 => name = Some(s),
                Attribute::Number(s) if i == 1 => semi_major = Some(s),
                Attribute::Number(s) if i == 2 => rf = Some(s),
                Attribute::Keyword(_, Node::UNIT(u)) => unit = Some(u),
                _ => (),
            }
        }

        if let Some(u) = unit.as_mut() {
            // Ellipsoid unit should be linear
            if u.unit_type == UnitType::Unknown {
                u.unit_type = UnitType::Linear;
            }
        }

        Ok(Ellipsoid {
            name: name.ok_or(Error::Wkt("Missing AUTHORITY name".into()))?,
            a: semi_major.ok_or(Error::Wkt("Invalid ELLIPSOID semi-major axis".into()))?,
            rf: rf.ok_or(Error::Wkt("Invalid ELLIPSOID inverse flattening".into()))?,
            unit,
        })
    }

    fn axis<'a>(&self, attrs: impl Iterator<Item = Attribute<'a, Node<'a>>>) -> Result<Axis<'a>> {
        let mut name = None;
        let mut direction = None;
        let mut unit = None;

        for (i, a) in attrs.enumerate() {
            match a {
                Attribute::Quoted(s) if i == 0 => name = Some(s),
                Attribute::Label(s) if i == 1 => direction = Some(s),
                Attribute::Keyword(_, Node::UNIT(u)) => unit = Some(u),
                _ => (),
            }
        }

        Ok(Axis {
            name: name.ok_or(Error::Wkt("Missing AXIS name".into()))?,
            direction: direction.ok_or(Error::Wkt("Missing AXIS direction".into()))?,
            unit,
        })
    }

    fn towgs84<'a>(
        &self,
        attrs: impl Iterator<Item = Attribute<'a, Node<'a>>>,
    ) -> Result<Vec<&'a str>> {
        let mut to_wgs84 = vec![];

        for a in attrs {
            match a {
                Attribute::Number(s) => to_wgs84.push(s),
                _ => {
                    return Err(Error::Wkt(format!("Expecting number not {a:?}").into()));
                }
            }
        }

        if !matches!(to_wgs84.len(), 0 | 3 | 7) {
            return Err(Error::Wkt("Wrong number of parameters for TOWGS84".into()));
        }

        Ok(to_wgs84)
    }
}

use crate::parse::FromStr;

pub fn parse_number(s: &str) -> Result<f64> {
    f64::from_str(s).map_err(|err| Error::Wkt(format!("Error parsing number: {err:?}").into()))
}

/*
pub fn parse_int(s: &str) -> Result<i32> {
    i32::from_str(s).map_err(|err| Error::Wkt(format!("Error parsing integer: {err:?}").into()))
}
*/
