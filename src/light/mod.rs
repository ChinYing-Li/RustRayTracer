pub mod arealight;
pub mod ambientocc;

use cgmath::Vector3;

use crate::world::shaderec::ShadeRec;
use crate::utils::color::Colorf;
use std::fmt::{Debug, Formatter, Error};
use crate::ray::Ray;

pub mod directional;
pub mod pointlight;
pub mod ambient;

pub trait Light
{
    fn get_direction(&self, sr: &ShadeRec) -> Vector3<f32>;
    fn L(&self, sr: &ShadeRec) -> Colorf;
    fn does_cast_shadow(&self) -> bool;
    fn is_in_shadow(&self, sr: &ShadeRec, ray: &Ray) -> bool;
    fn get_type(&self) -> String {  String::from("") }
}

impl Debug for dyn Light
{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.debug_struct("Light")
            .finish()
    }
}