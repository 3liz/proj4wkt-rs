//!
//! Parameter mapping
//!
//! From proj library: <https://github.com/OSGeo/PROJ>  (/src/iso19111/operation/parammappings)
//!
#![allow(dead_code)]

use crate::consts::parameters;
use crate::model::UnitType;

const WKT1_LATITUDE_OF_ORIGIN: &str = "latitude_of_origin";
const WKT1_CENTRAL_MERIDIAN: &str = "central_meridian";
const WKT1_SCALE_FACTOR: &str = "scale_factor";
const WKT1_FALSE_EASTING: &str = "false_easting";
const WKT1_FALSE_NORTHING: &str = "false_northing";
const WKT1_STANDARD_PARALLEL_1: &str = "standard_parallel_1";
const WKT1_STANDARD_PARALLEL_2: &str = "standard_parallel_2";
const WKT1_LATITUDE_OF_CENTER: &str = "latitude_of_center";
const WKT1_LONGITUDE_OF_CENTER: &str = "longitude_of_center";
const WKT1_AZIMUTH: &str = "azimuth";
const WKT1_RECTIFIED_GRID_ANGLE: &str = "rectified_grid_angle";

#[derive(Debug)]
pub struct ParamMapping {
    pub proj_name: &'static str,
    pub wkt2_name: &'static str,
    pub epsg_code: &'static str,
    pub wkt1_name: &'static str,
    pub unit_type: UnitType,
}

impl ParamMapping {
    const LAT_0: &str = "lat_0";
    const LAT_1: &str = "lat_1";
    const LAT_2: &str = "lat_2";
    const LAT_TS: &str = "lat_ts";
    const LON_0: &str = "lon_0";
    const LON_1: &str = "lon_1";
    const LON_2: &str = "lon_2";
    const LONC: &str = "lonc";
    const ALPHA: &str = "alpha";
    const GAMMA: &str = "gamma";
    const K_0: &str = "k_0";
    const K: &str = "k";
    const X_0: &str = "x_0";
    const Y_0: &str = "y_0";
    const H: &str = "h";
    const NULL: &str = "";
}

macro_rules! map {
    ($name:ident, $wkt2:ident, $wkt1_name:expr, $unit_type:ident, $proj_name:ident) => {
        pub(crate) const $name: ParamMapping = ParamMapping {
            proj_name: ParamMapping::$proj_name,
            wkt2_name: parameters::$wkt2.name,
            epsg_code: parameters::$wkt2.code,
            wkt1_name: $wkt1_name,
            unit_type: UnitType::$unit_type,
        };
    };
}

map!(
    LATITUDE_NAT_ORIGIN,
    LATITUDE_OF_NATURAL_ORIGIN,
    WKT1_LATITUDE_OF_ORIGIN,
    Angular,
    LAT_0
);

map!(
    LONGITUDE_NAT_ORIGIN,
    LONGITUDE_OF_NATURAL_ORIGIN,
    WKT1_CENTRAL_MERIDIAN,
    Angular,
    LON_0
);

map!(
    SCALE_FACTOR,
    SCALE_FACTOR_AT_NATURAL_ORIGIN,
    WKT1_SCALE_FACTOR,
    Scale,
    K_0
);

map!(
    SCALE_FACTOR_K,
    SCALE_FACTOR_AT_NATURAL_ORIGIN,
    WKT1_SCALE_FACTOR,
    Scale,
    K
);

map!(
    FALSE_EASTING,
    FALSE_EASTING,
    WKT1_FALSE_EASTING,
    Linear,
    X_0
);

map!(
    FALSE_NORTHING,
    FALSE_NORTHING,
    WKT1_FALSE_NORTHING,
    Linear,
    Y_0
);

map!(
    LATITUDE_FALSE_ORIGIN,
    LATITUDE_FALSE_ORIGIN,
    WKT1_LATITUDE_OF_ORIGIN,
    Angular,
    LAT_0
);

map!(
    LONGITUDE_FALSE_ORIGIN,
    LONGITUDE_FALSE_ORIGIN,
    WKT1_CENTRAL_MERIDIAN,
    Angular,
    LON_0
);

map!(
    FALSE_EASTING_ORIGIN,
    EASTING_FALSE_ORIGIN,
    WKT1_FALSE_EASTING,
    Linear,
    X_0
);

map!(
    FALSE_NORTHING_ORIGIN,
    NORTHING_FALSE_ORIGIN,
    WKT1_FALSE_NORTHING,
    Linear,
    Y_0
);

map!(
    LATITUDE_1ST_STD_PARALLEL,
    LATITUDE_1ST_STD_PARALLEL,
    WKT1_STANDARD_PARALLEL_1,
    Angular,
    LAT_1
);

map!(
    LATITUDE_2ND_STD_PARALLEL,
    LATITUDE_2ND_STD_PARALLEL,
    WKT1_STANDARD_PARALLEL_2,
    Angular,
    LAT_2
);

map!(
    LAT_FALSE_ORIGIN_LAT_OF_CENTER,
    LATITUDE_FALSE_ORIGIN,
    WKT1_LATITUDE_OF_CENTER,
    Angular,
    LAT_0
);

map!(
    LONG_FALSE_ORIGIN_LONG_OF_CENTER,
    LONGITUDE_FALSE_ORIGIN,
    WKT1_LONGITUDE_OF_CENTER,
    Angular,
    LON_0
);

map!(
    LAT_FIRST_POINT,
    LAT_FIRST_POINT,
    "Latitude_Of_1st_Point",
    Angular,
    LAT_1
);
map!(
    LONG_FIRST_POINT,
    LONG_FIRST_POINT,
    "Longitude_Of_1st_Point",
    Angular,
    LON_1
);
map!(
    LAT_SECOND_POINT,
    LAT_SECOND_POINT,
    "Latitude_Of_2nd_Point",
    Angular,
    LAT_2
);
map!(
    LONG_SECOND_POINT,
    LONG_SECOND_POINT,
    "Longitude_Of_2nd_Point",
    Angular,
    LON_2
);

map!(
    ELLIPSOID_SCALE_FACTOR,
    ELLIPSOID_SCALE_FACTOR,
    "",
    Scale,
    K_0
);

map!(
    LAT_NAT_LAT_CENTER,
    LATITUDE_OF_NATURAL_ORIGIN,
    WKT1_LATITUDE_OF_CENTER,
    Angular,
    LAT_0
);

map!(
    LONG_NAT_LONG_CENTER,
    LONGITUDE_OF_NATURAL_ORIGIN,
    WKT1_LONGITUDE_OF_CENTER,
    Angular,
    LON_0
);

map!(
    LAT_NAT_ORIGIN_LAT1,
    LATITUDE_OF_NATURAL_ORIGIN,
    WKT1_STANDARD_PARALLEL_1,
    Angular,
    LAT_1
);

map!(
    LAT_1ST_PARALLEL_LAT_TS,
    LATITUDE_1ST_STD_PARALLEL,
    WKT1_STANDARD_PARALLEL_1,
    Angular,
    LAT_TS
);

map!(
    LAT_CENTRE_LAT_CENTER,
    LATITUDE_PROJECTION_CENTRE,
    WKT1_LATITUDE_OF_CENTER,
    Angular,
    LAT_0
);

map!(
    LON_CENTRE_LON_CENTER_LONC,
    LONGITUDE_PROJECTION_CENTRE,
    WKT1_LONGITUDE_OF_CENTER,
    Angular,
    LONC
);

map!(AZIMUTH, AZIMUTH_INITIAL_LINE, WKT1_AZIMUTH, Angular, ALPHA);

map!(
    ANGLE_TO_SKEW_GRID,
    ANGLE_RECTIFIED_TO_SKEW_GRID,
    WKT1_RECTIFIED_GRID_ANGLE,
    Angular,
    GAMMA
);

map!(
    SCALE_FACTOR_INITIALLINE,
    SCALE_FACTOR_INITIAL_LINE,
    WKT1_SCALE_FACTOR,
    Scale,
    K
);

map!(
    FALSE_EASTING_PROJECTION_CENTRE,
    EASTING_PROJECTION_CENTRE,
    WKT1_FALSE_EASTING,
    Linear,
    X_0
);

map!(
    FALSE_NORTHING_PROJECTION_CENTRE,
    NORTHING_PROJECTION_CENTRE,
    WKT1_FALSE_NORTHING,
    Linear,
    Y_0
);

map!(
    LAT_POINT_1,
    LAT_FIRST_POINT,
    "latitude_of_point_1",
    Angular,
    LAT_1
);

map!(
    LONG_POINT_1,
    LONG_FIRST_POINT,
    "longitude_of_point_1",
    Angular,
    LON_1
);

map!(
    LAT_POINT_2,
    LAT_SECOND_POINT,
    "latitude_of_point_2",
    Angular,
    LAT_2
);

map!(
    LONG_POINT_2,
    LONG_SECOND_POINT,
    "longitude_of_point_2",
    Angular,
    LON_2
);

map!(
    LONG_CENTRE_LONG_CENTER,
    LONGITUDE_OF_ORIGIN,
    WKT1_LONGITUDE_OF_CENTER,
    Angular,
    LON_0
);

map!(
    COLATITUDE_CONE_AXIS,
    COLATITUDE_CONE_AXIS,
    WKT1_AZIMUTH,
    Angular,
    ALPHA
); // ignored by PROJ currently

map!(
    LATITUDE_PSEUDO_STD_PARALLEL,
    LATITUDE_PSEUDO_STANDARD_PARALLEL,
    "pseudo_standard_parallel_1",
    Angular,
    NULL
);

map!(
    LAT_LCC_1SP,
    LATITUDE_OF_NATURAL_ORIGIN,
    WKT1_LATITUDE_OF_ORIGIN,
    Angular,
    LAT_1
);

map!(
    SCALE_FACTOR_PSEUDO_STD_PARALLEL,
    SCALE_FACTOR_PSEUDO_STANDARD_PARALLEL,
    WKT1_SCALE_FACTOR,
    Scale,
    K
);

map!(
    LAT_MERC_1SP,
    LATITUDE_OF_NATURAL_ORIGIN,
    "", // always set to zero, not to be exported in WKT1
    Angular,
    NULL // always set to zero, not to be exported in PROJ strings
);

map!(
    LAT_STD_PARALLEL,
    LATITUDE_STD_PARALLEL,
    WKT1_LATITUDE_OF_ORIGIN,
    Angular,
    LAT_TS
);

map!(
    LONG_ORIGIN,
    LONGITUDE_OF_ORIGIN,
    WKT1_CENTRAL_MERIDIAN,
    Angular,
    LON_0
);
