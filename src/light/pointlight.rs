use cgmath::{Vector3, MetricSpace};
use std::f32::INFINITY;

use crate::light::Light;
use crate::utils::color::Colorf;
use crate::world::shaderec::ShadeRec;
use crate::cgmath::InnerSpace;
use crate::ray::Ray;

#[derive(Debug)]
pub struct PointLight
{
    m_ls: f32, // Radiance scaling factor
    m_color: Colorf,
    m_location: Vector3<f32>
}

impl PointLight
{
    pub fn new(ls: f32, color: Colorf, location: Vector3<f32>) -> PointLight
    {
        PointLight{ m_ls: ls, m_color: color, m_location: location}
    }

    pub fn set_radiance_scaling_factor(&mut self, new_ls: f32)
    {
        self.m_ls = new_ls;
    }

    pub fn set_color(&mut self, newcolor: Colorf)
    {
        self.m_color = newcolor;
    }
}

impl Light for PointLight
{
    fn get_direction(&self, sr: &ShadeRec) -> Vector3<f32>
    {
        (self.m_location - sr.m_hitpoint).normalize()
    }

    fn L(&self, sr: &ShadeRec) -> Colorf
    {
        self.m_color * self.m_ls
    }

    fn does_cast_shadow(&self) -> bool { true }

    fn is_in_shadow(&self, sr: &ShadeRec, ray: &Ray) -> bool
    {
        let disance_to_shadowed = self.m_location.distance(ray.m_origin);
        let world_ptr = sr.m_worldptr.clone();
        let mut t = INFINITY;

        for i in 0..world_ptr.m_objects.len()
        {
            if world_ptr.m_objects[i].lock().unwrap()
                .shadow_hit(ray, &mut t)
                && t < disance_to_shadowed
            { return true }
        }
        false
    }
}