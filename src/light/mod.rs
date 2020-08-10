use cgmath::Vector3;

use crate::utils::shaderec::ShadeRec;
use crate::utils::color::Colorf;
use std::fmt::{Debug, Formatter, Error};

mod directional;
mod pointlight;
pub mod ambient;

pub trait Light
{
    fn getDirection(&self, sr: &mut ShadeRec) -> Vector3<f32>;
    fn L(&self, sr: &mut ShadeRec) -> Colorf;
}

impl Debug for Light
{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.debug_struct("Light")
            .finish()
    }
}