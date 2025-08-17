//!
//!  Tests
//!
use crate::builder::{Builder, Node};
use crate::model::*;

use env_logger;
use std::sync::Once;

static INIT: Once = Once::new();

pub fn setup() {
    // Init setup
    INIT.call_once(|| {
        env_logger::init();
    });
}

pub mod fixtures {
    pub const WKT_PROJCS_NAD83: &str = concat!(
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

    pub const WKT_GEOGCS_WGS84: &str = r#"
        GEOGCS["WGS 84",
            DATUM["WGS_1984",
                SPHEROID["WGS 84",6378137,298.257223563,
                    AUTHORITY["EPSG","7030"]],
                AUTHORITY["EPSG","6326"]],
            PRIMEM["Greenwich",0,
                AUTHORITY["EPSG","8901"]],
            UNIT["degree",0.0174532925199433,
                AUTHORITY["EPSG","9122"]],
            AUTHORITY["EPSG","4326"]
        ]"#;
}

#[test]
fn build_parameter() {
    setup();
    let wkt = r#"PARAMETER["latitude_of_origin",41]"#;
    let r = Builder::new().parse(wkt).unwrap();
    assert_eq!(
        r,
        Node::PARAMETER(Parameter {
            name: "latitude_of_origin",
            value: "41",
            unit: None,
            authority: None,
        })
    );
}

#[test]
fn build_ellipsoid() {
    setup();
    let wkt = r#"SPHEROID["GRS 1980",6378137,298.257222101,AUTHORITY["EPSG","7019"]]"#;
    let r = Builder::new().parse(wkt).unwrap();
    assert_eq!(
        r,
        Node::ELLIPSOID(Ellipsoid {
            name: "GRS 1980",
            a: "6378137",
            rf: "298.257222101",
            unit: None,
        })
    );
}

#[test]
fn build_authority() {
    setup();
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
fn build_datum() {
    setup();
    let wkt = concat!(
        r#"DATUM["North_American_Datum_1983",SPHEROID["GRS 1980",6378137,298.257222101,"#,
        r#"AUTHORITY["EPSG","7019"]],AUTHORITY["EPSG","6269"]]"#
    );
    let r = Builder::new().parse(wkt).unwrap();
    assert_eq!(
        r,
        Node::DATUM(Datum {
            name: "North_American_Datum_1983",
            ellipsoid: Ellipsoid {
                name: "GRS 1980",
                a: "6378137",
                rf: "298.257222101",
                unit: None,
            },
            to_wgs84: vec![],
        })
    );
}

#[test]
fn build_wgs84() {
    setup();
    let wkt = r#"TOWGS84[59.47,-5.04,187.44,0.47,-0.1,1.024,-4.5993]"#;
    let r = Builder::new().parse(wkt).unwrap();
    assert_eq!(
        r,
        Node::TOWGS84(vec![
            "59.47", "-5.04", "187.44", "0.47", "-0.1", "1.024", "-4.5993"
        ]),
    );
}

#[test]
fn parse_wgs84_wkt() {
    setup();
    let r = Builder::new()
        .parse(fixtures::WKT_GEOGCS_WGS84)
        .expect("Failed to parse WGS84 WKT");
    if let Node::GEOGCRS(geogcrs) = &r {
        assert_eq!(geogcrs.name, "WGS 84");
        assert_eq!(
            geogcrs.authority,
            Some(Authority {
                name: "EPSG",
                code: "4326",
            })
        );
    } else {
        panic!("Expected GEOGCRS node");
    }
}

#[test]
fn build_nad83() {
    setup();
    let r = Builder::new().parse(fixtures::WKT_PROJCS_NAD83).unwrap();
    println!("{r:#?}");
    assert_eq!(
        r,
        Node::PROJCRS(Projcs {
            name: "NAD83 / Massachusetts Mainland",
            geogcs: Geogcs {
                name: "NAD83",
                datum: Datum {
                    name: "North_American_Datum_1983",
                    ellipsoid: Ellipsoid {
                        name: "GRS 1980",
                        a: "6378137",
                        rf: "298.257222101",
                        unit: None,
                    },
                    to_wgs84: vec![],
                },
                unit: Some(Unit {
                    name: "degree",
                    factor: 0.01745329251994328,
                    unit_type: UnitType::Angular,
                }),
                authority: None,
            },
            projection: Projection {
                name: "Unknown",
                method: Method {
                    name: "Lambert_Conformal_Conic_2SP",
                    authority: None,
                },
                parameters: vec![
                    Parameter {
                        name: "standard_parallel_1",
                        value: "42.68333333333333",
                        unit: None,
                        authority: None,
                    },
                    Parameter {
                        name: "standard_parallel_2",
                        value: "41.71666666666667",
                        unit: None,
                        authority: None,
                    },
                    Parameter {
                        name: "latitude_of_origin",
                        value: "-41",
                        unit: None,
                        authority: None,
                    },
                    Parameter {
                        name: "central_meridian",
                        value: "-71.5",
                        unit: None,
                        authority: None,
                    },
                    Parameter {
                        name: "false_easting",
                        value: "200000",
                        unit: None,
                        authority: None,
                    },
                    Parameter {
                        name: "false_northing",
                        value: "750000",
                        unit: None,
                        authority: None,
                    }
                ],
                authority: Some(Authority {
                    name: "EPSG",
                    code: "26986",
                }),
            },
            unit: Some(Unit {
                name: "metre",
                factor: 1.0,
                unit_type: UnitType::Linear,
            }),
        }),
    );
}
