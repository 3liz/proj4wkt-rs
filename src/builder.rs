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
pub enum Key {
    GEOGCS,
    GEOCCS,
    PROJCS,
    CONVERSION,
    METHOD,
    VERT_CS,
    LOCAL_CS,
    TIMECRS,
    COMPD_CS,
    FITTED_CS,
    DATUM,
    VERT_DATUM,
    LOCAL_DATUM,
    SPHEROID,
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
            "ELLIPSOID" | "SPHEROID" => Self::SPHEROID,
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
            /*
            "BOUNDCRS" => Self::BOUNDCRS,
            "VERT_CS" | "VERTCRS" | "VERTICALCRS" => Self::VERT_CS,
            "LOCAL_CS" | "ENGCRS" | "ENGINEERINGCRS" => Self::LOCAL_CS,
            "TIMECRS" => Self::TIMECRS,
            "COMPD_CS" | "COMPOUNDCRS" => Self::COMPD_CS,
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
    SPHEROID(Spheroid<'a>),
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
        match Key::from(key) {
            Key::PROJCS => self.projcs(attrs).map(Node::PROJCS),
            Key::GEOGCS => self.geogcs(attrs).map(Node::GEOGCS),
            Key::CONVERSION => self.projection(attrs).map(Node::PROJECTION),
            Key::METHOD => self.method(attrs).map(Node::METHOD),
            Key::PARAMETER => self.parameter(attrs).map(Node::PARAMETER),
            Key::AUTHORITY => self.authority(attrs).map(Node::AUTHORITY),
            Key::DATUM => self.datum(attrs).map(Node::DATUM),
            Key::UNIT => self.unit(key, attrs).map(Node::UNIT),
            _ => {
                // Consumme tokens
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
                    Node::METHOD(m) => method = Some(m),
                    Node::PARAMETER(p) => parameters.push(p),
                    Node::UNIT(u) => unit = Some(u),
                    Node::AUTHORITY(auth) => authority = Some(auth),
                    _ => (),
                },
                _ => (),
            }
        }

        // On pre WKT2015 parameters for projection are at the root level
        if projection.is_none() {
            if let Some(me) = method.as_mut() {
                let name = me.name;
                if me.authority.is_none() {
                    me.authority = authority;
                }
                projection = Some(Projection {
                    name,
                    method,
                    parameters,
                    unit,
                });
            }
        }

        Ok(Projcs {
            name: name.ok_or(Error::WktError("Missing PROJCS name".into()))?,
            geogcs: geogcs.ok_or(Error::WktError("Missing PROJCS GEOGCS".into()))?,
            projection: projection
                .ok_or(Error::WktError("Missing PROJCS projection".into()))?,
        })
    }

    fn geogcs<'a>(
        &self,
        attrs: impl Iterator<Item = Attribute<'a, Node<'a>>>,
    ) -> Result<Geogcs<'a>> {
        let mut name = None;
        let mut datum = None;

        for (i, a) in attrs.enumerate() {
            match a {
                Attribute::Quoted(s) if i == 0 => name = Some(s),
                Attribute::Keyword(_, n) => match n {
                    Node::DATUM(d) => datum = Some(d),
                    _ => (),
                },
                _ => (),
            }
        }

        Ok(Geogcs {
            name: name.unwrap_or(""),
            datum,
        })
    }

    fn datum<'a>(&self, attrs: impl Iterator<Item = Attribute<'a, Node<'a>>>) -> Result<Datum<'a>> {
        let mut name = None;

        for (i, a) in attrs.enumerate() {
            match a {
                Attribute::Quoted(s) if i == 0 => name = Some(s),
                Attribute::Keyword(_, n) => match n {
                    _ => (),
                },
                _ => (),
            }
        }

        Ok(Datum {
            name: name.ok_or(Error::WktError("Missing DATUM name".into()))?,
            ellps: None,
            to_wgs84: vec![],
        })
    }

    fn projection<'a>(
        &self,
        attrs: impl Iterator<Item = Attribute<'a, Node<'a>>>,
    ) -> Result<Projection<'a>> {
        let mut name = None;
        let mut method = None;
        let mut parameters: Vec<Parameter<'a>> = vec![];
        let mut unit = None;

        for (i, a) in attrs.enumerate() {
            match a {
                Attribute::Quoted(s) if i == 0 => name = Some(s),
                Attribute::Keyword(_, n) => match n {
                    Node::METHOD(m) => method = Some(m),
                    Node::UNIT(u) => unit = Some(u),
                    Node::PARAMETER(p) => parameters.push(p),
                    _ => (),
                },
                _ => (),
            }
        }

        Ok(Projection {
            name: name.unwrap_or(""),
            method,
            parameters,
            unit,
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
                Attribute::Keyword(_, n) => match n {
                    Node::AUTHORITY(auth) => authority = Some(auth),
                    _ => (),
                },
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

        for (i, a) in attrs.enumerate() {
            match a {
                Attribute::Quoted(s) if i == 0 => name = Some(s),
                Attribute::Number(s) if i == 1 => value = Some(parse_number(s)?),
                Attribute::Keyword(_, n) => match n {
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

        for (i, a) in attrs.enumerate() {
            match a {
                Attribute::Quoted(s) if i == 0 => name = Some(s),
                Attribute::Number(s) if i == 1 => factor = Some(parse_number(s)?),
                _ => (),
            }
        }

        Ok(Unit {
            name: name.ok_or(Error::WktError("Missing UNIT name".into()))?,
            factor: factor.ok_or(Error::WktError("Missing UNIT factor".into()))?,
            unit_type: match key {
                "ANGLEUNIT" => UnitType::Length,
                "SCALUNIT" => UnitType::Scale,
                "LENGTHUNIT" => UnitType::Length,
                _ => UnitType::Unknown,
            },
        })
    }
}

use crate::parse::FromStr;

pub fn parse_number(s: &str) -> Result<f64> {
    f64::from_str(s).map_err(|err| Error::WktError(format!("Error parsing number: {err:?}")))
}

pub fn parse_int(s: &str) -> Result<i32> {
    i32::from_str(s).map_err(|err| Error::WktError(format!("Error parsing integer: {err:?}")))
}
