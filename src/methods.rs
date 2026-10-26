//!
//! Method mapping
//!
use crate::builder::parse_number;
use crate::consts::{Wkt2Const, methods};
use crate::errors::{Error, Result};
use crate::model::{Authority, Parameter, Projcs, Unit};
use crate::params::{self, ParamMapping};

pub struct MethodMapping {
    wkt2_name: &'static str,
    epsg_code: &'static str,
    wkt1_name: &'static str,
    proj_name: &'static str,
    proj_aux: &'static str,
    param_mapping: &'static [&'static ParamMapping],
    spherical: bool,
}

impl MethodMapping {
    pub fn proj_name(&self) -> &'static str {
        self.proj_name
    }

    pub fn proj_aux(&self) -> &'static str {
        self.proj_aux
    }

    /// The projection uses the sphere of radius the
    /// semi-major axis whatever the ellipsoid
    pub fn is_spherical(&self) -> bool {
        self.spherical
    }

    /// Check if this is the mapping for `method`
    pub fn is(&self, method: &Wkt2Const) -> bool {
        self.wkt2_name == method.name && self.epsg_code == method.code
    }

    /// Look up for mapped proj parameters
    ///
    /// A parameter may be mapped to several proj parameters
    /// (i.e `lat_1` and `lat_0` for LCC 1SP)
    pub fn find_proj_params<'a>(
        &'a self,
        p: &'a Parameter<'a>,
    ) -> impl Iterator<Item = &'a ParamMapping> + 'a {
        // Trust EPSG code if known, otherwise check name
        let code = epsg_code(&p.authority)
            .filter(|code| self.param_mapping.iter().any(|pp| pp.epsg_code == *code));

        self.param_mapping.iter().copied().filter(move |pp| {
            !p.name.is_empty()
                && !pp.proj_name.is_empty()
                && match code {
                    Some(code) => pp.epsg_code == code,
                    None => {
                        pp.wkt2_name.eq_ignore_ascii_case(p.name)
                            || pp.wkt1_name.eq_ignore_ascii_case(p.name)
                    }
                }
        })
    }
}

macro_rules! method {
    {$wkt2:ident, $wkt1_name:expr, $proj_name:expr, $proj_aux:expr,
     $mapping:expr} => {
        method! {$wkt2, $wkt1_name, $proj_name, $proj_aux, $mapping, false}
    };
    {$wkt2:ident, $wkt1_name:expr, $proj_name:expr, $proj_aux:expr,
     $mapping:expr, $spherical:expr} => {
        MethodMapping {
            wkt2_name: methods::$wkt2.name,
            epsg_code: methods::$wkt2.code,
            wkt1_name: $wkt1_name,
            proj_name: $proj_name,
            proj_aux: $proj_aux,
            param_mapping: $mapping,
            spherical: $spherical,
        }
    };
}

mod parameters {
    use crate::params::{self, ParamMapping};

    pub const NAT_ORIGIN: [&ParamMapping; 4] = [
        &params::LATITUDE_NAT_ORIGIN,
        &params::LONGITUDE_NAT_ORIGIN,
        &params::FALSE_EASTING,
        &params::FALSE_NORTHING,
    ];

    pub const LONG_NAT_ORIGIN: [&ParamMapping; 3] = [
        &params::LONGITUDE_NAT_ORIGIN,
        &params::FALSE_EASTING,
        &params::FALSE_NORTHING,
    ];

    pub const NAT_ORIGIN_SCALE_K: [&ParamMapping; 5] = [
        &params::LATITUDE_NAT_ORIGIN,
        &params::LONGITUDE_NAT_ORIGIN,
        &params::SCALE_FACTOR_K,
        &params::FALSE_EASTING,
        &params::FALSE_NORTHING,
    ];

    pub const LCC_1SP: [&ParamMapping; 6] = [
        &params::LAT_LCC_1SP,
        &params::LAT_LCC_1SP_LAT_0,
        &params::LONGITUDE_NAT_ORIGIN,
        &params::SCALE_FACTOR,
        &params::FALSE_EASTING,
        &params::FALSE_NORTHING,
    ];

    pub const LCC_1SP_VARIANT_B: [&ParamMapping; 6] = [
        &params::LAT_LCC_1SP,
        &params::SCALE_FACTOR,
        &params::LATITUDE_FALSE_ORIGIN,
        &params::LONGITUDE_FALSE_ORIGIN,
        &params::FALSE_EASTING_ORIGIN,
        &params::FALSE_NORTHING_ORIGIN,
    ];

    pub const LCC_2SP: [&ParamMapping; 6] = [
        &params::LATITUDE_FALSE_ORIGIN,
        &params::LONGITUDE_FALSE_ORIGIN,
        &params::LATITUDE_1ST_STD_PARALLEL,
        &params::LATITUDE_2ND_STD_PARALLEL,
        &params::FALSE_EASTING_ORIGIN,
        &params::FALSE_NORTHING_ORIGIN,
    ];

    pub const LCC_2SP_MICHIGAN: [&ParamMapping; 7] = [
        &params::LATITUDE_FALSE_ORIGIN,
        &params::LONGITUDE_FALSE_ORIGIN,
        &params::LATITUDE_1ST_STD_PARALLEL,
        &params::LATITUDE_2ND_STD_PARALLEL,
        &params::FALSE_EASTING_ORIGIN,
        &params::FALSE_NORTHING_ORIGIN,
        &params::ELLIPSOID_SCALE_FACTOR,
    ];

    pub const AEA: [&ParamMapping; 8] = [
        &params::LAT_FALSE_ORIGIN_LAT_OF_CENTER,
        &params::LONG_FALSE_ORIGIN_LONG_OF_CENTER,
        &params::LATITUDE_1ST_STD_PARALLEL,
        &params::LATITUDE_2ND_STD_PARALLEL,
        &params::FALSE_EASTING_ORIGIN,
        &params::FALSE_NORTHING_ORIGIN,
        &params::ESRI_LATITUDE_OF_ORIGIN,
        &params::ESRI_CENTRAL_MERIDIAN,
    ];

    pub const LAEA: [&ParamMapping; 6] = [
        &params::LAT_NAT_LAT_CENTER,
        &params::LONG_NAT_LONG_CENTER,
        &params::FALSE_EASTING,
        &params::FALSE_NORTHING,
        &params::ESRI_LATITUDE_OF_ORIGIN,
        &params::ESRI_CENTRAL_MERIDIAN,
    ];

    pub const MERC_1SP: [&ParamMapping; 5] = [
        &params::LAT_MERC_1SP,
        &params::LONGITUDE_NAT_ORIGIN,
        &params::SCALE_FACTOR_K,
        &params::FALSE_EASTING,
        &params::FALSE_NORTHING,
    ];

    pub const MERC_2SP: [&ParamMapping; 4] = [
        &params::LAT_1ST_PARALLEL_LAT_TS,
        &params::LONGITUDE_NAT_ORIGIN,
        &params::FALSE_EASTING,
        &params::FALSE_NORTHING,
    ];

    pub const POLAR_STEREO: [&ParamMapping; 5] = [
        &params::LAT_STD_PARALLEL,
        &params::LONG_ORIGIN,
        &params::FALSE_EASTING,
        &params::FALSE_NORTHING,
        &params::ESRI_STANDARD_PARALLEL_1_LAT_TS,
    ];

    pub const OBLIQUE_STEREO: [&ParamMapping; 5] = [
        &params::LATITUDE_NAT_ORIGIN,
        &params::LONGITUDE_NAT_ORIGIN,
        &params::SCALE_FACTOR_K,
        &params::FALSE_EASTING,
        &params::FALSE_NORTHING,
    ];

    pub const LONG_NAT_ORIGIN_LONGITUDE_CENTRE: [&ParamMapping; 4] = [
        &params::LONG_NAT_LONG_CENTER,
        &params::FALSE_EASTING,
        &params::FALSE_NORTHING,
        &params::ESRI_CENTRAL_MERIDIAN,
    ];

    pub const EQC: [&ParamMapping; 5] = [
        &params::LAT_1ST_PARALLEL_LAT_TS,
        &params::LATITUDE_NAT_ORIGIN,
        &params::LONGITUDE_NAT_ORIGIN,
        &params::FALSE_EASTING,
        &params::FALSE_NORTHING,
    ];

    pub const CEA: [&ParamMapping; 4] = [
        &params::LAT_1ST_PARALLEL_LAT_TS,
        &params::LONGITUDE_NAT_ORIGIN,
        &params::FALSE_EASTING,
        &params::FALSE_NORTHING,
    ];

    // Natural origin, with ESRI names
    pub const NAT_ORIGIN_ESRI_CENTER: [&ParamMapping; 6] = [
        &params::LATITUDE_NAT_ORIGIN,
        &params::LONGITUDE_NAT_ORIGIN,
        &params::FALSE_EASTING,
        &params::FALSE_NORTHING,
        &params::ESRI_LATITUDE_OF_CENTER,
        &params::ESRI_LONGITUDE_OF_CENTER,
    ];

    pub const ESRI_LOCAL: [&ParamMapping; 6] = [
        &params::FALSE_EASTING,
        &params::FALSE_NORTHING,
        &params::SCALE_FACTOR,
        &params::AZIMUTH,
        &params::ESRI_LATITUDE_OF_CENTER,
        &params::ESRI_LONGITUDE_OF_CENTER,
    ];

    pub const HOM_VARIANT_A: [&ParamMapping; 8] = [
        &params::LAT_CENTRE_LAT_CENTER,
        &params::LON_CENTRE_LON_CENTER_LONC,
        &params::AZIMUTH,
        &params::ANGLE_TO_SKEW_GRID,
        &params::SCALE_FACTOR_INITIALLINE,
        &params::FALSE_EASTING,
        &params::FALSE_NORTHING,
        &params::ESRI_XY_PLANE_ROTATION,
    ];

    pub const HOM_VARIANT_B: [&ParamMapping; 8] = [
        &params::LAT_CENTRE_LAT_CENTER,
        &params::LON_CENTRE_LON_CENTER_LONC,
        &params::AZIMUTH,
        &params::ANGLE_TO_SKEW_GRID,
        &params::SCALE_FACTOR_INITIALLINE,
        &params::FALSE_EASTING_PROJECTION_CENTRE,
        &params::FALSE_NORTHING_PROJECTION_CENTRE,
        &params::ESRI_XY_PLANE_ROTATION,
    ];

    pub const HOM_TWO_POINT: [&ParamMapping; 12] = [
        &params::LAT_CENTRE_LAT_CENTER,
        &params::LAT_POINT_1,
        &params::LONG_POINT_1,
        &params::LAT_POINT_2,
        &params::LONG_POINT_2,
        &params::SCALE_FACTOR_INITIALLINE,
        &params::FALSE_EASTING_PROJECTION_CENTRE,
        &params::FALSE_NORTHING_PROJECTION_CENTRE,
        &params::ESRI_LATITUDE_OF_1ST_POINT,
        &params::ESRI_LONGITUDE_OF_1ST_POINT,
        &params::ESRI_LATITUDE_OF_2ND_POINT,
        &params::ESRI_LONGITUDE_OF_2ND_POINT,
    ];

    pub const SOMERC: [&ParamMapping; 5] = [
        &params::LAT_CENTRE_LAT_CENTER,
        &params::LON_CENTRE_LON_0,
        &params::SCALE_FACTOR_INITIAL_LINE_K_0,
        &params::FALSE_EASTING_PROJECTION_CENTRE,
        &params::FALSE_NORTHING_PROJECTION_CENTRE,
    ];

    pub const GEOS: [&ParamMapping; 6] = [
        &params::LONGITUDE_NAT_ORIGIN,
        &params::SATELLITE_HEIGHT,
        &params::FALSE_EASTING,
        &params::FALSE_NORTHING,
        &params::ESRI_LONGITUDE_OF_CENTER,
        &params::ESRI_HEIGHT,
    ];

    pub const NSPER: [&ParamMapping; 8] = [
        &params::LAT_TOPOCENTRIC_ORIGIN,
        &params::LON_TOPOCENTRIC_ORIGIN,
        &params::VIEWPOINT_HEIGHT,
        &params::FALSE_EASTING,
        &params::FALSE_NORTHING,
        &params::ESRI_LATITUDE_OF_CENTER,
        &params::ESRI_LONGITUDE_OF_CENTER,
        &params::ESRI_HEIGHT,
    ];

    pub const KROVAK_PARAMETERS: [&ParamMapping; 7] = [
        &params::LAT_CENTRE_LAT_CENTER,
        &params::LONG_CENTRE_LONG_CENTER,
        &params::COLATITUDE_CONE_AXIS,
        &params::LATITUDE_PSEUDO_STD_PARALLEL,
        &params::SCALE_FACTOR_PSEUDO_STD_PARALLEL,
        &params::FALSE_EASTING,
        &params::FALSE_NORTHING,
    ];
}

const WKT1_POLAR_STEREOGRAPHIC: &str = "Polar_Stereographic";
const WKT1_KROVAK: &str = "Krovak";

pub const METHOD_MAPPINGS: &[MethodMapping] = &[
    method! {TRANSVERSE_MERCATOR, "Transverse_Mercator", "tmerc", "", &parameters::NAT_ORIGIN_SCALE_K},
    // no mapping to WKT1
    method! {TRANSVERSE_MERCATOR_3D, "", "tmerc", "", &parameters::NAT_ORIGIN_SCALE_K},
    method! {TRANSVERSE_MERCATOR_SOUTH_ORIENTATED, "Transverse_Mercator_South_Orientated", "tmerc", "+axis=wsu",
    &parameters::NAT_ORIGIN_SCALE_K},
    method! {ALBERS_EQUAL_AREA, "Albers_Conic_Equal_Area", "aea", "", &parameters::AEA},
    method! {LAMBERT_CONIC_CONFORMAL_1SP, "Lambert_Conformal_Conic_1SP", "lcc", "", &parameters::LCC_1SP},
    // no mapping to WKT1
    method! {LAMBERT_CONIC_CONFORMAL_1SP_VARIANT_B, "", "lcc", "", &parameters::LCC_1SP_VARIANT_B},
    method! {LAMBERT_CONIC_CONFORMAL_2SP, "Lambert_Conformal_Conic_2SP", "lcc", "",
    &parameters::LCC_2SP},
    // no mapping to WKT1
    method! {LAMBERT_CONIC_CONFORMAL_2SP_MICHIGAN, "", "lcc", "", &parameters::LCC_2SP_MICHIGAN},
    method! {LAMBERT_CONIC_CONFORMAL_2SP_BELGIUM, "Lambert_Conformal_Conic_2SP_Belgium", "lcc", "",
    &parameters::LCC_2SP},
    method! {LAMBERT_AZIMUTHAL_EQUAL_AREA, "Lambert_Azimuthal_Equal_Area", "laea", "", &parameters::LAEA},
    method! {LAMBERT_AZIMUTHAL_EQUAL_AREA_SPHERICAL, "Lambert_Azimuthal_Equal_Area", "laea", "+R_A",
    &parameters::LAEA},
    method! {MERCATOR_VARIANT_A, "Mercator_1SP", "merc", "", &parameters::MERC_1SP},
    method! {MERCATOR_VARIANT_B, "Mercator_2SP", "merc", "", &parameters::MERC_2SP},
    method! {POPULAR_VISUALISATION_PSEUDO_MERCATOR, "Popular_Visualisation_Pseudo_Mercator", "webmerc", "",
    &parameters::NAT_ORIGIN},
    method! {PROJ_WKT2_NAME_MOLLWEIDE, "Mollweide", "moll", "", &parameters::LONG_NAT_ORIGIN},
    method! {PROJ_WKT2_NAME_WAGNER_IV, "Wagner_IV", "wag4", "", &parameters::LONG_NAT_ORIGIN},
    method! {PROJ_WKT2_NAME_WAGNER_V, "Wagner_V", "wag5", "", &parameters::LONG_NAT_ORIGIN},
    method! {OBLIQUE_STEREOGRAPHIC, "Oblique_Stereographic", "sterea", "",
    &parameters::OBLIQUE_STEREO},
    method! {POLAR_STEREOGRAPHIC_VARIANT_A, WKT1_POLAR_STEREOGRAPHIC, "stere", "", &parameters::OBLIQUE_STEREO},
    method! {POLAR_STEREOGRAPHIC_VARIANT_B, WKT1_POLAR_STEREOGRAPHIC, "stere", "", &parameters::POLAR_STEREO},
    method! {PROJ_WKT2_NAME_METHOD_STEREOGRAPHIC, "Stereographic", "stere", "", &parameters::OBLIQUE_STEREO},
    method! {PROJ_WKT2_NAME_METHOD_ROBINSON, "Robinson", "robin", "", &parameters::LONG_NAT_ORIGIN_LONGITUDE_CENTRE},
    method! {PROJ_WKT2_NAME_METHOD_SINUSOIDAL, "Sinusoidal", "sinu", "", &parameters::LONG_NAT_ORIGIN_LONGITUDE_CENTRE},
    method! {KROVAK, WKT1_KROVAK, "krovak", "+axis=swu", &parameters::KROVAK_PARAMETERS},
    method! {EPSG_METHOD_KROVAK_NORTH_ORIENTED, WKT1_KROVAK, "krovak", "", &parameters::KROVAK_PARAMETERS},
    method! {EQUIDISTANT_CYLINDRICAL, "Equirectangular", "eqc", "", &parameters::EQC},
    method! {EQUIDISTANT_CYLINDRICAL_SPHERICAL, "", "eqc", "", &parameters::EQC},
    // Same parameters as LAEA
    method! {AZIMUTHAL_EQUIDISTANT, "Azimuthal_Equidistant", "aeqd", "", &parameters::LAEA},
    method! {MODIFIED_AZIMUTHAL_EQUIDISTANT, "", "aeqd", "", &parameters::LAEA},
    method! {GUAM_PROJECTION, "", "aeqd", "+guam", &parameters::LAEA},
    method! {PROJ_WKT2_NAME_METHOD_MILLER_CYLINDRICAL, "Miller_Cylindrical", "mill", "",
    &parameters::LONG_NAT_ORIGIN_LONGITUDE_CENTRE},
    method! {LAMBERT_CYLINDRICAL_EQUAL_AREA, "Cylindrical_Equal_Area", "cea", "", &parameters::CEA},
    method! {LAMBERT_CYLINDRICAL_EQUAL_AREA_SPHERICAL, "", "cea", "", &parameters::CEA},
    // Standard parallel is always 30° (as in PROJ)
    method! {ESRI_BEHRMANN, "Behrmann", "cea", "+lat_ts=30", &parameters::LONG_NAT_ORIGIN},
    method! {AMERICAN_POLYCONIC, "Polyconic", "poly", "", &parameters::NAT_ORIGIN},
    method! {ORTHOGRAPHIC, "Orthographic", "ortho", "", &parameters::NAT_ORIGIN},
    // ESRI 'Orthographic' is spherical (as in PROJ)
    method! {PROJ_WKT2_NAME_METHOD_ORTHOGRAPHIC_SPHERICAL, "", "ortho", "",
    &parameters::NAT_ORIGIN_ESRI_CENTER, true},
    method! {ESRI_LOCAL, "Local", "ortho", "", &parameters::ESRI_LOCAL},
    method! {PROJ_WKT2_NAME_METHOD_GNOMONIC, "Gnomonic", "gnom", "", &parameters::NAT_ORIGIN_ESRI_CENTER},
    method! {CASSINI_SOLDNER, "Cassini_Soldner", "cass", "", &parameters::NAT_ORIGIN},
    method! {HYPERBOLIC_CASSINI_SOLDNER, "", "cass", "+hyperbolic", &parameters::NAT_ORIGIN},
    // Same parameters as AEA
    method! {EQUIDISTANT_CONIC, "Equidistant_Conic", "eqdc", "", &parameters::AEA},
    method! {EQUAL_EARTH, "Equal_Earth", "eqearth", "", &parameters::LONG_NAT_ORIGIN},
    method! {HOTINE_OBLIQUE_MERCATOR_VARIANT_A, "Hotine_Oblique_Mercator", "omerc", "+no_uoff",
    &parameters::HOM_VARIANT_A},
    method! {HOTINE_OBLIQUE_MERCATOR_VARIANT_B, "Hotine_Oblique_Mercator_Azimuth_Center", "omerc", "",
    &parameters::HOM_VARIANT_B},
    method! {SWISS_OBLIQUE_CYLINDRICAL, "", "somerc", "", &parameters::SOMERC},
    method! {PROJ_WKT2_NAME_METHOD_HOTINE_OBLIQUE_MERCATOR_TWO_POINT_NATURAL_ORIGIN,
    "Hotine_Oblique_Mercator_Two_Point_Natural_Origin", "omerc", "", &parameters::HOM_TWO_POINT},
    method! {PROJ_WKT2_NAME_METHOD_GEOSTATIONARY_SATELLITE_SWEEP_Y, "Geostationary_Satellite", "geos", "",
    &parameters::GEOS},
    method! {PROJ_WKT2_NAME_METHOD_GEOSTATIONARY_SATELLITE_SWEEP_X, "", "geos", "+sweep=x", &parameters::GEOS},
    method! {VERTICAL_PERSPECTIVE, "Vertical_Near_Side_Perspective", "nsper", "", &parameters::NSPER},
];

/// EPSG code of an authority
fn epsg_code<'a>(authority: &Option<Authority<'a>>) -> Option<&'a str> {
    authority
        .as_ref()
        .filter(|auth| auth.name.eq_ignore_ascii_case("EPSG"))
        .map(|auth| auth.code)
}

/// Retrieve method mapping from model
///
/// Trust EPSG code first if known, otherwise check name
pub fn find_method_mapping(projcs: &Projcs) -> Result<Option<&'static MethodMapping>> {
    let projection = &projcs.projection;
    let params = &projection.parameters;
    let geod_unit = projcs.geogcs.unit.as_ref();
    let me = &projection.method;
    let mapping = if me.name.is_empty() {
        None
    } else if let Some(m) = epsg_code(&me.authority)
        .and_then(|code| METHOD_MAPPINGS.iter().find(|m| m.epsg_code == code))
    {
        Some(m)
    } else if me.name.eq_ignore_ascii_case(WKT1_POLAR_STEREOGRAPHIC) {
        Some(wkt1_polar_stereographic(params, geod_unit)?)
    } else if me.name.eq_ignore_ascii_case(WKT1_KROVAK) {
        Some(krovak(params, &projcs.axis_directions)?)
    } else if me.name.eq_ignore_ascii_case(methods::ORTHOGRAPHIC.name) && is_esri(params) {
        Some(get_method_mapping(
            &methods::PROJ_WKT2_NAME_METHOD_ORTHOGRAPHIC_SPHERICAL,
        ))
    } else if let Some(m) = METHOD_MAPPINGS.iter().find(|m| {
        m.wkt2_name.eq_ignore_ascii_case(me.name) || m.wkt1_name.eq_ignore_ascii_case(me.name)
    }) {
        Some(m)
    } else {
        esri_method_mapping(me.name, params)?
    };

    match mapping {
        Some(m)
            if m.is(&methods::HOTINE_OBLIQUE_MERCATOR_VARIANT_B)
                && is_swiss_oblique_mercator(m, params, geod_unit)? =>
        {
            Ok(Some(get_method_mapping(
                &methods::SWISS_OBLIQUE_CYLINDRICAL,
            )))
        }
        // GDAL WKT1 'Geostationary_Satellite' defines the sweep axis
        // in the PROJ4 extension only (as in PROJ)
        Some(m)
            if m.is(&methods::PROJ_WKT2_NAME_METHOD_GEOSTATIONARY_SATELLITE_SWEEP_Y)
                && projcs
                    .proj4_extension
                    .is_some_and(|ext| ext.split_whitespace().any(|t| t == "+sweep=x")) =>
        {
            Ok(Some(get_method_mapping(
                &methods::PROJ_WKT2_NAME_METHOD_GEOSTATIONARY_SATELLITE_SWEEP_X,
            )))
        }
        m => Ok(m),
    }
}

/// ESRI WKT1 names the projection centre 'Longitude_Of_Center'
/// and 'Latitude_Of_Center'
fn is_esri(params: &[Parameter]) -> bool {
    params.iter().any(|p| {
        p.name == params::ESRI_LONGITUDE_OF_CENTER.wkt1_name
            || p.name == params::ESRI_LATITUDE_OF_CENTER.wkt1_name
    })
}

/// Hotine Oblique Mercator (variant B) with an azimuth and an angle
/// from the rectified to the skew grid of 90° is the Swiss Oblique
/// Mercator (as in PROJ). The angle defaults to the azimuth.
fn is_swiss_oblique_mercator(
    mapping: &MethodMapping,
    params: &[Parameter],
    geod_unit: Option<&Unit>,
) -> Result<bool> {
    let find_param = |proj_name: &str| {
        params.iter().find(|p| {
            mapping
                .find_proj_params(p)
                .any(|pm| pm.proj_name == proj_name)
        })
    };
    let is_right_angle = |p: &Parameter| {
        angle(p, geod_unit).map(|a| (a - std::f64::consts::FRAC_PI_2).abs() < 1.0e-10)
    };

    match find_param(params::AZIMUTH.proj_name) {
        Some(azimuth) => match find_param(params::ANGLE_TO_SKEW_GRID.proj_name) {
            Some(gamma) => Ok(is_right_angle(azimuth)? && is_right_angle(gamma)?),
            None => is_right_angle(azimuth),
        },
        None => Ok(false),
    }
}

/// Angular parameter value in radians
fn angle(p: &Parameter, geod_unit: Option<&Unit>) -> Result<f64> {
    let value = parse_number(p.value)?;
    Ok(match p.unit.as_ref().or(geod_unit) {
        Some(unit) => value * unit.factor,
        None => value.to_radians(),
    })
}

/// 'Krovak' without EPSG code may be north orientated:
/// - ESRI WKT1 defines the orientation with 'X_Scale', 'Y_Scale'
///   and 'XY_Plane_Rotation', any other values than the default
///   1, 1, 0 are north orientated.
/// - Otherwise only south-west axes are not north orientated
///   (as in PROJ).
fn krovak(params: &[Parameter], axis_directions: &[&str]) -> Result<&'static MethodMapping> {
    let find_param = |name: &str| params.iter().find(|p| p.name.eq_ignore_ascii_case(name));
    let value = |p: Option<&Parameter>, default| p.map_or(Ok(default), |p| parse_number(p.value));

    let x_scale = find_param("X_Scale");
    let y_scale = find_param("Y_Scale");
    let rotation = find_param("XY_Plane_Rotation");

    let north_orientated = if x_scale.is_some() || y_scale.is_some() || rotation.is_some() {
        (
            value(x_scale, 1.0)?,
            value(y_scale, 1.0)?,
            value(rotation, 0.0)?,
        ) != (1.0, 1.0, 0.0)
    } else {
        !matches!(axis_directions, [x, y]
            if x.eq_ignore_ascii_case("south") && y.eq_ignore_ascii_case("west"))
    };

    Ok(get_method_mapping(if north_orientated {
        &methods::EPSG_METHOD_KROVAK_NORTH_ORIENTED
    } else {
        &methods::KROVAK
    }))
}

fn get_method_mapping(method: &Wkt2Const) -> &'static MethodMapping {
    METHOD_MAPPINGS.iter().find(|m| m.is(method)).unwrap()
}

// ESRI WKT1 projection names
// See PROJ /src/iso19111/operation/esriparammappings.cpp
const ESRI_METHODS: &[(&str, &Wkt2Const)] = &[
    ("Albers", &methods::ALBERS_EQUAL_AREA),
    ("Cassini", &methods::CASSINI_SOLDNER),
    ("Double_Stereographic", &methods::OBLIQUE_STEREOGRAPHIC),
    ("Equidistant_Cylindrical", &methods::EQUIDISTANT_CYLINDRICAL),
    (
        "Equidistant_Cylindrical_Ellipsoidal",
        &methods::EQUIDISTANT_CYLINDRICAL,
    ),
    ("Gauss_Kruger", &methods::TRANSVERSE_MERCATOR),
    (
        "Hotine_Oblique_Mercator_Azimuth_Natural_Origin",
        &methods::HOTINE_OBLIQUE_MERCATOR_VARIANT_A,
    ),
    ("Plate_Carree", &methods::EQUIDISTANT_CYLINDRICAL),
    // ESRI exact Transverse Mercator: PROJ default algorithm
    ("Transverse_Mercator_Complex", &methods::TRANSVERSE_MERCATOR),
    (
        "Rectified_Skew_Orthomorphic_Center",
        &methods::HOTINE_OBLIQUE_MERCATOR_VARIANT_B,
    ),
    (
        "Rectified_Skew_Orthomorphic_Natural_Origin",
        &methods::HOTINE_OBLIQUE_MERCATOR_VARIANT_A,
    ),
    ("Mercator", &methods::MERCATOR_VARIANT_B),
    (
        "Mercator_Auxiliary_Sphere",
        &methods::POPULAR_VISUALISATION_PSEUDO_MERCATOR,
    ),
    (
        "Polar_Stereographic_Variant_A",
        &methods::POLAR_STEREOGRAPHIC_VARIANT_A,
    ),
    (
        "Stereographic_North_Pole",
        &methods::POLAR_STEREOGRAPHIC_VARIANT_B,
    ),
    (
        "Stereographic_South_Pole",
        &methods::POLAR_STEREOGRAPHIC_VARIANT_B,
    ),
];

/// Retrieve method mapping from ESRI WKT1 projection name
fn esri_method_mapping(name: &str, params: &[Parameter]) -> Result<Option<&'static MethodMapping>> {
    let find_param = |param: &str| params.iter().find(|p| p.name.eq_ignore_ascii_case(param));

    let method = if name.eq_ignore_ascii_case("Lambert_Conformal_Conic") {
        // LCC 2SP if there is a second standard parallel
        if find_param("Standard_Parallel_2").is_some() {
            &methods::LAMBERT_CONIC_CONFORMAL_2SP
        } else {
            &methods::LAMBERT_CONIC_CONFORMAL_1SP
        }
    } else if let Some((_, method)) = ESRI_METHODS
        .iter()
        .find(|(n, _)| n.eq_ignore_ascii_case(name))
    {
        *method
    } else {
        return Ok(None);
    };

    // Only the sphere with the WGS84 semi-major axis
    // is supported (as in PROJ)
    if let Some(p) = find_param("Auxiliary_Sphere_Type")
        && parse_number(p.value)? != 0.0
    {
        return Err(Error::Wkt(
            format!("Unsupported Auxiliary_Sphere_Type {}", p.value).into(),
        ));
    }

    Ok(Some(get_method_mapping(method)))
}

/// WKT1 'Polar_Stereographic' is used for both variants:
/// the latitude of origin is the pole for variant A, otherwise
/// it is the latitude of true scale of variant B.
fn wkt1_polar_stereographic(
    params: &[Parameter],
    geod_unit: Option<&Unit>,
) -> Result<&'static MethodMapping> {
    let lat_origin = params.iter().find(|p| {
        p.name
            .eq_ignore_ascii_case(params::LAT_STD_PARALLEL.wkt1_name)
    });

    let variant = match lat_origin {
        Some(p) => {
            if (angle(p, geod_unit)?.abs() - std::f64::consts::FRAC_PI_2).abs() < 1.0e-10 {
                &methods::POLAR_STEREOGRAPHIC_VARIANT_A
            } else {
                &methods::POLAR_STEREOGRAPHIC_VARIANT_B
            }
        }
        None => &methods::POLAR_STEREOGRAPHIC_VARIANT_A,
    };

    Ok(get_method_mapping(variant))
}
