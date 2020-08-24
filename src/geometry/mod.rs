use std::fmt;
use cgmath::prelude::*;

use crate::{ray::Ray,
            utils::{shaderec::ShadeRec, color::Colorf}};
use crate::material::Material;
use std::sync::Arc;
use std::ops::{Deref, DerefMut};

pub mod sphere;

/// This trait

pub trait Geometry: fmt::Debug
{
    fn hit(&self, incomeray: &Ray, time: &mut f32, shaderecord: &mut ShadeRec) -> bool;

    fn get_color(&self) ->Colorf;
    fn set_color(&mut self, newcolor: Colorf);

    fn get_material(&self) -> Arc<dyn Material>;
    fn set_material(&mut self, material: Arc<dyn Material>);
}