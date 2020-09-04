pub mod instance;
pub mod trimesh;
pub mod triangle;
pub mod cuboid;
pub mod bbox;

use std::fmt;
use cgmath::prelude::*;

use crate::{ray::Ray,
            utils::{shaderec::ShadeRec, color::Colorf}};
use crate::material::Material;
use std::sync::Arc;
use std::ops::{Deref, DerefMut};
use crate::utils::colorconstant::COLOR_BLACK;

pub mod sphere;
pub const KEPSILON: f32 = 0.0001;
/// This trait


#[derive(Debug)]
pub enum GeomError
{
    WrongSizeError,
    NoSolutionError,
}

impl std::error::Error for GeomError {}

impl fmt::Display for GeomError
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self
        {
            GeomError::WrongSizeError => write!(f, "Calculation can't be done "),
            GeomError::NoSolutionError => write!(f, ""),
        }
    }
}

pub trait Geometry: fmt::Debug
{
    fn hit(&self, incomeray: &Ray, time: &mut f32, shaderecord: &mut ShadeRec) -> Result<bool, GeomError>;

}

pub trait Shadable
{
    fn get_color(&self) -> Colorf { COLOR_BLACK }
    fn set_color(&mut self, newcolor: Colorf) {}
    fn get_material(&self) -> Arc<dyn Material>;
    fn set_material(&mut self, material: Arc<dyn Material>) {}
    fn shadow_hit(&self, ray: &Ray, tmin: &mut f32) -> bool;
}

pub trait Concrete: Geometry + Shadable {}
impl<T> Concrete for T where T: Geometry + Shadable {}
