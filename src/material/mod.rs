pub mod dielectric;
pub mod glossyreflector;
pub mod reflector;
pub mod emissive;
pub mod matte;
pub mod phong;

use std::fmt;
use std::error::Error;

use crate::world::shaderec::ShadeRec;
use crate::utils::color::Colorf;

pub trait Material: Send + Sync
{
    fn shade(&self, sr: &mut ShadeRec) -> Colorf;
    fn area_light_shade(&self, sr: &mut ShadeRec) -> Colorf;
}

impl fmt::Debug for dyn Material
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Material")
            .finish()
    }
}