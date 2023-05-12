//!
//! Projection representation model
//!

#[derive(Debug, PartialEq)]
pub struct Projcs<'a> {
    pub name: &'a str,
    pub geogcs: Geogcs<'a>,
    pub projection: Projection<'a>,
}

#[derive(Debug, PartialEq)]
pub struct Datum<'a> {
    pub name: &'a str,
    pub ellps: Option<Spheroid<'a>>,
    pub to_wgs84: Vec<f64>,
}

#[derive(Debug, PartialEq)]
pub struct Geogcs<'a> {
    pub name: &'a str,
    pub datum: Option<Datum<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Parameter<'a> {
    pub name: &'a str,
    pub value: f64,
    pub unit: Option<Unit<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Projection<'a> {
    pub name: &'a str,
    pub method: Option<Method<'a>>,
    pub parameters: Vec<Parameter<'a>>,
    pub unit: Option<Unit<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Spheroid<'a> {
    pub name: &'a str,
    pub a: Option<f64>,
    pub rf: Option<f64>,
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
    Angle,
    Length,
    Scale,
    Unknown,
}

#[derive(Debug, PartialEq)]
pub struct Unit<'a> {
    pub name: &'a str,
    pub factor: f64,
    pub unit_type: UnitType,
}
