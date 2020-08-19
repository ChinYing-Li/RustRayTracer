use std::fmt;
use cgmath::prelude::*;

use crate::{ray::Ray,
            utils::{shaderec::ShadeRec, color::Colorf}};
use crate::material::Material;
use std::sync::Arc;
use std::ops::{Deref, DerefMut};

pub mod sphere;

/// This trait

pub trait Geometry<'a>: fmt::Debug
{
    fn hit(&self, incomeray: &Ray, time: &mut f32, shaderecord: &mut ShadeRec) -> bool;

    fn getColor(&self)->Colorf;
    fn setColor(&mut self, newcolor: Colorf);

    fn getMaterial(&self) -> &'a Material;
    fn setMaterial(&'a mut self, material: &'a Material);
}