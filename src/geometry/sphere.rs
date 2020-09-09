use std::{f32};
use cgmath::prelude::*;
use cgmath::{Vector3, dot};

use crate::geometry::{Geometry, KEPSILON, Shadable, GeomError, Boundable};
use crate::ray::Ray;
use crate::utils::shaderec::ShadeRec;
use crate::math::polynomial::*;
use std::fmt;
use crate::utils::color::Colorf;
use crate::material::Material;
use std::sync::Arc;
use std::ops::Deref;
use crate::geometry::bbox::BBox;
use std::f32::consts::SQRT_2;

#[derive(Clone)]
pub struct Sphere
{
    pub m_radius: f32,
    pub m_center: Vector3<f32>,
    pub m_color: Colorf,
    pub m_material: Option<Arc<dyn Material>>,
}

impl Sphere
{
    pub fn new(radius: f32, center: Vector3<f32>, color: Colorf) -> Sphere
    {
        Sphere
        {
            m_radius: radius,
            m_center: center,
            m_color: color,
            m_material: None
        }
    }

    pub fn set_radius(&mut self, newradius: f32)
    {
        self.m_radius = newradius;
    }

    pub fn set_center(&mut self, newcenter: Vector3<f32>)
    {
        self.m_center = newcenter;
    }
}

impl fmt::Debug for Sphere
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        f.debug_struct("Sphere")
            .field("radius", &self.m_radius)
            .field("center", &self.m_center)
            .finish()
    }
}

impl Geometry for Sphere
{
    fn hit(&self, incomeray: &Ray, tmin: &mut f32, shaderecord: &mut ShadeRec) -> Result<bool, GeomError>
    {
        let temp = incomeray.m_origin - self.m_center;
        let a = dot(incomeray.m_direction, incomeray.m_direction);
        let b = 2.0 * dot(temp, incomeray.m_direction);
        let c = dot(temp, temp) - self.m_radius.powf(2.0);
        let roots = get_quadratic_poly_root(a, b, c);

        let mut res = false;
        for it in roots.iter()
        {
            if let Some(time) = it
            {
                if *time > KEPSILON
                {
                    //c_updateShadeRecNormal(time);
                    shaderecord.m_normal = (temp + *time * incomeray.m_direction).normalize();
                    *tmin = *time;
                    res = true;
                    break;
                }
            }
        }
        Ok(res)
    }
}

impl Boundable for Sphere
{
    fn get_bbox(&self) -> BBox {
        let diag = self.m_radius * SQRT_2;
        let offset = Vector3::new(diag, diag, diag);
        BBox::new(self.m_center - offset, self.m_center + offset)
    }
}

impl Shadable for Sphere
{
    fn get_color(&self) -> Colorf { self.m_color }

    fn set_color(&mut self, newcolor: Colorf) { self.m_color = newcolor; }

    fn get_material(&self) -> Arc<dyn Material>
    {
        if let Some(x) = self.m_material.clone() { x }
        else { panic!("The material for sphere is Not set") }
    }
    fn set_material(&mut self, material: Arc<dyn Material>)
    {
        self.m_material = Some(material.clone());
    }

    fn shadow_hit(&self, shadowray: &Ray, tmin: &mut f32) -> bool {
        let temp = shadowray.m_origin - self.m_center;
        let a = dot(shadowray.m_direction, shadowray.m_direction);
        let b = 2.0 * dot(temp, shadowray.m_direction);
        let c = dot(temp, temp) - self.m_radius.powf(2.0);
        let roots = get_quadratic_poly_root(a, b, c);

        for it in roots.iter()
        {
            if let Some(time) = it
            {
                if *time > KEPSILON
                {
                    *tmin = *time;
                    println!("shadow hit!");
                    return true;
                }
            }
        }
        false
    }
}
