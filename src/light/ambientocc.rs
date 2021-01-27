use cgmath::{Vector3, InnerSpace, ElementWise};
use std::sync::{Arc, RwLock};
use std::f32::INFINITY;
use std::cell::RefCell;

use crate::light::Light;
use crate::utils::color::Colorf;
use crate::world::shaderec::ShadeRec;
use crate::ray::Ray;
use crate::sampler::Sampler;
use crate::utils::colorconstant::COLOR_BLACK;

pub struct AmbientOccluder
{
    pub m_u: RwLock<Vector3<f32>>,
    pub m_v: RwLock<Vector3<f32>>,
    pub m_w: RwLock<Vector3<f32>>,
    m_color: Colorf,
    m_ls: f32,
    pub m_min_color: Colorf,
    m_samplerptr: Arc<dyn Sampler>,
}

impl AmbientOccluder
{
    pub fn new(min_color: Colorf, ls: f32, samplerptr: Arc<dyn Sampler>) -> AmbientOccluder
    {
        AmbientOccluder
        {
            m_u: RwLock::new(Vector3::unit_x()),
            m_v: RwLock::new(Vector3::unit_y()),
            m_w: RwLock::new(Vector3::unit_z()),
            m_color: COLOR_BLACK,
            m_ls: ls,
            m_min_color: min_color,
            m_samplerptr: samplerptr,
        }
    }
    pub fn set_sampler(&mut self, sampler: Arc<dyn Sampler>)
    {
        self.m_samplerptr = sampler;
    }

    pub fn set_color(&mut self, color: Colorf)
    {
        self.m_color = color;
    }
}

impl Light for AmbientOccluder
{
    fn get_direction(&self, sr: &ShadeRec) -> Vector3<f32>
    {
        let sample = self.m_samplerptr.as_ref().get_hemisphere_sample();
        let result = (self.m_u.read().unwrap().mul_element_wise(sample.x)
            + self.m_v.read().unwrap().mul_element_wise(sample.y )
            + self.m_w.read().unwrap().mul_element_wise(sample.z )).normalize();
        //println!("{}, {}, {}", result.x, result.y, result.z);
        result
    }

    fn L(&self, sr: &ShadeRec) -> Colorf
    {
        *self.m_w.write().unwrap() = sr.m_normal;
        let jittered_up = Vector3::new(0.00031, 1.0, 0.00021).normalize();
        *self.m_v.write().unwrap() = self.m_w.read().unwrap().cross(jittered_up).normalize();
        *self.m_u.write().unwrap() = self.m_v.read().unwrap().cross(*self.m_w.read().unwrap()).normalize();

        let shadow_ray = Ray::new(sr.m_hitpoint, self.get_direction(sr));
        if self.is_in_shadow(sr, &shadow_ray)
        {
            return self.m_color * self.m_min_color * self.m_ls;
        }
        self.m_color * self.m_ls
    }

    fn does_cast_shadow(&self) -> bool {
        true
    }

    fn is_in_shadow(&self, sr: &ShadeRec, ray: &Ray) -> bool
    {
        let mut time = INFINITY;
        for object in sr.m_worldptr.as_ref().m_objects.iter()
        {
            if object.lock().unwrap().shadow_hit(ray, &mut time)
            {
                return true
            }
        }
        false
    }

    fn get_type(&self) -> String
    {
        String::from("AmbientOccluder")
    }
}