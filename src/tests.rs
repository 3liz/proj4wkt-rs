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
                prime_meridian: Some(PrimeMeridian {
                    name: "Greenwich",
                    longitude: "0",
                    unit: None,
                }),
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
            " +towgs84=0,0,0,0,0,0,0",
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
            "+proj=longlat +a=6378350.8704 +rf=294.26 +towgs84=0,0,0,0,0,0,0",
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
        (
            geogcs(r#"PRIMEM["Paris",2.5969213]"#, grad),
            "2.3372291700000005",
        ),
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
