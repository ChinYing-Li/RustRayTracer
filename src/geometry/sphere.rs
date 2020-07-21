use std::{f32};
use cgmath::prelude::*;
use cgmath::{Vector3, dot};

use crate::geometry::Geometry;
use crate::ray::Ray;
use crate::utils::shaderec::ShadeRec;
use crate::math::polynomial::*;
use std::fmt;

#[derive(Clone, Copy, PartialEq)]
pub struct Sphere
{
    m_radius: f32,
    m_center: Vector3<f32>,
}

impl Sphere
{
    const KEPSILON: f32 = 0.0001;

    pub fn new(radius: f32, center: Vector3<f32>) -> Sphere
    {
        Sphere{m_radius: radius, m_center: center}
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
    fn hit(&self, incomeray: &mut Ray, tmin: &mut f32, shaderecord: &mut ShadeRec) -> bool
    {
        let temp = incomeray.m_origin - self.m_center;
        let a = dot(incomeray.m_velocity, incomeray.m_velocity);
        let b = 2.0 * dot(temp, incomeray.m_velocity);
        let c = dot(temp, temp) - self.m_radius.powf(2.0);
        let roots = getQuadraticPolyRoot(a, b, c);

        let mut c_updateShadeRec = |time: &f32|
            {
                *tmin = *time;
                shaderecord.m_ishitting = true;
                shaderecord.m_normal = temp + *time * incomeray.m_velocity;
                shaderecord.m_hitpoint = incomeray.m_origin + *time * incomeray.m_velocity;
            };

        let mut res = false;
        for it in roots.iter()
        {
            if let Some(time) = it
            {
                if *time > Sphere::KEPSILON
                {
                    c_updateShadeRec(time);
                    res = true;
                    break;
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod SphereTest
{
    use super::*;
    use crate::geometry::sphere::Sphere;
    use cgmath::Vector3;
    use crate::ray::Ray;
    use crate::utils::shaderec::ShadeRec;

    #[test]
    fn checkHit()
    {
        let sphere = Sphere::new(5.0, Vector3::new(0.0, 0.0, 0.0));
        let mut ray = Ray::new(Vector3::new(10.0, 3.0, 0.0),
                               Vector3::new(-1.0, 0.0, 0.0));
        let mut shaderecord = ShadeRec::new();
        let mut tmin = 100.0;

        let res = sphere.hit(&mut ray, &mut tmin, &mut shaderecord);
        println!("time is {}", tmin);
        assert!(res);
        assert_eq!(tmin, 6.0);
        assert_eq!(shaderecord.m_hitpoint, Vector3::new(4.0, 3.0, 0.0));
    }

    #[test]
    fn checkNoHit()
    {
        let sphere = Sphere::new(5.0, Vector3::new(0.0, 0.0, 0.0));
        let mut ray = Ray::new(Vector3::new(7.0, 0.5, 0.0), Vector3::new(-3.0, 3.0, 0.0));
        let mut shaderecord = ShadeRec::new();
        let mut tmin = 100.0;
        let res = sphere.hit(&mut ray, &mut tmin, &mut shaderecord);
        assert!(!res);
    }
}