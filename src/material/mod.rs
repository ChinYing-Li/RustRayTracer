pub mod dielectric;
pub mod glossyreflector;
pub mod reflector;
pub mod emissive;
pub mod matte;
pub mod phong;

use crate::world::shaderec::ShadeRec;
use crate::utils::color::Colorf;
use std::fmt::{Debug, Formatter};
use std::fmt;
use std::error::Error;
use crate::tracer::Tracer;

pub trait Material
{
    fn shade(&self, sr: &mut ShadeRec) -> Colorf;
    fn area_light_shade(&self, sr: &mut ShadeRec) -> Colorf;
}

impl Debug for Material
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("Material")
            .finish()
    }
}