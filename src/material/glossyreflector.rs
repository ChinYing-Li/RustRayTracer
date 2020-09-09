use crate::material::phong::Phong;
use crate::brdf::glossyspec::GlossySpecular;
use std::sync::Arc;
use crate::material::Material;
use crate::utils::color::Colorf;
use crate::utils::shaderec::ShadeRec;
use crate::ray::Ray;
use crate::brdf::BRDF;
use cgmath::{Vector3, Zero, InnerSpace};
use crate::tracer::Tracer;
use crate::utils::colorconstant::COLOR_BLACK;

pub struct GlossyReflector
{
    pub m_phong: Phong,
    pub m_glossyspec_brdf: Arc<GlossySpecular>
}

impl GlossyReflector
{
    pub fn new(phong: &Phong, glossyspec_brdf: Arc<GlossySpecular>) -> GlossyReflector
    {
        GlossyReflector
        {
            m_phong: (*phong).clone(),          m_glossyspec_brdf: glossyspec_brdf
        }
    }

    pub fn set_samples(&mut self)
    {
        unimplemented!()
    }

    pub fn set_kr(&mut self, kr: f32)
    {
        self.m_glossyspec_brdf.set_ks(kr);
    }

    pub fn set_exponent(&mut self, e: f32)
    {
        self.m_glossyspec_brdf.set_exponent(e);
    }
}

impl Material for GlossyReflector
{
    fn shade(&self, sr: &mut ShadeRec) -> Colorf {
        unimplemented!()
    }

    fn area_light_shade(&self, sr: &mut ShadeRec) -> Colorf
    {
        let mut clr = self.m_phong.area_light_shade(sr);
        let mut w_o = -sr.m_ray.m_direction;
        let mut w_i = Vector3::zero();
        let mut pdf = 0.0_f32;
        let fr = self.m_glossyspec_brdf.sampleFunc(sr,&mut w_i, &mut w_o, &mut pdf);
        let reflected_ray = Ray::new(sr.m_hitpoint, w_i);
        /*
        clr += fr * tracer.unwrap().trace_ray(sr.m_worldptr.unwrap(), &reflected_ray, sr.m_depth +1)
            * sr.m_normal.dot(w_i) / pdf;*/
        clr
    }
}