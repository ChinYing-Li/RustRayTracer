use std::{f32};
use cgmath::prelude::*;
use cgmath::{Vector3, dot};

use crate::geometry::{Geometry, KEPSILON, Shadable, GeomError, Boundable};
use crate::ray::Ray;
use crate::world::shaderec::ShadeRec;
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
    pub m_material: Option<Arc<dyn Material>>,
}

impl Sphere
{
    pub fn new(radius: f32, center: Vector3<f32>) -> Sphere
    {
        Sphere
        {
            m_radius: radius,
            m_center: center,
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
                if *time > KEPSILON && *tmin > *time
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
    fn get_material(&self) -> Arc<dyn Material>
    {
        if let Some(x) = self.m_material.clone() { x }
        else { panic!("The material for sphere is not set") }
    }

    fn set_material(&mut self, material: Arc<dyn Material>)
    {
        self.m_material = Some(material);
    }

    fn shadow_hit(&self, shadowray: &Ray, tmin: &mut f32) -> bool {
        let temp = shadowray.m_origin - self.m_center;
        let a = dot(shadowray.m_direction, shadowray.m_direction);
        let b = 2.0 * dot(temp, shadowray.m_direction);
        let c = dot(temp, temp) - self.m_radius.powf(2.0);
        let roots = get_quadratic_poly_root(a, b, c);

        for it in roots.iter()
        {
            if let Some(root) = it
            {
                if *root > KEPSILON
                {
                    *tmin = *root;
                    println!("shadow hit!");
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod TestSphere
{
    use super::*;
    use std::f32::consts::PI;
    use approx::{assert_relative_eq};
    use crate::utils::colorconstant::COLOR_RED;

    const INV_PI: f32 = 1.0 / PI ;
    const INV_GAMMA: f32 = 1.0 / 1.8;

    #[test]
    pub fn TestSphereBBox()
    {
        let sphere = Sphere::new(10.0, Vector3::new(20.0, 10.0, 30.0));
        let bbox = sphere.get_bbox();
        assert_relative_eq!(bbox.m_vertex_0.x, 5.85786437626);
        assert_relative_eq!(bbox.m_vertex_0.y, -4.1421356237);
        assert_relative_eq!(bbox.m_vertex_1.z, 44.1421356237);
    }
}