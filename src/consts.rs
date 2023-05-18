//!
//! WKT2/ISO 19111 names and codes
//!
#![allow(dead_code)]

pub struct Wkt2Const {
    pub name: &'static str,
    pub code: &'static str,
}

#[rustfmt::skip]
pub mod parameters {
    use super::*;
    macro_rules! define {
        ($name:ident, $wkt2_name:expr, $epsg_code:expr) => {
            pub const $name: Wkt2Const = Wkt2Const {
                name: $wkt2_name,
                code: $epsg_code,
            };
        };
    }

    define!(COLATITUDE_CONE_AXIS,                  "Co-latitude of cone axis",          "1036");
    define!(ELLIPSOID_SCALE_FACTOR,                "Ellipsoid scaling factor",          "1038");
    define!(PROJECTION_PLANE_ORIGIN_HEIGHT,        "Projection plane origin height",    "1039");

    define!(LATITUDE_OF_NATURAL_ORIGIN,            "Latitude of natural origin",        "8801");
    define!(LONGITUDE_OF_NATURAL_ORIGIN,           "Longitude of natural origin",       "8802");
    define!(SCALE_FACTOR_AT_NATURAL_ORIGIN,        "Scale factor at natural origin",    "8805");
    define!(FALSE_EASTING,                         "False easting",                     "8806");
    define!(FALSE_NORTHING,                        "False northing",                    "8807");
    define!(LATITUDE_PROJECTION_CENTRE,            "Latitude of projection centre",     "8811");
    define!(LONGITUDE_PROJECTION_CENTRE,           "Longitude of projection centre",    "8812");
    define!(AZIMUTH_INITIAL_LINE,                  "Azimuth of initial line",           "8813");
    define!(ANGLE_RECTIFIED_TO_SKEW_GRID,          "Angle from Rectified to Skew Grid", "8814");
    define!(SCALE_FACTOR_INITIAL_LINE,             "Scale factor on initial line",      "8815");
    define!(EASTING_PROJECTION_CENTRE,             "Easting at projection centre",      "8816");
    define!(NORTHING_PROJECTION_CENTRE,            "Northing at projection centre",     "8817");
    define!(LATITUDE_PSEUDO_STANDARD_PARALLEL,     "Latitude of pseudo standard parallel",     "8818");
    define!(SCALE_FACTOR_PSEUDO_STANDARD_PARALLEL, "Scale factor on pseudo standard parallel", "8819");
    define!(LATITUDE_FALSE_ORIGIN,                 "Latitude of false origin",          "8821");
    define!(LONGITUDE_FALSE_ORIGIN,                "Longitude of false origin",         "8822");
    define!(LATITUDE_1ST_STD_PARALLEL,             "Latitude of 1st standard parallel", "8823");
    define!(LATITUDE_2ND_STD_PARALLEL,             "Latitude of 2nd standard parallel", "8824");
    define!(EASTING_FALSE_ORIGIN,                  "Easting at false origin",           "8826");
    define!(NORTHING_FALSE_ORIGIN,                 "Northing at false origin",          "8827");
    define!(LATITUDE_STD_PARALLEL,                 "Latitude of standard parallel",     "8832");
    define!(LONGITUDE_OF_ORIGIN,                   "Longitude of origin",               "8833");
    define!(LATITUDE_TOPOGRAPHIC_ORIGIN,           "Latitude of topocentric origin",    "8834");
    define!(LONGITUDE_TOPOGRAPHIC_ORIGIN,          "Longitude of topocentric origin",   "8835");
    define!(ELLIPSOIDAL_HEIGHT_TOPOCENTRIC_ORIGIN, "Ellipsoidal height of topocentric origin", "8836");
    define!(VIEWPOINT_HEIGHT,                      "Viewpoint height",                  "8840");

    // No EPSG definition
    define!(LAT_FIRST_POINT,   "Latitude of 1st point" , "");
    define!(LONG_FIRST_POINT,  "Longitude of 1st point", "");
    define!(LAT_SECOND_POINT,  "Latitude of 2nd point",  "");
    define!(LONG_SECOND_POINT, "Longitude of 2nd point", "");
}

#[rustfmt::skip]
pub mod methods {
    use super::*;
    macro_rules! define {
        ($name:ident, $wkt2_name:expr, $epsg_code:expr) => {
            pub const $name: Wkt2Const = Wkt2Const {
                name: $wkt2_name,
                code: $epsg_code,
            };
        };
    }

    define!(POPULAR_VISUALISATION_PSEUDO_MERCATOR,  "Popular Visualisation Pseudo Mercator",    "1024");
    define!(LAMBERT_AZIMUTHAL_EQUAL_AREA_SPHERICAL, "Lambert Azimuthal Equal Area (Spherical)", "1027");
    define!(LAMBERT_CONIC_CONFORMAL_2SP_MICHIGAN,   "Lambert Conic Conformal (2SP Michigan)",   "1051");

    define!(LAMBERT_CONIC_CONFORMAL_1SP,            "Lambert Conic Conformal (1SP)",            "9801");
    define!(LAMBERT_CONIC_CONFORMAL_2SP,            "Lambert Conic Conformal (2SP)",            "9802");
    define!(LAMBERT_CONIC_CONFORMAL_2SP_BELGIUM,    "Lambert Conic Conformal (2SP Belgium)",    "9803");
    define!(MERCATOR_VARIANT_A,                     "Mercator (variant A)",                     "9804");
    define!(MERCATOR_VARIANT_B,                     "Mercator (variant B)",                     "9805");
    define!(TRANSVERSE_MERCATOR,                    "Transverse Mercator"                   ,   "9807");
    define!(TRANSVERSE_MERCATOR_SOUTH_ORIENTATED,   "Transverse Mercator (South Orientated)",   "9808");
    define!(OBLIQUE_STEREOGRAPHIC,                  "Oblique Stereographic",                    "9809");
    define!(POLAR_STEREOGRAPHIC_VARIANT_A,          "Polar Stereographic (variant A)",          "9810");
    define!(ALBERS_EQUAL_AREA,                      "Albers Equal Area",                        "9822");
    define!(LAMBERT_AZIMUTHAL_EQUAL_AREA,           "Lambert Azimuthal Equal Area",             "9820");
    define!(POLAR_STEREOGRAPHIC_VARIANT_B,          "Polar Stereographic (variant B)",          "9829");

    define!(PROJ_WKT2_NAME_MOLLWEIDE,            "Mollweide",     "");
    define!(PROJ_WKT2_NAME_WAGNER_IV,            "Wagner IV",     "");
    define!(PROJ_WKT2_NAME_WAGNER_V,             "Wagner V",      "");
    define!(PROJ_WKT2_NAME_METHOD_STEREOGRAPHIC, "Stereographic", "");
}
