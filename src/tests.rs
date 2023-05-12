//!
//!  Tests
//!
use crate::builder::{Builder, Node};
use crate::errors::{Error, Result};
use crate::model::*;

const WKT_PROJCS_NAD83: &str = concat!(
    r#"PROJCS["NAD83 / Massachusetts Mainland",GEOGCS["NAD83","#,
    r#"DATUM["North_American_Datum_1983",SPHEROID["GRS 1980",6378137,298.257222101,"#,
    r#"AUTHORITY["EPSG","7019"]],AUTHORITY["EPSG","6269"]],PRIMEM["Greenwich",0,"#,
    r#"AUTHORITY["EPSG","8901"]],UNIT["degree",0.01745329251994328,"#,
    r#"AUTHORITY["EPSG","9122"]],AUTHORITY["EPSG","4269"]],UNIT["metre",1,"#,
    r#"AUTHORITY["EPSG","9001"]],PROJECTION["Lambert_Conformal_Conic_2SP"],"#,
    r#"PARAMETER["standard_parallel_1",42.68333333333333],"#,
    r#"PARAMETER["standard_parallel_2",41.71666666666667],"#,
    r#"PARAMETER["latitude_of_origin", -41],PARAMETER["central_meridian",-71.5],"#,
    r#"PARAMETER["false_easting",200000],PARAMETER["false_northing",750000],"#,
    r#"AUTHORITY["EPSG","26986"],AXIS["X",EAST],AXIS["Y",NORTH]]"#,
);

#[test]
fn build_parameter() {
    let wkt = r#"PARAMETER["latitude_of_origin",41]"#;
    let r = Builder::new().parse(wkt).unwrap();
    assert_eq!(
        r,
        Node::PARAMETER(Parameter {
            name: "latitude_of_origin",
            value: 41.0,
            unit: None,
        })
    );
}

#[test]
fn build_authority() {
    let wkt = r#"AUTHORITY["EPSG","26986"]"#;
    let r = Builder::new().parse(wkt).unwrap();
    assert_eq!(
        r,
        Node::AUTHORITY(Authority {
            name: "EPSG",
            code: "26986",
        })
    );
}

#[test]
fn build_unit() {
    let wkt = r#"UNIT["degree",0.01745329251994328,AUTHORITY["EPSG","9122"]]"#;
    let r = Builder::new().parse(wkt).unwrap();
    assert_eq!(
        r,
        Node::UNIT(Unit {
            name: "degree",
            factor: 0.01745329251994328,
            unit_type: UnitType::Unknown,
        })
    );
}

#[test]
fn build_nad83() {
    let r = Builder::new().parse(WKT_PROJCS_NAD83).unwrap();
    assert_eq!(
        r,
        Node::PROJCS(Projcs {
            name: "NAD83 / Massachusetts Mainland",
            geogcs: Geogcs {
                name: "NAD83",
                datum: Some(Datum {
                    name: "North_American_Datum_1983",
                    ellps: None,
                    to_wgs84: vec![]
                })
            },
            projection: Projection {
                name: "Lambert_Conformal_Conic_2SP",
                method: Some(Method {
                    name: "Lambert_Conformal_Conic_2SP",
                    authority: Some(Authority {
                        name: "EPSG",
                        code: "26986",
                    }),
                }),
                parameters: vec![
                    Parameter {
                        name: "standard_parallel_1",
                        value: 42.68333333333333,
                        unit: None
                    },
                    Parameter {
                        name: "standard_parallel_2",
                        value: 41.71666666666667,
                        unit: None
                    },
                    Parameter {
                        name: "latitude_of_origin",
                        value: -41.0,
                        unit: None
                    },
                    Parameter {
                        name: "central_meridian",
                        value: -71.5,
                        unit: None
                    },
                    Parameter {
                        name: "false_easting",
                        value: 200000.0,
                        unit: None
                    },
                    Parameter {
                        name: "false_northing",
                        value: 750000.0,
                        unit: None
                    }
                ],
                unit: Some(Unit {
                    name: "metre",
                    factor: 1.0,
                    unit_type: UnitType::Unknown
                })
            }
        }),
    );
}
