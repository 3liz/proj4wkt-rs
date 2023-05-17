//!
//! Build proj string
//!
//! ## Specifications
//!
//! * WKT CRS standards: https://www.ogc.org/standard/wkt-crs/
//! * WKT2015 specs: https://docs.ogc.org/is/12-063r5/12-063r5.html
//! * WKT2019 specs: https://docs.ogc.org/is/18-010r7/18-010r7.html
//!
//!
use crate::errors::{Error, Result};
use crate::model::*;
use crate::parser::{parse, Attribute, Processor};

#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum Key {
    GEOGCS,
    GEOCCS,
    PROJCS,
    CONVERSION,
    METHOD,
    VERTICALCRS,
    LOCAL_CS,
    TIMECRS,
    COMPOUNDCRS,
    FITTED_CS,
    DATUM,
    VERT_DATUM,
    LOCAL_DATUM,
    ELLIPSOID,
    PRIMEM,
    PROJECTION,
    PARAMETER,
    AXIS,
    UNIT,
    AUTHORITY,
    BOUNDCRS,
    TOWGS84,
    OTHER,
}

// See https://docs.ogc.org/is/18-010r7/18-010r7.html
impl From<&str> for Key {
    fn from(key: &str) -> Self {
        match key {
            // Geodetic CRS
            "GEOGCS" | "GEOGCRS" | "GEOGRAPHICCRS" | "BASEGEODCRS" | "BASEGEOGCRS" => Self::GEOGCS,
            // Geodetic CRS with geocentric Cartesian coordinate system
            "GEOCCS" | "GEODCRS" | "GEODETICCRS" => Self::GEOCCS,
            // Projected CRS
            "PROJCS" | "PROJCRS" | "PROJECTEDCRS" => Self::PROJCS,
            // Datum - geodetic reference frame
            "DATUM" | "GEODETICDATUM" | "TRF" => Self::DATUM,
            // Ellipsoid
            "ELLIPSOID" | "SPHEROID" => Self::ELLIPSOID,
            // Prime meridian
            "PRIMEM" | "PRIMEMERIDIAN" => Self::PRIMEM,
            // Map projection method
            "PROJECTION" | "METHOD" => Self::METHOD,
            // Map projection
            "CONVERSION" => Self::CONVERSION,
            "PARAMETER" => Self::PARAMETER,
            "AXIS" => Self::AXIS,
            "UNIT" | "LENGTHUNIT" | "ANGLEUNIT" | "SCALUNIT" => Self::UNIT,
            "AUTHORITY" | "ID" => Self::AUTHORITY,
            // To wgs84 factors
            "TOWGS84" => Self::TOWGS84,
            "COMPD_CS" | "COMPOUNDCRS" => Self::COMPOUNDCRS,
            "VERT_CS" | "VERTCRS" | "VERTICALCRS" => Self::VERTICALCRS,
            /*
            "BOUNDCRS" => Self::BOUNDCRS,
            "LOCAL_CS" | "ENGCRS" | "ENGINEERINGCRS" => Self::LOCAL_CS,
            "TIMECRS" => Self::TIMECRS,
            "FITTED_CS" => Self::FITTED_CS,
            "VERT_DATUM" | "VDATUM" | "VERTICALDATUM" | "VRF" => Self::VERT_DATUM,
            "LOCAL_DATUM" | "EDATUM" | "ENGINEERINGDATUM" => Self::LOCAL_DATUM,
            */
            // Same concept as TOWGS84
            //"ABRIDGEDTRANSFORMATION" => (),
            _ => Self::OTHER,
        }
    }
}
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq)]
pub enum Node<'a> {
    AUTHORITY(Authority<'a>),
    UNIT(Unit<'a>),
    METHOD(Method<'a>),
    PARAMETER(Parameter<'a>),
    DATUM(Datum<'a>),
    PROJCS(Projcs<'a>),
    GEOGCS(Geogcs<'a>),
    PROJECTION(Projection<'a>),
    ELLIPSOID(Ellipsoid<'a>),
    COMPOUNDCRS(Compoundcrs<'a>),
    VERTICALCRS(Verticalcrs<'a>),
    TOWGS84(Vec<&'a str>),
    OTHER(&'a str),
}

#[derive(Debug, Default)]
pub struct Builder;

impl Builder {
    pub fn new() -> Self {
        Builder {}
    }

    pub fn parse<'a>(&self, s: &'a str) -> Result<Node<'a>> {
        parse(s, self)
    }
}

impl<'a> Processor<'a> for Builder {
    type Err = Error;
    type Output = Node<'a>;

    fn process<I>(&self, key: &'a str, depth: usize, attrs: I) -> Result<Self::Output, Self::Err>
    where
        I: Iterator<Item = Attribute<'a, Self::Output>>,
    {
        match key {
            "AUTHORITY" | "ID" => self.authority(attrs).map(Node::AUTHORITY),
            "PROJCS" | "PROJCRS" | "PROJECTEDCRS" => self.projcs(attrs).map(Node::PROJCS),
            "GEOGCS" | "GEOGCRS" | "GEOGRAPHICCRS" | "BASEGEODCRS" | "BASEGEOGCRS" => {
                self.geogcs(attrs).map(Node::GEOGCS)
            }
            "ELLIPSOID" | "SPHEROID" => self.ellipsoid(attrs).map(Node::ELLIPSOID),
            "CONVERSION" => self.projection(attrs).map(Node::PROJECTION),
            "PROJECTION" | "METHOD" => self.method(attrs).map(Node::METHOD),
            "PARAMETER" => self.parameter(attrs).map(Node::PARAMETER),
            "DATUM" | "GEODETICDATUM" | "TRF" => self.datum(attrs).map(Node::DATUM),
            "UNIT" => self.unit(key, attrs).map(Node::UNIT),
            "COMPD_CS" | "COMPOUNDCRS" => self.compoundcrs(attrs).map(Node::COMPOUNDCRS),
            "VERT_CS" | "VERTCRS" | "VERTICALCRS" => self.verticalcrs(attrs).map(Node::VERTICALCRS),
            "TOWGS84" => self.towgs84(attrs).map(Node::TOWGS84),
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
        let mut authority = None;

        let mut parameters: Vec<Parameter<'a>> = vec![];

        for (i, a) in attrs.enumerate() {
            match a {
                Attribute::Quoted(s) if i == 0 => name = Some(s),
                Attribute::Keyword(_, n) => match n {
                    Node::GEOGCS(cs) => geogcs = Some(cs),
                    Node::PROJECTION(p) => projection = Some(p),
                    // Handle WKT1
                    Node::AUTHORITY(auth) => authority = Some(auth),
                    Node::UNIT(u) => unit = Some(u),
                    Node::METHOD(m) => method = Some(m),
                    Node::PARAMETER(p) => parameters.push(p),
                    _ => (),
                },
                _ => (),
            }
        }

        // On pre WKT2 parameters for projection are at the root level
        if projection.is_none() {
            let me = method.ok_or(Error::WktError("No projection method defined".into()))?;
            projection = Some(Projection {
                name: "Unknown",
                method: me,
                parameters,
                authority,
            });
        }

        Ok(Projcs {
            name: name.unwrap_or("Unknown"),
            geogcs: geogcs.ok_or(Error::WktError("Missing PROJCS GEOGCS".into()))?,
            projection: projection.ok_or(Error::WktError("Missing PROJCS projection".into()))?,
            unit,
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
            method: method.ok_or(Error::WktError(
                "Missing METHOD in projection definition".into(),
            ))?,
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
            name: name.ok_or(Error::WktError("Missing METHOD or PROJECTION name".into()))?,
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
            name: name.ok_or(Error::WktError("Missing PARAMETER name".into()))?,
            value: value.ok_or(Error::WktError("Missing PARAMETER value".into()))?,
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
        let mut unit = None;

        for (i, a) in attrs.enumerate() {
            match a {
                Attribute::Quoted(s) if i == 0 => name = Some(s),
                Attribute::Keyword(_, n) => match n {
                    Node::DATUM(d) => datum = Some(d),
                    Node::UNIT(u) => unit = Some(u),
                    _ => (),
                },
                _ => (),
            }
        }

        Ok(Geogcs {
            name: name.unwrap_or(""),
            datum: datum.ok_or(Error::WktError("Missing DATUM for Geodetic CRS".into()))?,
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
            ellipsoid: ellipsoid.ok_or(Error::WktError("Missing ellipsoid for DATUM".into()))?,
            to_wgs84,
        })
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
            name: name.ok_or(Error::WktError("Missing AUTHORITY name".into()))?,
            code: code.ok_or(Error::WktError("Missing AUTHORITY code".into()))?,
        })
    }

    fn unit<'a>(
        &self,
        key: &'a str,
        attrs: impl Iterator<Item = Attribute<'a, Node<'a>>>,
    ) -> Result<Unit<'a>> {
        let mut name = None;
        let mut factor = None;
        let mut authority = None;

        for (i, a) in attrs.enumerate() {
            match a {
                Attribute::Quoted(s) if i == 0 => name = Some(s),
                Attribute::Number(s) if i == 1 => factor = Some(parse_number(s)?),
                Attribute::Keyword(_, Node::AUTHORITY(auth)) => authority = Some(auth),
                _ => (),
            }
        }

        Ok(Unit {
            name: name.ok_or(Error::WktError("Missing UNIT name".into()))?,
            factor: factor.ok_or(Error::WktError("Missing UNIT factor".into()))?,
            unit_type: match key {
                "ANGLEUNIT" => UnitType::Angular,
                "SCALUNIT" => UnitType::Scale,
                "LENGTHUNIT" => UnitType::Linear,
                _ => UnitType::Unknown,
            },
            authority,
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
                    Node::PROJCS(cs) => h_crs = Some(Horizontalcrs::Projcs(cs)),
                    Node::GEOGCS(cs) => h_crs = Some(Horizontalcrs::Geogcs(cs)),
                    Node::VERTICALCRS(cs) => v_crs = Some(cs),
                    _ => (),
                },
                _ => (),
            }
        }

        Ok(Compoundcrs {
            name: name.ok_or(Error::WktError("Missing Compound Crs name".into()))?,
            h_crs: h_crs.ok_or(Error::WktError(
                "Missing Horzontal CRS for compound crs name".into(),
            ))?,
            v_crs: v_crs.ok_or(Error::WktError("Missing Vertical crs for compound".into()))?,
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

        Ok(Ellipsoid {
            name: name.ok_or(Error::WktError("Missing AUTHORITY name".into()))?,
            a: semi_major.ok_or(Error::WktError("Invalid ELLIPSOID semi-major axis".into()))?,
            rf: rf.ok_or(Error::WktError(
                "Invalid ELLIPSOID inverse flattening".into(),
            ))?,
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
                    return Err(Error::WktError(format!("Expecting number not {a:?}")));
                }
            }
        }

        if !matches!(to_wgs84.len(), 0 | 3 | 7) {
            return Err(Error::WktError(
                "Wrong number of parameters for TOWGS84".into(),
            ));
        }

        Ok(to_wgs84)
    }
}

use crate::parse::FromStr;

pub fn parse_number(s: &str) -> Result<f64> {
    f64::from_str(s).map_err(|err| Error::WktError(format!("Error parsing number: {err:?}")))
}

/*
pub fn parse_int(s: &str) -> Result<i32> {
    i32::from_str(s).map_err(|err| Error::WktError(format!("Error parsing integer: {err:?}")))
}
*/
