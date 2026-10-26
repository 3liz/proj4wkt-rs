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

    // EPSG:4326, WKT2 2019 with datum ensemble (projinfo, PROJ 9.4.0)
    pub const WKT2_2019_GEOGCRS_WGS84: &str = r#"
        GEOGCRS["WGS 84",
            ENSEMBLE["World Geodetic System 1984 ensemble",
                MEMBER["World Geodetic System 1984 (Transit)"],
                MEMBER["World Geodetic System 1984 (G730)"],
                MEMBER["World Geodetic System 1984 (G873)"],
                MEMBER["World Geodetic System 1984 (G1150)"],
                MEMBER["World Geodetic System 1984 (G1674)"],
                MEMBER["World Geodetic System 1984 (G1762)"],
                MEMBER["World Geodetic System 1984 (G2139)"],
                ELLIPSOID["WGS 84",6378137,298.257223563,
                    LENGTHUNIT["metre",1]],
                ENSEMBLEACCURACY[2.0]],
            PRIMEM["Greenwich",0,
                ANGLEUNIT["degree",0.0174532925199433]],
            CS[ellipsoidal,2],
                AXIS["geodetic latitude (Lat)",north,
                    ORDER[1],
                    ANGLEUNIT["degree",0.0174532925199433]],
                AXIS["geodetic longitude (Lon)",east,
                    ORDER[2],
                    ANGLEUNIT["degree",0.0174532925199433]],
            USAGE[
                SCOPE["Horizontal component of 3D system."],
                AREA["World."],
                BBOX[-90,-180,90,180]],
            ID["EPSG",4326]]"#;

    // EPSG:32631, WKT2 2019 (projinfo, PROJ 9.4.0)
    pub const WKT2_2019_PROJCRS_UTM31N: &str = r#"
        PROJCRS["WGS 84 / UTM zone 31N",
            BASEGEOGCRS["WGS 84",
                ENSEMBLE["World Geodetic System 1984 ensemble",
                    MEMBER["World Geodetic System 1984 (Transit)"],
                    MEMBER["World Geodetic System 1984 (G730)"],
                    MEMBER["World Geodetic System 1984 (G873)"],
                    MEMBER["World Geodetic System 1984 (G1150)"],
                    MEMBER["World Geodetic System 1984 (G1674)"],
                    MEMBER["World Geodetic System 1984 (G1762)"],
                    MEMBER["World Geodetic System 1984 (G2139)"],
                    ELLIPSOID["WGS 84",6378137,298.257223563,
                        LENGTHUNIT["metre",1]],
                    ENSEMBLEACCURACY[2.0]],
                PRIMEM["Greenwich",0,
                    ANGLEUNIT["degree",0.0174532925199433]],
                ID["EPSG",4326]],
            CONVERSION["UTM zone 31N",
                METHOD["Transverse Mercator",
                    ID["EPSG",9807]],
                PARAMETER["Latitude of natural origin",0,
                    ANGLEUNIT["degree",0.0174532925199433],
                    ID["EPSG",8801]],
                PARAMETER["Longitude of natural origin",3,
                    ANGLEUNIT["degree",0.0174532925199433],
                    ID["EPSG",8802]],
                PARAMETER["Scale factor at natural origin",0.9996,
                    SCALEUNIT["unity",1],
                    ID["EPSG",8805]],
                PARAMETER["False easting",500000,
                    LENGTHUNIT["metre",1],
                    ID["EPSG",8806]],
                PARAMETER["False northing",0,
                    LENGTHUNIT["metre",1],
                    ID["EPSG",8807]]],
            CS[Cartesian,2],
                AXIS["(E)",east,
                    ORDER[1],
                    LENGTHUNIT["metre",1]],
                AXIS["(N)",north,
                    ORDER[2],
                    LENGTHUNIT["metre",1]],
            USAGE[
                SCOPE["Navigation and medium accuracy spatial referencing."],
                AREA["Between 0°E and 6°E, northern hemisphere between equator and 84°N, onshore and offshore. Algeria. Andorra. Belgium. Benin. Burkina Faso. Denmark - North Sea. France. Germany - North Sea. Ghana. Luxembourg. Mali. Netherlands. Niger. Nigeria. Norway. Spain. Togo. United Kingdom (UK) - North Sea."],
                BBOX[0,0,84,6]],
            ID["EPSG",32631]]"#;

    // EPSG:4326, WKT2 2015 (projinfo, PROJ 9.4.0)
    pub const WKT2_2015_GEODCRS_WGS84: &str = r#"
        GEODCRS["WGS 84",
            DATUM["World Geodetic System 1984",
                ELLIPSOID["WGS 84",6378137,298.257223563,
                    LENGTHUNIT["metre",1]]],
            PRIMEM["Greenwich",0,
                ANGLEUNIT["degree",0.0174532925199433]],
            CS[ellipsoidal,2],
                AXIS["geodetic latitude (Lat)",north,
                    ORDER[1],
                    ANGLEUNIT["degree",0.0174532925199433]],
                AXIS["geodetic longitude (Lon)",east,
                    ORDER[2],
                    ANGLEUNIT["degree",0.0174532925199433]],
            SCOPE["Horizontal component of 3D system."],
            AREA["World."],
            BBOX[-90,-180,90,180],
            ID["EPSG",4326]]"#;

    // EPSG:4978, geocentric WKT2 2019 (projinfo, PROJ 9.4.0)
    pub const WKT2_2019_GEODCRS_GEOCENTRIC: &str = r#"
        GEODCRS["WGS 84",
            ENSEMBLE["World Geodetic System 1984 ensemble",
                MEMBER["World Geodetic System 1984 (Transit)"],
                MEMBER["World Geodetic System 1984 (G730)"],
                MEMBER["World Geodetic System 1984 (G873)"],
                MEMBER["World Geodetic System 1984 (G1150)"],
                MEMBER["World Geodetic System 1984 (G1674)"],
                MEMBER["World Geodetic System 1984 (G1762)"],
                MEMBER["World Geodetic System 1984 (G2139)"],
                ELLIPSOID["WGS 84",6378137,298.257223563,
                    LENGTHUNIT["metre",1]],
                ENSEMBLEACCURACY[2.0]],
            PRIMEM["Greenwich",0,
                ANGLEUNIT["degree",0.0174532925199433]],
            CS[Cartesian,3],
                AXIS["(X)",geocentricX,
                    ORDER[1],
                    LENGTHUNIT["metre",1]],
                AXIS["(Y)",geocentricY,
                    ORDER[2],
                    LENGTHUNIT["metre",1]],
                AXIS["(Z)",geocentricZ,
                    ORDER[3],
                    LENGTHUNIT["metre",1]],
            USAGE[
                SCOPE["Geodesy. Navigation and positioning using GPS satellite system."],
                AREA["World."],
                BBOX[-90,-180,90,180]],
            ID["EPSG",4978]]"#;
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
            authority: Some(Authority {
                name: "EPSG",
                code: "9122",
            }),
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
                authority: None,
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
                prime_meridian: Some(PrimeMeridian {
                    name: "Greenwich",
                    longitude: "0",
                    unit: None,
                }),
                unit: Some(Unit {
                    name: "degree",
                    factor: 0.01745329251994328,
                    unit_type: UnitType::Angular,
                    authority: Some(Authority {
                        name: "EPSG",
                        code: "9122",
                    }),
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
                authority: Some(Authority {
                    name: "EPSG",
                    code: "9001",
                }),
            }),
            axis_directions: vec!["EAST", "NORTH"],
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
        authority: None,
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
        authority: None,
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

#[test]
fn skip_parameters_without_proj_name() {
    setup();
    // WKT1 Krovak: 'pseudo_standard_parallel_1' has no proj equivalent
    let krovak = concat!(
        r#"PROJCS["S-JTSK / Krovak East North",GEOGCS["S-JTSK",DATUM["S_JTSK","#,
        r#"SPHEROID["Bessel 1841",6377397.155,299.1528128]],PRIMEM["Greenwich",0],"#,
        r#"UNIT["degree",0.0174532925199433]],PROJECTION["Krovak"],"#,
        r#"PARAMETER["latitude_of_center",49.5],PARAMETER["longitude_of_center",24.83333333333333],"#,
        r#"PARAMETER["azimuth",30.28813972222222],PARAMETER["pseudo_standard_parallel_1",78.5],"#,
        r#"PARAMETER["scale_factor",0.9999],PARAMETER["false_easting",0],"#,
        r#"PARAMETER["false_northing",0],UNIT["metre",1]]"#,
    );
    assert_eq!(
        crate::wkt_to_projstring(krovak).unwrap(),
        concat!(
            "+proj=krovak +lat_0=49.5 +lon_0=24.83333333333333 +alpha=30.28813972222222",
            " +k=0.9999 +x_0=0 +y_0=0 +units=m +a=6377397.155 +rf=299.1528128",
        )
    );

    // WKT2 Mercator (variant A): 'Latitude of natural origin' is not exported
    let merc = concat!(
        r#"PROJCRS["WGS 84 / World Mercator",BASEGEOGCRS["WGS 84",DATUM["World Geodetic System 1984","#,
        r#"ELLIPSOID["WGS 84",6378137,298.257223563]]],CONVERSION["World Mercator","#,
        r#"METHOD["Mercator (variant A)",ID["EPSG",9804]],"#,
        r#"PARAMETER["Latitude of natural origin",0],PARAMETER["Longitude of natural origin",0],"#,
        r#"PARAMETER["Scale factor at natural origin",1],PARAMETER["False easting",0],"#,
        r#"PARAMETER["False northing",0]]]"#,
    );
    assert_eq!(
        crate::wkt_to_projstring(merc).unwrap(),
        concat!(
            "+proj=merc +lon_0=0 +k=1 +x_0=0 +y_0=0 +units=m +a=6378137 +rf=298.257223563",
            " +towgs84=0,0,0,0,0,0,0",
        )
    );
}

#[test]
fn parse_trailing_whitespaces() {
    setup();
    // .prj files and pasted WKT usually end with a newline
    for trailing in ["\n", "\r\n", " ", "\t", "  \n\n"] {
        let wkt = format!("{}{trailing}", fixtures::WKT_GEOGCS_WGS84);
        let r = Builder::new().parse(&wkt);
        assert!(
            matches!(r, Ok(Node::GEOGCRS(_))),
            "Trailing {trailing:?}: {r:?}"
        );
    }
}

#[test]
fn convert_ellipsoid_unit() {
    setup();
    // Only the semi-major axis is converted, inverse flattening is unitless
    for ellps in [
        r#"SPHEROID["Clarke 1866 (ft)",20926348,294.26,UNIT["foot",0.3048]]"#,
        r#"ELLIPSOID["Clarke 1866 (ft)",20926348,294.26,LENGTHUNIT["foot",0.3048]]"#,
    ] {
        let wkt =
            format!(r#"GEOGCS["test",DATUM["test",{ellps}],UNIT["degree",0.0174532925199433]]"#);
        assert_eq!(
            crate::wkt_to_projstring(&wkt).unwrap(),
            "+proj=longlat +a=6378350.8704 +rf=294.26",
            "{ellps}"
        );
    }
}

#[test]
fn convert_lcc_1sp_lat_0() {
    setup();
    // Latitude of natural origin is both 'lat_1' and 'lat_0'
    let wkt1 = concat!(
        r#"PROJCS["test",GEOGCS["WGS 84",DATUM["WGS_1984","#,
        r#"SPHEROID["WGS 84",6378137,298.257223563]],UNIT["degree",0.0174532925199433]],"#,
        r#"PROJECTION["Lambert_Conformal_Conic_1SP"],"#,
        r#"PARAMETER["latitude_of_origin",46.8],PARAMETER["central_meridian",2.337],"#,
        r#"PARAMETER["scale_factor",0.9998],PARAMETER["false_easting",600000],"#,
        r#"PARAMETER["false_northing",2200000],UNIT["metre",1]]"#,
    );
    let wkt2 = concat!(
        r#"PROJCRS["test",BASEGEOGCRS["WGS 84",DATUM["World Geodetic System 1984","#,
        r#"ELLIPSOID["WGS 84",6378137,298.257223563]]],CONVERSION["test","#,
        r#"METHOD["Lambert Conic Conformal (1SP)",ID["EPSG",9801]],"#,
        r#"PARAMETER["Latitude of natural origin",46.8,ID["EPSG",8801]],"#,
        r#"PARAMETER["Longitude of natural origin",2.337,ID["EPSG",8802]],"#,
        r#"PARAMETER["Scale factor at natural origin",0.9998,ID["EPSG",8805]],"#,
        r#"PARAMETER["False easting",600000,ID["EPSG",8806]],"#,
        r#"PARAMETER["False northing",2200000,ID["EPSG",8807]]]]"#,
    );
    for wkt in [wkt1, wkt2] {
        assert_eq!(
            crate::wkt_to_projstring(wkt).unwrap(),
            concat!(
                "+proj=lcc +lat_1=46.8 +lat_0=46.8 +lon_0=2.337 +k_0=0.9998",
                " +x_0=600000 +y_0=2200000 +units=m +a=6378137 +rf=298.257223563",
                " +towgs84=0,0,0,0,0,0,0",
            ),
            "{wkt}"
        );
    }
}

#[test]
fn convert_polar_stereographic() {
    setup();
    const GEOGCS: &str = concat!(
        r#"GEOGCS["WGS 84",DATUM["WGS_1984",SPHEROID["WGS 84",6378137,298.257223563]],"#,
        r#"PRIMEM["Greenwich",0],UNIT["degree",0.0174532925199433]]"#,
    );
    const ELLPS: &str = " +a=6378137 +rf=298.257223563 +towgs84=0,0,0,0,0,0,0";

    // WKT1 latitude of origin is the latitude of true scale (variant B)
    let wkt = format!(
        concat!(
            r#"PROJCS["WGS 84 / Antarctic Polar Stereographic",{},"#,
            r#"PROJECTION["Polar_Stereographic"],PARAMETER["latitude_of_origin",-71],"#,
            r#"PARAMETER["central_meridian",0],PARAMETER["false_easting",0],"#,
            r#"PARAMETER["false_northing",0],UNIT["metre",1]]"#,
        ),
        GEOGCS
    );
    assert_eq!(
        crate::wkt_to_projstring(&wkt).unwrap(),
        format!("+proj=stere +lat_0=-90 +lat_ts=-71 +lon_0=0 +x_0=0 +y_0=0 +units=m{ELLPS}"),
    );

    // WKT1 latitude of origin is the pole (variant A)
    let wkt = format!(
        concat!(
            r#"PROJCS["WGS 84 / UPS North (N,E)",{},"#,
            r#"PROJECTION["Polar_Stereographic"],PARAMETER["latitude_of_origin",90],"#,
            r#"PARAMETER["central_meridian",0],PARAMETER["scale_factor",0.994],"#,
            r#"PARAMETER["false_easting",2000000],PARAMETER["false_northing",2000000],"#,
            r#"UNIT["metre",1]]"#,
        ),
        GEOGCS
    );
    assert_eq!(
        crate::wkt_to_projstring(&wkt).unwrap(),
        format!(
            "+proj=stere +lat_0=90 +lon_0=0 +k=0.994 +x_0=2000000 +y_0=2000000 +units=m{ELLPS}"
        ),
    );

    // WKT2 variant B
    let wkt = concat!(
        r#"PROJCRS["WGS 84 / Antarctic Polar Stereographic",BASEGEOGCRS["WGS 84","#,
        r#"DATUM["World Geodetic System 1984",ELLIPSOID["WGS 84",6378137,298.257223563]]],"#,
        r#"CONVERSION["Antarctic Polar Stereographic","#,
        r#"METHOD["Polar Stereographic (variant B)",ID["EPSG",9829]],"#,
        r#"PARAMETER["Latitude of standard parallel",-71,ID["EPSG",8832]],"#,
        r#"PARAMETER["Longitude of origin",0,ID["EPSG",8833]],"#,
        r#"PARAMETER["False easting",0,ID["EPSG",8806]],"#,
        r#"PARAMETER["False northing",0,ID["EPSG",8807]]]]"#,
    );
    assert_eq!(
        crate::wkt_to_projstring(wkt).unwrap(),
        format!("+proj=stere +lat_0=-90 +lat_ts=-71 +lon_0=0 +x_0=0 +y_0=0 +units=m{ELLPS}"),
    );
}

#[test]
fn convert_prime_meridian() {
    setup();
    const ELLPS: &str = " +a=6378249.2 +rf=293.4660212936269 +towgs84=-168,-60,320,0,0,0,0";
    let geogcs = |primem: &str, unit: &str| {
        format!(
            concat!(
                r#"GEOGCS["NTF (Paris)",DATUM["Nouvelle_Triangulation_Francaise_Paris","#,
                r#"SPHEROID["Clarke 1880 (IGN)",6378249.2,293.4660212936269],"#,
                r#"TOWGS84[-168,-60,320,0,0,0,0]],{},{}]"#,
            ),
            primem, unit
        )
    };
    let grad = r#"UNIT["grad",0.01570796326794897]"#;
    let degree = r#"UNIT["degree",0.0174532925199433]"#;

    for (wkt, pm) in [
        // WKT1: longitude in the GEOGCS unit
        (geogcs(r#"PRIMEM["Paris",2.5969213]"#, grad), "2.33722917"),
        // GDAL WKT1: Paris longitude in degrees with GEOGCS unit in grads
        (geogcs(r#"PRIMEM["Paris",2.33722917]"#, grad), "2.33722917"),
        (
            geogcs(r#"PRIMEM["Paris",2.33722917]"#, degree),
            "2.33722917",
        ),
        // WKT2: longitude in its own unit
        (
            geogcs(
                r#"PRIMEM["Ferro",-17.6666666666667,ANGLEUNIT["degree",0.0174532925199433]]"#,
                grad,
            ),
            "-17.6666666666667",
        ),
    ] {
        assert_eq!(
            crate::wkt_to_projstring(&wkt).unwrap(),
            format!("+proj=longlat{ELLPS} +pm={pm}"),
            "{wkt}"
        );
    }

    // Greenwich is not written
    assert_eq!(
        crate::wkt_to_projstring(&geogcs(r#"PRIMEM["Greenwich",0]"#, degree)).unwrap(),
        format!("+proj=longlat{ELLPS}"),
    );
}

#[test]
fn convert_wkt2_geodetic_crs() {
    setup();
    const WGS84: &str = " +a=6378137 +rf=298.257223563 +towgs84=0,0,0,0,0,0,0";

    for wkt in [
        fixtures::WKT2_2019_GEOGCRS_WGS84,
        fixtures::WKT2_2015_GEODCRS_WGS84,
    ] {
        assert_eq!(
            crate::wkt_to_projstring(wkt).unwrap(),
            format!("+proj=longlat{WGS84}"),
            "{wkt}"
        );
    }

    assert_eq!(
        crate::wkt_to_projstring(fixtures::WKT2_2019_PROJCRS_UTM31N).unwrap(),
        format!("+proj=tmerc +lat_0=0 +lon_0=3 +k=0.9996 +x_0=500000 +y_0=0 +units=m{WGS84}"),
    );

    // Vertical datum ensemble has no ellipsoid
    let compound = format!(
        concat!(
            r#"COMPOUNDCRS["WGS 84 + Height",{},"#,
            r#"VERTCRS["Height",ENSEMBLE["Vertical ensemble",MEMBER["A"],MEMBER["B"],"#,
            r#"ENSEMBLEACCURACY[0.1]],CS[vertical,1],AXIS["gravity-related height (H)",up,"#,
            r#"LENGTHUNIT["metre",1]]]]"#,
        ),
        fixtures::WKT2_2019_GEOGCRS_WGS84
    );
    assert_eq!(
        crate::wkt_to_projstring(&compound).unwrap(),
        format!("+proj=longlat{WGS84}"),
    );

    // Geocentric crs is not supported
    assert!(crate::wkt_to_projstring(fixtures::WKT2_2019_GEODCRS_GEOCENTRIC).is_err());
}

#[test]
fn convert_esri_wkt1() {
    setup();
    // ESRI WKT1 (projinfo -o WKT1_ESRI, PROJ 9.4.0)
    for (wkt, projstr) in [
        // EPSG:3035, LAEA: ESRI parameter names
        (
            r#"PROJCS["ETRS_1989_LAEA",GEOGCS["GCS_ETRS_1989",DATUM["D_ETRS_1989",SPHEROID["GRS_1980",6378137.0,298.257222101]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Lambert_Azimuthal_Equal_Area"],PARAMETER["False_Easting",4321000.0],PARAMETER["False_Northing",3210000.0],PARAMETER["Central_Meridian",10.0],PARAMETER["Latitude_Of_Origin",52.0],UNIT["Meter",1.0]]"#,
            "+proj=laea +x_0=4321000.0 +y_0=3210000.0 +lon_0=10.0 +lat_0=52.0 +units=m +a=6378137.0 +rf=298.257222101 +towgs84=0,0,0,0,0,0,0",
        ),
        // EPSG:2154, Lambert_Conformal_Conic with 2 standard parallels: LCC 2SP
        (
            r#"PROJCS["RGF_1993_Lambert_93",GEOGCS["GCS_RGF_1993",DATUM["D_RGF_1993",SPHEROID["GRS_1980",6378137.0,298.257222101]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Lambert_Conformal_Conic"],PARAMETER["False_Easting",700000.0],PARAMETER["False_Northing",6600000.0],PARAMETER["Central_Meridian",3.0],PARAMETER["Standard_Parallel_1",49.0],PARAMETER["Standard_Parallel_2",44.0],PARAMETER["Latitude_Of_Origin",46.5],UNIT["Meter",1.0]]"#,
            "+proj=lcc +x_0=700000.0 +y_0=6600000.0 +lon_0=3.0 +lat_1=49.0 +lat_2=44.0 +lat_0=46.5 +units=m +a=6378137.0 +rf=298.257222101 +towgs84=0,0,0,0,0,0,0",
        ),
        // EPSG:27572, Lambert_Conformal_Conic with 1 standard parallel: LCC 1SP
        (
            r#"PROJCS["NTF_Paris_Lambert_Zone_II",GEOGCS["GCS_NTF_Paris",DATUM["Nouvelle_Triangulation_Francaise_(Paris)",SPHEROID["Clarke_1880_IGN",6378249.2,293.466021293627]],PRIMEM["Paris",2.33722917],UNIT["Grad",0.0157079632679489]],PROJECTION["Lambert_Conformal_Conic"],PARAMETER["False_Easting",600000.0],PARAMETER["False_Northing",2200000.0],PARAMETER["Central_Meridian",0.0],PARAMETER["Standard_Parallel_1",52.0],PARAMETER["Scale_Factor",0.99987742],PARAMETER["Latitude_Of_Origin",52.0],UNIT["Meter",1.0]]"#,
            "+proj=lcc +x_0=600000.0 +y_0=2200000.0 +lon_0=0 +k_0=0.99987742 +lat_1=46.8 +lat_0=46.8 +units=m +a=6378249.2 +rf=293.466021293627 +pm=2.33722917",
        ),
        // EPSG:3031, Stereographic_South_Pole: Polar Stereographic variant B
        (
            r#"PROJCS["WGS_1984_Antarctic_Polar_Stereographic",GEOGCS["GCS_WGS_1984",DATUM["D_WGS_1984",SPHEROID["WGS_1984",6378137.0,298.257223563]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Stereographic_South_Pole"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",0.0],PARAMETER["Standard_Parallel_1",-71.0],UNIT["Meter",1.0]]"#,
            "+proj=stere +lat_0=-90 +x_0=0.0 +y_0=0.0 +lon_0=0.0 +lat_ts=-71.0 +units=m +a=6378137.0 +rf=298.257223563 +towgs84=0,0,0,0,0,0,0",
        ),
        // EPSG:3857, Mercator_Auxiliary_Sphere: Popular Visualisation Pseudo Mercator
        (
            r#"PROJCS["WGS_1984_Web_Mercator_Auxiliary_Sphere",GEOGCS["GCS_WGS_1984",DATUM["D_WGS_1984",SPHEROID["WGS_1984",6378137.0,298.257223563]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Mercator_Auxiliary_Sphere"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Central_Meridian",0.0],PARAMETER["Standard_Parallel_1",0.0],PARAMETER["Auxiliary_Sphere_Type",0.0],UNIT["Meter",1.0]]"#,
            "+proj=webmerc +x_0=0.0 +y_0=0.0 +lon_0=0.0 +units=m +a=6378137.0 +rf=298.257223563 +towgs84=0,0,0,0,0,0,0",
        ),
    ] {
        assert_eq!(crate::wkt_to_projstring(wkt).unwrap(), projstr, "{wkt}");
    }

    // ESRI parameter name for central meridian
    let robinson = concat!(
        r#"PROJCS["Robinson",GEOGCS["GCS_WGS_1984",DATUM["D_WGS_1984","#,
        r#"SPHEROID["WGS_1984",6378137.0,298.257223563]],PRIMEM["Greenwich",0.0],"#,
        r#"UNIT["Degree",0.0174532925199433]],PROJECTION["Robinson"],"#,
        r#"PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],"#,
        r#"PARAMETER["Central_Meridian",150.0],UNIT["Meter",1.0]]"#,
    );
    assert_eq!(
        crate::wkt_to_projstring(robinson).unwrap(),
        concat!(
            "+proj=robin +x_0=0.0 +y_0=0.0 +lon_0=150.0 +units=m +a=6378137.0",
            " +rf=298.257223563 +towgs84=0,0,0,0,0,0,0",
        ),
    );

    // Only the WGS84 semi-major axis auxiliary sphere is supported
    let aux_sphere = concat!(
        r#"PROJCS["Mercator",GEOGCS["GCS_WGS_1984",DATUM["D_WGS_1984","#,
        r#"SPHEROID["WGS_1984",6378137.0,298.257223563]],PRIMEM["Greenwich",0.0],"#,
        r#"UNIT["Degree",0.0174532925199433]],PROJECTION["Mercator_Auxiliary_Sphere"],"#,
        r#"PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],"#,
        r#"PARAMETER["Central_Meridian",0.0],PARAMETER["Standard_Parallel_1",0.0],"#,
        r#"PARAMETER["Auxiliary_Sphere_Type",2.0],UNIT["Meter",1.0]]"#,
    );
    assert!(crate::wkt_to_projstring(aux_sphere).is_err());
}

#[test]
fn convert_sphere() {
    setup();
    // Inverse flattening of 0 defines a sphere (projinfo, PROJ 9.4.0)
    let epsg_3408 = r#"
            PROJCS["NSIDC EASE-Grid North",
                GEOGCS["NSIDC Authalic Sphere",
                    DATUM["NSIDC_International_1924_Authalic_Sphere",
                        SPHEROID["International 1924 Authalic Sphere",6371228,0,
                            AUTHORITY["EPSG","7057"]],
                        AUTHORITY["EPSG","1360"]],
                    PRIMEM["Greenwich",0,
                        AUTHORITY["EPSG","8901"]],
                    UNIT["degree",0.0174532925199433,
                        AUTHORITY["EPSG","9122"]],
                    AUTHORITY["EPSG","10346"]],
                PROJECTION["Lambert_Azimuthal_Equal_Area"],
                PARAMETER["latitude_of_center",90],
                PARAMETER["longitude_of_center",0],
                PARAMETER["false_easting",0],
                PARAMETER["false_northing",0],
                UNIT["metre",1,
                    AUTHORITY["EPSG","9001"]],
                AUTHORITY["EPSG","3408"]]"#;
    assert_eq!(
        crate::wkt_to_projstring(epsg_3408).unwrap(),
        "+proj=laea +lat_0=90 +lon_0=0 +x_0=0 +y_0=0 +units=m +R=6371228",
    );

    let epsg_4047 = r#"
            GEOGCRS["Unspecified datum based upon the GRS 1980 Authalic Sphere",
                DATUM["Not specified (based on GRS 1980 Authalic Sphere)",
                    ELLIPSOID["GRS 1980 Authalic Sphere",6371007,0,
                        LENGTHUNIT["metre",1]]],
                PRIMEM["Greenwich",0,
                    ANGLEUNIT["degree",0.0174532925199433]],
                CS[ellipsoidal,2],
                    AXIS["geodetic latitude (Lat)",north,
                        ORDER[1],
                        ANGLEUNIT["degree",0.0174532925199433]],
                    AXIS["geodetic longitude (Lon)",east,
                        ORDER[2],
                        ANGLEUNIT["degree",0.0174532925199433]],
                USAGE[
                    SCOPE["Statistical analysis."],
                    AREA["Not specified."],
                    BBOX[-90,-180,90,180]],
                ID["EPSG",4047]]"#;
    assert_eq!(
        crate::wkt_to_projstring(epsg_4047).unwrap(),
        "+proj=longlat +R=6371007",
    );

    // Radius is converted to meter
    let feet = concat!(
        r#"GEOGCS["test",DATUM["test",SPHEROID["sphere",20000000,0,UNIT["foot",0.3048]]],"#,
        r#"UNIT["degree",0.0174532925199433]]"#,
    );
    assert_eq!(
        crate::wkt_to_projstring(feet).unwrap(),
        "+proj=longlat +R=6096000",
    );
}

#[test]
fn convert_krovak() {
    setup();
    // EPSG:2065 is Krovak (EPSG:9819), EPSG:5514 is Krovak (North Orientated)
    // (projinfo, PROJ 9.4.0)
    for (wkt, projstr) in [
        // WKT2: method EPSG code
        (
            r#"
        PROJCRS["S-JTSK (Ferro) / Krovak",
            BASEGEOGCRS["S-JTSK (Ferro)",
                DATUM["System of the Unified Trigonometrical Cadastral Network (Ferro)",
                    ELLIPSOID["Bessel 1841",6377397.155,299.1528128,
                        LENGTHUNIT["metre",1]]],
                PRIMEM["Ferro",-17.6666666666667,
                    ANGLEUNIT["degree",0.0174532925199433]],
                ID["EPSG",4818]],
            CONVERSION["Krovak",
                METHOD["Krovak",
                    ID["EPSG",9819]],
                PARAMETER["Latitude of projection centre",49.5,
                    ANGLEUNIT["degree",0.0174532925199433],
                    ID["EPSG",8811]],
                PARAMETER["Longitude of origin",42.5,
                    ANGLEUNIT["degree",0.0174532925199433],
                    ID["EPSG",8833]],
                PARAMETER["Co-latitude of cone axis",30.2881397527778,
                    ANGLEUNIT["degree",0.0174532925199433],
                    ID["EPSG",1036]],
                PARAMETER["Latitude of pseudo standard parallel",78.5,
                    ANGLEUNIT["degree",0.0174532925199433],
                    ID["EPSG",8818]],
                PARAMETER["Scale factor on pseudo standard parallel",0.9999,
                    SCALEUNIT["unity",1],
                    ID["EPSG",8819]],
                PARAMETER["False easting",0,
                    LENGTHUNIT["metre",1],
                    ID["EPSG",8806]],
                PARAMETER["False northing",0,
                    LENGTHUNIT["metre",1],
                    ID["EPSG",8807]]],
            CS[Cartesian,2],
                AXIS["southing (X)",south,
                    ORDER[1],
                    LENGTHUNIT["metre",1]],
                AXIS["westing (Y)",west,
                    ORDER[2],
                    LENGTHUNIT["metre",1]],
            USAGE[
                SCOPE["Cadastre in Czechia."],
                AREA["Czechia; Slovakia."],
                BBOX[47.73,12.09,51.06,22.56]],
            ID["EPSG",2065]]"#,
            concat!(
                "+proj=krovak +lat_0=49.5 +lon_0=42.5 +alpha=30.2881397527778 +k=0.9999",
                " +x_0=0 +y_0=0 +units=m +a=6377397.155 +rf=299.1528128",
                " +pm=-17.6666666666667 +axis=swu",
            ),
        ),
        // ESRI: X_Scale, Y_Scale, XY_Plane_Rotation of 1, 1, 0
        (
            r#"PROJCS["S-JTSK_Ferro_Krovak",GEOGCS["GCS_S_JTSK_Ferro",DATUM["System_of_the_Unified_Trigonometrical_Cadastral_Network_(Ferro)",SPHEROID["Bessel_1841",6377397.155,299.1528128]],PRIMEM["Ferro",-17.6666666666667],UNIT["Degree",0.0174532925199433]],PROJECTION["Krovak"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Pseudo_Standard_Parallel_1",78.5],PARAMETER["Scale_Factor",0.9999],PARAMETER["Azimuth",30.2881397527778],PARAMETER["Longitude_Of_Center",42.5],PARAMETER["Latitude_Of_Center",49.5],PARAMETER["X_Scale",1.0],PARAMETER["Y_Scale",1.0],PARAMETER["XY_Plane_Rotation",0.0],UNIT["Meter",1.0]]"#,
            concat!(
                "+proj=krovak +x_0=0.0 +y_0=0.0 +k=0.9999 +alpha=30.2881397527778",
                " +lon_0=42.5 +lat_0=49.5 +units=m +a=6377397.155 +rf=299.1528128",
                " +pm=-17.6666666666667 +axis=swu",
            ),
        ),
        // ESRI: any other values are north orientated
        (
            r#"PROJCS["S-JTSK_Krovak_East_North",GEOGCS["GCS_S_JTSK",DATUM["D_S_JTSK",SPHEROID["Bessel_1841",6377397.155,299.1528128]],PRIMEM["Greenwich",0.0],UNIT["Degree",0.0174532925199433]],PROJECTION["Krovak"],PARAMETER["False_Easting",0.0],PARAMETER["False_Northing",0.0],PARAMETER["Pseudo_Standard_Parallel_1",78.5],PARAMETER["Scale_Factor",0.9999],PARAMETER["Azimuth",30.28813975277778],PARAMETER["Longitude_Of_Center",24.83333333333333],PARAMETER["Latitude_Of_Center",49.5],PARAMETER["X_Scale",-1.0],PARAMETER["Y_Scale",1.0],PARAMETER["XY_Plane_Rotation",90.0],UNIT["Meter",1.0]]"#,
            concat!(
                "+proj=krovak +x_0=0.0 +y_0=0.0 +k=0.9999 +alpha=30.28813975277778",
                " +lon_0=24.83333333333333 +lat_0=49.5 +units=m +a=6377397.155",
                " +rf=299.1528128",
            ),
        ),
    ] {
        assert_eq!(crate::wkt_to_projstring(wkt).unwrap(), projstr, "{wkt}");
    }

    // WKT1: only south-west axes are not north orientated, as in PROJ.
    // GDAL WKT1 export of EPSG:2065 has no AXIS
    let wkt1 = r#"
        PROJCS["S-JTSK (Ferro) / Krovak",
            GEOGCS["S-JTSK (Ferro)",
                DATUM["System_of_the_Unified_Trigonometrical_Cadastral_Network_Ferro",
                    SPHEROID["Bessel 1841",6377397.155,299.1528128,
                        AUTHORITY["EPSG","7004"]],
                    AUTHORITY["EPSG","6818"]],
                PRIMEM["Ferro",-17.6666666666667,
                    AUTHORITY["EPSG","8909"]],
                UNIT["degree",0.0174532925199433,
                    AUTHORITY["EPSG","9122"]],
                AUTHORITY["EPSG","4818"]],
            PROJECTION["Krovak"],
            PARAMETER["latitude_of_center",49.5],
            PARAMETER["longitude_of_center",42.5],
            PARAMETER["azimuth",30.2881397527778],
            PARAMETER["pseudo_standard_parallel_1",78.5],
            PARAMETER["scale_factor",0.9999],
            PARAMETER["false_easting",0],
            PARAMETER["false_northing",0],
            UNIT["metre",1,
                AUTHORITY["EPSG","9001"]],
            AUTHORITY["EPSG","2065"]]"#;
    let projstr = concat!(
        "+proj=krovak +lat_0=49.5 +lon_0=42.5 +alpha=30.2881397527778 +k=0.9999",
        " +x_0=0 +y_0=0 +units=m +a=6377397.155 +rf=299.1528128",
        " +pm=-17.6666666666667",
    );
    assert_eq!(crate::wkt_to_projstring(wkt1).unwrap(), projstr);

    let wkt1 = wkt1.replace(
        r#"AUTHORITY["EPSG","2065"]]"#,
        r#"AXIS["X",SOUTH],AXIS["Y",WEST],AUTHORITY["EPSG","2065"]]"#,
    );
    assert_eq!(
        crate::wkt_to_projstring(&wkt1).unwrap(),
        format!("{projstr} +axis=swu")
    );
}

#[test]
fn convert_parameter_unit() {
    setup();
    // Parameter unit with UNIT keyword has the type of the parameter:
    // EPSG:27572 WKT2 2019 (projinfo, PROJ 9.4.0) with UNIT keyword only
    let wkt = r#"
        PROJCRS["NTF (Paris) / Lambert zone II",
            BASEGEOGCRS["NTF (Paris)",
                DATUM["Nouvelle Triangulation Francaise (Paris)",
                    ELLIPSOID["Clarke 1880 (IGN)",6378249.2,293.466021293627,
                        UNIT["metre",1]]],
                PRIMEM["Paris",2.5969213,
                    UNIT["grad",0.0157079632679489]],
                ID["EPSG",4807]],
            CONVERSION["Lambert zone II",
                METHOD["Lambert Conic Conformal (1SP)",
                    ID["EPSG",9801]],
                PARAMETER["Latitude of natural origin",52,
                    UNIT["grad",0.0157079632679489],
                    ID["EPSG",8801]],
                PARAMETER["Longitude of natural origin",0,
                    UNIT["grad",0.0157079632679489],
                    ID["EPSG",8802]],
                PARAMETER["Scale factor at natural origin",0.99987742,
                    UNIT["unity",1],
                    ID["EPSG",8805]],
                PARAMETER["False easting",600000,
                    UNIT["metre",1],
                    ID["EPSG",8806]],
                PARAMETER["False northing",2200000,
                    UNIT["metre",1],
                    ID["EPSG",8807]]],
            CS[Cartesian,2],
                AXIS["easting (X)",east,
                    ORDER[1],
                    UNIT["metre",1]],
                AXIS["northing (Y)",north,
                    ORDER[2],
                    UNIT["metre",1]],
            USAGE[
                SCOPE["Engineering survey, topographic mapping."],
                AREA["France mainland onshore between 50.5 grads and 53.5 grads North (45°27'N to 48°09'N). Also used over all onshore mainland France."],
                BBOX[42.33,-4.87,51.14,8.23]],
            ID["EPSG",27572]]"#;
    assert_eq!(
        crate::wkt_to_projstring(wkt).unwrap(),
        concat!(
            "+proj=lcc +lat_1=46.8 +lat_0=46.8 +lon_0=0",
            " +k_0=0.99987742 +x_0=600000 +y_0=2200000 +units=m +a=6378249.2",
            " +rf=293.466021293627 +pm=2.33722917",
        ),
    );

    const BASE: &str = concat!(
        r#"PROJCRS["test",BASEGEOGCRS["WGS 84",DATUM["WGS_1984","#,
        r#"ELLIPSOID["WGS 84",6378137,298.257223563]]],CONVERSION["test","#,
        r#"METHOD["Transverse Mercator",ID["EPSG",9807]],"#,
    );

    // Linear parameter with UNIT keyword
    let wkt = format!(
        concat!(
            "{}",
            r#"PARAMETER["Longitude of natural origin",-75,UNIT["degree",0.0174532925199433]],"#,
            r#"PARAMETER["False easting",984250,UNIT["US survey foot",0.304800609601219]]]]"#,
        ),
        BASE
    );
    assert_eq!(
        crate::wkt_to_projstring(&wkt).unwrap(),
        concat!(
            "+proj=tmerc +lon_0=-75 +x_0=300000 +units=m +a=6378137",
            " +rf=298.257223563 +towgs84=0,0,0,0,0,0,0",
        ),
    );

    // Unit of the wrong type
    let wkt = format!(
        r#"{}PARAMETER["Longitude of natural origin",-75,LENGTHUNIT["metre",1]]]]"#,
        BASE
    );
    assert!(crate::wkt_to_projstring(&wkt).is_err());
}

#[test]
fn convert_datum_without_towgs84() {
    setup();
    // (projinfo, PROJ 9.4.0)
    // OSGB36 (Airy ellipsoid): the datum shift is unknown
    let epsg_27700 = r#"
        PROJCS["OSGB36 / British National Grid",
            GEOGCS["OSGB36",
                DATUM["Ordnance_Survey_of_Great_Britain_1936",
                    SPHEROID["Airy 1830",6377563.396,299.3249646,
                        AUTHORITY["EPSG","7001"]],
                    AUTHORITY["EPSG","6277"]],
                PRIMEM["Greenwich",0,
                    AUTHORITY["EPSG","8901"]],
                UNIT["degree",0.0174532925199433,
                    AUTHORITY["EPSG","9122"]],
                AUTHORITY["EPSG","4277"]],
            PROJECTION["Transverse_Mercator"],
            PARAMETER["latitude_of_origin",49],
            PARAMETER["central_meridian",-2],
            PARAMETER["scale_factor",0.9996012717],
            PARAMETER["false_easting",400000],
            PARAMETER["false_northing",-100000],
            UNIT["metre",1,
                AUTHORITY["EPSG","9001"]],
            AXIS["Easting",EAST],
            AXIS["Northing",NORTH],
            AUTHORITY["EPSG","27700"]]"#;
    assert_eq!(
        crate::wkt_to_projstring(epsg_27700).unwrap(),
        concat!(
            "+proj=tmerc +lat_0=49 +lon_0=-2 +k=0.9996012717 +x_0=400000",
            " +y_0=-100000 +units=m +a=6377563.396 +rf=299.3249646",
        ),
    );

    // ETRS89 (GRS80 ellipsoid) is compatible with WGS84
    let epsg_4258 = r#"
        GEOGCRS["ETRS89",
            ENSEMBLE["European Terrestrial Reference System 1989 ensemble",
                MEMBER["European Terrestrial Reference Frame 1989"],
                MEMBER["European Terrestrial Reference Frame 1990"],
                MEMBER["European Terrestrial Reference Frame 1991"],
                MEMBER["European Terrestrial Reference Frame 1992"],
                MEMBER["European Terrestrial Reference Frame 1993"],
                MEMBER["European Terrestrial Reference Frame 1994"],
                MEMBER["European Terrestrial Reference Frame 1996"],
                MEMBER["European Terrestrial Reference Frame 1997"],
                MEMBER["European Terrestrial Reference Frame 2000"],
                MEMBER["European Terrestrial Reference Frame 2005"],
                MEMBER["European Terrestrial Reference Frame 2014"],
                ELLIPSOID["GRS 1980",6378137,298.257222101,
                    LENGTHUNIT["metre",1]],
                ENSEMBLEACCURACY[0.1]],
            PRIMEM["Greenwich",0,
                ANGLEUNIT["degree",0.0174532925199433]],
            CS[ellipsoidal,2],
                AXIS["geodetic latitude (Lat)",north,
                    ORDER[1],
                    ANGLEUNIT["degree",0.0174532925199433]],
                AXIS["geodetic longitude (Lon)",east,
                    ORDER[2],
                    ANGLEUNIT["degree",0.0174532925199433]],
            USAGE[
                SCOPE["Spatial referencing."],
                AREA["Europe - onshore and offshore: Albania; Andorra; Austria; Belgium; Bosnia and Herzegovina; Bulgaria; Croatia; Cyprus; Czechia; Denmark; Estonia; Faroe Islands; Finland; France; Germany; Gibraltar; Greece; Hungary; Ireland; Italy; Kosovo; Latvia; Liechtenstein; Lithuania; Luxembourg; Malta; Moldova; Monaco; Montenegro; Netherlands; North Macedonia; Norway including Svalbard and Jan Mayen; Poland; Portugal; Romania; San Marino; Serbia; Slovakia; Slovenia; Spain; Sweden; Switzerland; United Kingdom (UK) including Channel Islands and Isle of Man; Vatican City State."],
                BBOX[32.88,-16.1,84.73,40.18]],
            ID["EPSG",4258]]"#;
    assert_eq!(
        crate::wkt_to_projstring(epsg_4258).unwrap(),
        "+proj=longlat +a=6378137 +rf=298.257222101 +towgs84=0,0,0,0,0,0,0",
    );
}

#[test]
fn convert_degree_unit() {
    setup();
    // WKT unit names are free text: the EPSG code, otherwise the
    // conversion factor, defines the degree
    let wkt = |unit: &str, primem: &str| {
        format!(
            concat!(
                r#"PROJCS["test",GEOGCS["test",DATUM["WGS_1984","#,
                r#"SPHEROID["WGS 84",6378137,298.257223563]],{},{}],"#,
                r#"PROJECTION["Transverse_Mercator"],PARAMETER["central_meridian",3],"#,
                r#"UNIT["metre",1]]"#,
            ),
            primem, unit
        )
    };
    const GREENWICH: &str = r#"PRIMEM["Greenwich",0]"#;
    const WGS84: &str = " +units=m +a=6378137 +rf=298.257223563 +towgs84=0,0,0,0,0,0,0";

    for (unit, primem, projstr) in [
        // Any name with the degree conversion factor
        (
            r#"UNIT["Degrees",0.0174532925199433]"#,
            GREENWICH,
            format!("+proj=tmerc +lon_0=3{WGS84}"),
        ),
        (
            r#"UNIT["Degrees",0.0174532925199433]"#,
            r#"PRIMEM["Ferro",-17.6666666666667]"#,
            format!("+proj=tmerc +lon_0=3{WGS84} +pm=-17.6666666666667"),
        ),
        // EPSG code takes precedence over the conversion factor
        (
            r#"UNIT["deg",0.01745329,AUTHORITY["EPSG","9122"]]"#,
            GREENWICH,
            format!("+proj=tmerc +lon_0=3{WGS84}"),
        ),
        (
            r#"UNIT["deg",0.01745329]"#,
            GREENWICH,
            format!("+proj=tmerc +lon_0=2.999999566854{WGS84}"),
        ),
        // EPSG code takes precedence over the name (grad)
        (
            r#"UNIT["degree",0.01570796326794897,AUTHORITY["EPSG","9105"]]"#,
            GREENWICH,
            format!("+proj=tmerc +lon_0=2.7{WGS84}"),
        ),
    ] {
        assert_eq!(
            crate::wkt_to_projstring(&wkt(unit, primem)).unwrap(),
            projstr,
            "{unit} {primem}"
        );
    }
}

#[test]
fn convert_non_epsg_authority() {
    setup();
    // EPSG:2154, WKT2 2019 (projinfo, PROJ 9.4.0)
    const WKT: &str = r#"
        PROJCRS["RGF93 v1 / Lambert-93",
            BASEGEOGCRS["RGF93 v1",
                DATUM["Reseau Geodesique Francais 1993 v1",
                    ELLIPSOID["GRS 1980",6378137,298.257222101,
                        LENGTHUNIT["metre",1]]],
                PRIMEM["Greenwich",0,
                    ANGLEUNIT["degree",0.0174532925199433]],
                ID["EPSG",4171]],
            CONVERSION["Lambert-93",
                METHOD["Lambert Conic Conformal (2SP)",
                    ID["EPSG",9802]],
                PARAMETER["Latitude of false origin",46.5,
                    ANGLEUNIT["degree",0.0174532925199433],
                    ID["EPSG",8821]],
                PARAMETER["Longitude of false origin",3,
                    ANGLEUNIT["degree",0.0174532925199433],
                    ID["EPSG",8822]],
                PARAMETER["Latitude of 1st standard parallel",49,
                    ANGLEUNIT["degree",0.0174532925199433],
                    ID["EPSG",8823]],
                PARAMETER["Latitude of 2nd standard parallel",44,
                    ANGLEUNIT["degree",0.0174532925199433],
                    ID["EPSG",8824]],
                PARAMETER["Easting at false origin",700000,
                    LENGTHUNIT["metre",1],
                    ID["EPSG",8826]],
                PARAMETER["Northing at false origin",6600000,
                    LENGTHUNIT["metre",1],
                    ID["EPSG",8827]]],
            CS[Cartesian,2],
                AXIS["easting (X)",east,
                    ORDER[1],
                    LENGTHUNIT["metre",1]],
                AXIS["northing (Y)",north,
                    ORDER[2],
                    LENGTHUNIT["metre",1]],
            USAGE[
                SCOPE["Engineering survey, topographic mapping."],
                AREA["France - onshore and offshore, mainland and Corsica (France métropolitaine including Corsica)."],
                BBOX[41.15,-9.86,51.56,10.38]],
            ID["EPSG",2154]]"#;
    const PROJSTR: &str = concat!(
        "+proj=lcc +lat_0=46.5 +lon_0=3 +lat_1=49 +lat_2=44 +x_0=700000",
        " +y_0=6600000 +units=m +a=6378137 +rf=298.257222101 +towgs84=0,0,0,0,0,0,0",
    );
    assert_eq!(crate::wkt_to_projstring(WKT).unwrap(), PROJSTR);

    // Method with another authority, or with an unknown EPSG code,
    // is found by name
    for id in [r#"ID["ESRI",43020]"#, r#"ID["EPSG",99999]"#] {
        let wkt = WKT.replace(r#"ID["EPSG",9802]"#, id);
        assert_eq!(crate::wkt_to_projstring(&wkt).unwrap(), PROJSTR, "{id}");
    }

    // Parameters with another authority, or with unknown EPSG codes,
    // are found by name
    for authority in [r#"ID["IGNF","#, r#"ID["EPSG",99"#] {
        let wkt = ["8821", "8822", "8823", "8824", "8826", "8827"]
            .iter()
            .fold(WKT.to_string(), |wkt, code| {
                wkt.replace(
                    &format!(r#"ID["EPSG",{code}]"#),
                    &format!("{authority}{code}]"),
                )
            });
        assert!(!wkt.contains(r#"ID["EPSG",88"#));
        assert_eq!(
            crate::wkt_to_projstring(&wkt).unwrap(),
            PROJSTR,
            "{authority}"
        );
    }
}

#[test]
fn report_wkt_errors() {
    use crate::errors::Error;
    setup();
    // Processor errors are reported, the innermost one first
    for (wkt, message) in [
        (
            r#"GEOGCS["x",UNIT["degree",0.0174532925199433]]"#,
            "Missing DATUM for geodetic crs",
        ),
        (
            concat!(
                r#"PROJCS["x",GEOGCS["x",DATUM["d"],UNIT["degree",0.0174532925199433]],"#,
                r#"PROJECTION["Transverse_Mercator"],UNIT["metre",1]]"#,
            ),
            "Missing ellipsoid for DATUM",
        ),
        (
            concat!(
                r#"GEOGCS["x",DATUM["d",SPHEROID["s",6378137,298.257223563],"#,
                r#"TOWGS84[1,2]],UNIT["degree",0.0174532925199433]]"#,
            ),
            "Wrong number of parameters for TOWGS84",
        ),
        (
            fixtures::WKT2_2019_GEODCRS_GEOCENTRIC,
            "Geocentric crs is not supported",
        ),
    ] {
        match crate::wkt_to_projstring(wkt) {
            Err(Error::Wkt(msg)) => assert_eq!(msg, message, "{wkt}"),
            other => panic!("Expected Wkt error for {wkt}, got {other:?}"),
        }
    }

    // Syntax errors
    for wkt in [
        concat!(
            r#"GEOGCS["x",DATUM["d",SPHEROID["s",6378137,298.257223563]],"#,
            r#"UNIT["degree",0.0174532925199433]"#,
        ),
        "not a wkt",
    ] {
        assert!(
            matches!(crate::wkt_to_projstring(wkt), Err(Error::Parse)),
            "{wkt}"
        );
    }
}
