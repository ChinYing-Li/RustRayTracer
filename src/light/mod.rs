use cgmath::Vector3;

use crate::utils::shaderec::ShadeRec;
use crate::utils::color::Colorf;
use std::fmt::{Debug, Formatter, Error};

mod directional;
mod pointlight;
pub mod ambient;

pub trait Light<'a>
{
    fn getDirection(&self, sr: &'a mut ShadeRec) -> Vector3<f32>;
    fn L(&self, sr: &'a mut ShadeRec) -> Colorf;
}

impl Debug for Light<'_>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.debug_struct("Light")
            .finish()
    }
}