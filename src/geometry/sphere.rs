use std::{f32};
use cgmath::prelude::*;
use cgmath::{Vector3, dot};

use crate::geometry::Geometry;
use crate::ray::Ray;
use crate::utils::shaderec::ShadeRec;
use crate::math::polynomial::*;
use std::fmt;
use crate::utils::color::Colorf;
use crate::material::Material;
use std::sync::Arc;
use std::ops::Deref;

#[derive(Clone)]
pub struct Sphere<'a>
{
    pub m_radius: f32,
    pub m_center: Vector3<f32>,
    pub m_color: Colorf,
    pub m_material: Option<&'a Material>,
}

impl Sphere<'_>
{
    const KEPSILON: f32 = 0.0001;

    pub fn new<'a>(radius: f32, center: Vector3<f32>, color: Colorf) -> Sphere<'a>
    {
        Sphere
        {
            m_radius: radius,
            m_center: center,
            m_color: color,
            m_material: None
        }
    }

    pub fn setRadius(&mut self, newradius: f32)
    {
        self.m_radius = newradius;
    }

    pub fn setCenter(&mut self, newcenter: Vector3<f32>)
    {
        self.m_center = newcenter;
    }
}

impl fmt::Debug for Sphere<'_>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        f.debug_struct("Sphere")
            .field("radius", &self.m_radius)
            .field("center", &self.m_center)
            .finish()
    }
}

impl<'a> Geometry<'a> for Sphere<'a>
{
    fn hit(&self, incomeray: &Ray, tmin: &mut f32, shaderecord: &mut ShadeRec) -> bool
    {
        let temp = incomeray.m_origin - self.m_center;
        let a = dot(incomeray.m_velocity, incomeray.m_velocity);
        let b = 2.0 * dot(temp, incomeray.m_velocity);
        let c = dot(temp, temp) - self.m_radius.powf(2.0);
        let roots = getQuadraticPolyRoot(a, b, c);

        let mut res = false;
        for it in roots.iter()
        {
            if let Some(time) = it
            {
                if *time > Sphere::KEPSILON
                {
                    //c_updateShadeRecNormal(time);
                    shaderecord.m_normal = temp + *time * incomeray.m_velocity;
                    *tmin = *time;
                    res = true;
                    break;
                }
            }
        }
        res
    }

    fn getColor(&self) -> Colorf { self.m_color }

    fn setColor(&mut self, newcolor: Colorf) { self.m_color = newcolor; }

    fn getMaterial(&self) -> &'a Material
    {
        if let Some(x) = self.m_material { x }
        else { panic!("The material is Not set") }
    }

    fn setMaterial(&'a mut self, material: &'a Material)
    {
        self.m_material = Some(material.clone());
    }
}