use std::fmt;
use cgmath::prelude::*;

use crate::{ray::Ray,
            utils::{shaderec::ShadeRec, color::Colorf}};

pub mod sphere;

/// This trait

pub trait Geometry: fmt::Debug
{
    fn hit(&self, incomeray: &Ray, time: &mut f32, shaderecord: &mut ShadeRec) -> bool;
    fn getColor(&self)->Colorf;
    //fn updateShadeRec(t: f32, &mut tmin: f32, &mut shaderecord: ShadeRec);
}