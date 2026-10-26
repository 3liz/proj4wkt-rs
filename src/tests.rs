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
fn build_wkt2_units() {
    setup();
    for (wkt, name, factor, unit_type) in [
        (
            r#"ANGLEUNIT["degree",0.0174532925199433]"#,
            "degree",
            0.0174532925199433,
            UnitType::Angular,
        ),
        (
            r#"LENGTHUNIT["US survey foot",0.304800609601219]"#,
            "US survey foot",
            0.304800609601219,
            UnitType::Linear,
        ),
        (r#"SCALEUNIT["unity",1]"#, "unity", 1.0, UnitType::Scale),
    ] {
        let r = Builder::new().parse(wkt).unwrap();
        assert_eq!(
            r,
            Node::UNIT(Unit {
                name,
                factor,
                unit_type,
            }),
            "{wkt}"
        );
    }
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
                authority: Some(Authority {
                    name: "EPSG",
                    code: "4269",
                }),
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

#[test]
fn build_wkt2_projcs_axis_unit() {
    setup();
    let unit = Unit {
        name: "US survey foot",
        factor: 0.304800609601219,
        unit_type: UnitType::Linear,
    };
    let crs = concat!(
        r#"PROJCRS["NAD83 / New York Long Island (ftUS)","#,
        r#"BASEGEOGCRS["NAD83",DATUM["North American Datum 1983","#,
        r#"ELLIPSOID["GRS 1980",6378137,298.257222101,LENGTHUNIT["metre",1]]],"#,
        r#"ANGLEUNIT["degree",0.0174532925199433]],"#,
        r#"CONVERSION["SPCS83 New York Long Island zone (US Survey feet)","#,
        r#"METHOD["Lambert Conic Conformal (2SP)",ID["EPSG",9802]],"#,
        r#"PARAMETER["Easting at false origin",984250,"#,
        r#"LENGTHUNIT["US survey foot",0.304800609601219],ID["EPSG",8826]]],"#,
    );
    // Unit defined on each axis
    let per_axis = concat!(
        r#"CS[Cartesian,2],"#,
        r#"AXIS["easting (X)",east,ORDER[1],LENGTHUNIT["US survey foot",0.304800609601219]],"#,
        r#"AXIS["northing (Y)",north,ORDER[2],LENGTHUNIT["US survey foot",0.304800609601219]]]"#,
    );
    // Unit common to all axes
    let common = concat!(
        r#"CS[Cartesian,2],AXIS["easting (X)",east],AXIS["northing (Y)",north],"#,
        r#"LENGTHUNIT["US survey foot",0.304800609601219]]"#,
    );

    for cs in [per_axis, common] {
        let wkt = format!("{crs}{cs}");
        match Builder::new().parse(&wkt).unwrap() {
            Node::PROJCRS(projcs) => assert_eq!(projcs.unit.as_ref(), Some(&unit), "{cs}"),
            other => panic!("Expected PROJCRS node, got {other:?}"),
        }

        let projstr = crate::wkt_to_projstring(&wkt).unwrap();
        assert!(
            projstr.contains(" +to_meter=0.304800609601219"),
            "{projstr}"
        );
    }
}

#[test]
fn build_wkt2_geogcs_axis_unit() {
    setup();
    let unit = Unit {
        name: "grad",
        factor: 0.0157079632679489,
        unit_type: UnitType::Angular,
    };
    let crs = concat!(
        r#"GEOGCRS["NTF (Paris)",DATUM["Nouvelle Triangulation Francaise (Paris)","#,
        r#"ELLIPSOID["Clarke 1880 (IGN)",6378249.2,293.466021293627,LENGTHUNIT["metre",1]]],"#,
        r#"PRIMEM["Paris",2.5969213,ANGLEUNIT["grad",0.0157079632679489]],"#,
    );
    // Unit defined on each axis, with an ellipsoidal height axis
    let per_axis = concat!(
        r#"CS[ellipsoidal,3],"#,
        r#"AXIS["geodetic latitude (Lat)",north,ORDER[1],ANGLEUNIT["grad",0.0157079632679489]],"#,
        r#"AXIS["geodetic longitude (Lon)",east,ORDER[2],ANGLEUNIT["grad",0.0157079632679489]],"#,
        r#"AXIS["ellipsoidal height (h)",up,ORDER[3],LENGTHUNIT["metre",1]]]"#,
    );
    // Unit common to all axes
    let common = concat!(
        r#"CS[ellipsoidal,2],AXIS["latitude",north],AXIS["longitude",east],"#,
        r#"ANGLEUNIT["grad",0.0157079632679489]]"#,
    );

    for cs in [per_axis, common] {
        let wkt = format!("{crs}{cs}");
        match Builder::new().parse(&wkt).unwrap() {
            Node::GEOGCRS(geogcs) => assert_eq!(geogcs.unit.as_ref(), Some(&unit), "{cs}"),
            other => panic!("Expected GEOGCRS node, got {other:?}"),
        }
    }
}
