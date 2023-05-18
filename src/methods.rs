//!
//! Method mapping
//!
use crate::consts::methods;
use crate::model::Parameter;
use crate::params::ParamMapping;

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

    /// Look up for mapped proj parameter
    pub fn find_proj_param(&self, p: &Parameter) -> Option<&ParamMapping> {
        if p.name.is_empty() {
            None
        } else if let Some(auth) = &p.authority {
            if auth.name == "EPSG" {
                self.param_mapping
                    .iter()
                    .find(|pp| !pp.proj_name.is_empty() && pp.epsg_code == auth.code)
            } else {
                None
            }
        } else {
            self.param_mapping.iter().find(|pp| {
                pp.wkt2_name.eq_ignore_ascii_case(p.name)
                    || pp.wkt1_name.eq_ignore_ascii_case(p.name)
            })
        }
        .copied()
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

    pub const LCC_1SP: [&ParamMapping; 5] = [
        &params::LAT_LCC_1SP,
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

    pub const AEA: [&ParamMapping; 6] = [
        &params::LAT_FALSE_ORIGIN_LAT_OF_CENTER,
        &params::LONG_FALSE_ORIGIN_LONG_OF_CENTER,
        &params::LATITUDE_1ST_STD_PARALLEL,
        &params::LATITUDE_2ND_STD_PARALLEL,
        &params::FALSE_EASTING_ORIGIN,
        &params::FALSE_NORTHING_ORIGIN,
    ];

    pub const LAEA: [&ParamMapping; 4] = [
        &params::LAT_NAT_LAT_CENTER,
        &params::LONG_NAT_LONG_CENTER,
        &params::FALSE_EASTING,
        &params::FALSE_NORTHING,
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

    pub const POLAR_STEREO: [&ParamMapping; 4] = [
        &params::LAT_STD_PARALLEL,
        &params::LONG_ORIGIN,
        &params::FALSE_EASTING,
        &params::FALSE_NORTHING,
    ];

    pub const OBLIQUE_STEREO: [&ParamMapping; 5] = [
        &params::LATITUDE_NAT_ORIGIN,
        &params::LONGITUDE_NAT_ORIGIN,
        &params::SCALE_FACTOR_K,
        &params::FALSE_EASTING,
        &params::FALSE_NORTHING,
    ];
}

pub const METHOD_MAPPINGS: [MethodMapping; 19] = [
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
    method! {POLAR_STEREOGRAPHIC_VARIANT_A, "Polar_Stereographic", "stere", "", &parameters::OBLIQUE_STEREO},
    method! {POLAR_STEREOGRAPHIC_VARIANT_B, "Polar_Stereographic", "stere", "", &parameters::POLAR_STEREO},
    method! {PROJ_WKT2_NAME_METHOD_STEREOGRAPHIC, "Stereographic", "stere", "", &parameters::OBLIQUE_STEREO},
];

use crate::model::Method;

/// Retrieve method mappinf from model
///
/// Trust EPSG code first if available, otherwise check name
pub fn find_method_mapping(me: &Method) -> Option<&'static MethodMapping> {
    if me.name.is_empty() {
        None
    } else if let Some(auth) = &me.authority {
        METHOD_MAPPINGS
            .iter()
            .find(|m| auth.name == "EPSG" && m.epsg_code == auth.code)
    } else {
        METHOD_MAPPINGS.iter().find(|m| {
            m.wkt2_name.eq_ignore_ascii_case(me.name) || m.wkt1_name.eq_ignore_ascii_case(me.name)
        })
    }
}
