use cgmath::prelude::*;

use crate::ray::Ray;
use crate::utils::shaderec::ShadeRec;
use std::fmt;
use crate::utils::color::Color;

pub mod sphere;

/// This trait

pub trait Geometry: fmt::Debug
{
    fn hit(&self, incomeray: &Ray, time: &mut f32, shaderecord: &mut ShadeRec) -> bool;
    fn getColor(&self)->Color;
    //fn updateShadeRec(t: f32, &mut tmin: f32, &mut shaderecord: ShadeRec);
}