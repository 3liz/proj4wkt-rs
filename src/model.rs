//!
//! Projection representation model
//!
#[derive(Debug, PartialEq)]
pub struct Geogcs<'a> {
    pub name: &'a str,
    pub datum: Datum<'a>,
    pub unit: Option<Unit<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Datum<'a> {
    pub name: &'a str,
    pub ellipsoid: Ellipsoid<'a>,
    pub to_wgs84: Vec<&'a str>,
}

#[derive(Debug, PartialEq)]
pub struct Ellipsoid<'a> {
    pub name: &'a str,
    pub a: &'a str,
    pub rf: &'a str,
    pub unit: Option<Unit<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Projcs<'a> {
    pub name: &'a str,
    pub geogcs: Geogcs<'a>,
    pub projection: Projection<'a>,
    pub unit: Option<Unit<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Projection<'a> {
    pub name: &'a str,
    pub method: Method<'a>,
    pub parameters: Vec<Parameter<'a>>,
    pub authority: Option<Authority<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Parameter<'a> {
    pub name: &'a str,
    pub value: &'a str,
    pub unit: Option<Unit<'a>>,
    pub authority: Option<Authority<'a>>,
}

// WKT 2015/2019
#[derive(Debug, PartialEq)]
pub struct Method<'a> {
    pub name: &'a str,
    pub authority: Option<Authority<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Authority<'a> {
    pub name: &'a str,
    pub code: &'a str,
}

#[derive(Debug, PartialEq)]
pub enum UnitType {
    Angular,
    Linear,
    Scale,
    Unknown,
}

/// See https://epsg.io/?q=foot%20kind%3AUNIT
/// for units EPSG definition
#[derive(Debug, PartialEq)]
pub struct Unit<'a> {
    pub name: &'a str,
    pub factor: f64,
    pub unit_type: UnitType,
}

// see https://docs.ogc.org/is/18-010r7/18-010r7.html#125
#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum Horizontalcrs<'a> {
    Projcs(Projcs<'a>),
    Geogcs(Geogcs<'a>),
}

// TODO
#[derive(Debug, PartialEq)]
pub struct Verticalcrs<'a> {
    pub name: &'a str,
}

#[derive(Debug, PartialEq)]
pub struct Compoundcrs<'a> {
    pub name: &'a str,
    pub h_crs: Horizontalcrs<'a>,
    pub v_crs: Verticalcrs<'a>,
}
