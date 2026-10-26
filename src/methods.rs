//!
//! Method mapping
//!
use crate::builder::parse_number;
use crate::consts::{Wkt2Const, methods};
use crate::errors::{Error, Result};
use crate::model::{Parameter, Projection, Unit};
use crate::params::{self, ParamMapping};

pub struct MethodMapping {
    wkt2_name: &'static str,
    epsg_code: &'static str,
    wkt1_name: &'static str,
    proj_name: &'static str,
    proj_aux: &'static str,
    param_mapping: &'static [&'static ParamMapping],
}

impl MethodMapping {
    pub fn proj_name(&self) -> &'static str {
        self.proj_name
    }

    pub fn proj_aux(&self) -> &'static str {
        self.proj_aux
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
        self.param_mapping.iter().copied().filter(move |pp| {
            !p.name.is_empty()
                && !pp.proj_name.is_empty()
                && match &p.authority {
                    Some(auth) => auth.name == "EPSG" && pp.epsg_code == auth.code,
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
        MethodMapping {
            wkt2_name: methods::$wkt2.name,
            epsg_code: methods::$wkt2.code,
            wkt1_name: $wkt1_name,
            proj_name: $proj_name,
            proj_aux: $proj_aux,
            param_mapping: $mapping,
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

pub const METHOD_MAPPINGS: [MethodMapping; 22] = [
    method! {TRANSVERSE_MERCATOR, "Transverse_Mercator", "tmerc", "", &parameters::NAT_ORIGIN_SCALE_K},
    method! {TRANSVERSE_MERCATOR_SOUTH_ORIENTATED, "Transverse_Mercator_South_Orientated", "tmerc", "+axis=wsu",
    &parameters::NAT_ORIGIN_SCALE_K},
    method! {ALBERS_EQUAL_AREA, "Albers_Conic_Equal_Area", "aea", "", &parameters::AEA},
    method! {LAMBERT_CONIC_CONFORMAL_1SP, "Lambert_Conformal_Conic_1SP", "lcc", "", &parameters::LCC_1SP},
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
    method! {EPSG_METHOD_KROVAK_NORTH_ORIENTED, "Krovak", "krovak", "", &parameters::KROVAK_PARAMETERS},
];

/// Retrieve method mapping from model
///
/// Trust EPSG code first if available, otherwise check name
pub fn find_method_mapping(
    projection: &Projection,
    geod_unit: Option<&Unit>,
) -> Result<Option<&'static MethodMapping>> {
    let me = &projection.method;
    if me.name.is_empty() {
        Ok(None)
    } else if let Some(auth) = &me.authority {
        Ok(METHOD_MAPPINGS
            .iter()
            .find(|m| auth.name == "EPSG" && m.epsg_code == auth.code))
    } else if me.name.eq_ignore_ascii_case(WKT1_POLAR_STEREOGRAPHIC) {
        wkt1_polar_stereographic(&projection.parameters, geod_unit).map(Some)
    } else if let Some(m) = METHOD_MAPPINGS.iter().find(|m| {
        m.wkt2_name.eq_ignore_ascii_case(me.name) || m.wkt1_name.eq_ignore_ascii_case(me.name)
    }) {
        Ok(Some(m))
    } else {
        esri_method_mapping(me.name, &projection.parameters)
    }
}

fn get_method_mapping(method: &Wkt2Const) -> &'static MethodMapping {
    METHOD_MAPPINGS.iter().find(|m| m.is(method)).unwrap()
}

// ESRI WKT1 projection names
// See PROJ /src/iso19111/operation/esriparammappings.cpp
const ESRI_METHODS: [(&str, &Wkt2Const); 8] = [
    ("Albers", &methods::ALBERS_EQUAL_AREA),
    ("Double_Stereographic", &methods::OBLIQUE_STEREOGRAPHIC),
    ("Gauss_Kruger", &methods::TRANSVERSE_MERCATOR),
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
            let lat = parse_number(p.value)?;
            let lat = match p.unit.as_ref().or(geod_unit) {
                Some(unit) => lat * unit.factor,
                None => lat.to_radians(),
            };
            if (lat.abs() - std::f64::consts::FRAC_PI_2).abs() < 1.0e-10 {
                &methods::POLAR_STEREOGRAPHIC_VARIANT_A
            } else {
                &methods::POLAR_STEREOGRAPHIC_VARIANT_B
            }
        }
        None => &methods::POLAR_STEREOGRAPHIC_VARIANT_A,
    };

    Ok(get_method_mapping(variant))
}
