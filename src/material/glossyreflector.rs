use cgmath::{Vector3, Zero, InnerSpace};
use std::sync::Arc;

use crate::utils::color::Colorf;
use crate::world::shaderec::ShadeRec;
use crate::ray::Ray;
use crate::material::Material;
use crate::material::phong::Phong;
use crate::tracer::Tracer;
use crate::brdf::{BRDF,
                  perfectspec::PerfectSpecular,
                  lambertian::Lambertian,
                  glossyspec::GlossySpecular};

pub struct GlossyReflector
{
    pub m_phong: Arc<Phong>,
    pub m_glossyspec_brdf: Arc<GlossySpecular>
}

impl GlossyReflector
{
    pub fn new(phong: Arc<Phong>, glossyspec_brdf: Arc<GlossySpecular>) -> GlossyReflector
    {
        GlossyReflector
        {
            m_phong: phong,
            m_glossyspec_brdf: glossyspec_brdf,
        }
    }
}

impl Material for GlossyReflector
{
    fn shade(&self, sr: &mut ShadeRec) -> Colorf {
        let mut clr = self.m_phong.shade(sr);
        let mut w_o = -sr.m_ray.m_direction.normalize();
        let mut w_i = Vector3::zero();
        let mut pdf = 0.0_f32;

        let fr = self.m_glossyspec_brdf.sampleFunc(sr, &mut w_i, &mut w_o, &mut pdf);
        let reflected_ray = Ray::new(sr.m_hitpoint, w_i);

        clr += fr * sr.m_worldptr.m_tracer.trace_ray(sr.m_worldptr.clone(), &reflected_ray, sr.m_depth +1)
            * sr.m_normal.normalize().dot(w_i) / pdf;
        clr
    }

    fn area_light_shade(&self, sr: &mut ShadeRec) -> Colorf
    {
        let mut clr = self.m_phong.area_light_shade(sr);
        let mut w_o = -sr.m_ray.m_direction;
        let mut w_i = Vector3::zero();
        let mut pdf = 0.0_f32;

        let fr = self.m_glossyspec_brdf.sampleFunc(sr,&mut w_i, &mut w_o, &mut pdf);
        let reflected_ray = Ray::new(sr.m_hitpoint, w_i);

        clr += fr * sr.m_worldptr.clone().m_tracer.trace_ray(sr.m_worldptr.clone(), &reflected_ray, sr.m_depth +1)
            * sr.m_normal.dot(w_i) / pdf;
        clr
    }
}