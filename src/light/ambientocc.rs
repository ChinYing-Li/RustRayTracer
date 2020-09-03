// TODO: Ambient Occluder

use crate::light::Light;
use crate::utils::color::Colorf;
use crate::utils::shaderec::ShadeRec;
use cgmath::Vector3;
use crate::ray::Ray;
use crate::sampler::Sampler;
use std::sync::Arc;
use std::f32::INFINITY;

pub struct AmbientOccluder
{
    pub m_u: Vector3<f32>,
    pub m_v: Vector3<f32>,
    pub m_w: Vector3<f32>,
    pub m_min_amount: Colorf,
    m_samplerptr: Option<Arc<dyn Sampler>>,
}

impl AmbientOccluder
{
    pub fn new(u: Vector3<f32>, v: Vector3<f32>, w: Vector3<f32>, min_amount: Colorf) -> AmbientOccluder
    {
        AmbientOccluder
        {
            m_u: u, m_v: v, m_w: w,
            m_min_amount: min_amount,
            m_samplerptr: None,
        }
    }
    pub fn set_sampler(&mut self, sampler: Arc<dyn Sampler>)
    {
        self.m_samplerptr = Some(sampler);
    }
}

impl Light for AmbientOccluder
{
    fn get_direction(&self, sr: &mut ShadeRec) -> Vector3<f32>
    {
        unimplemented!()
    }

    fn L(&self, sr: &mut ShadeRec) -> Colorf {
        unimplemented!()
    }

    fn does_cast_shadow(&self) -> bool {
        true
    }

    fn is_in_shadow(&self, sr: &ShadeRec, ray: &Ray) -> bool
    {
        let mut time = INFINITY;
        for object in &sr.m_worldptr.clone().unwrap().m_objects
        {
            if object.lock().unwrap().shadow_hit(ray, &mut time)
            {
                return true
            }
        }
        false
    }
}