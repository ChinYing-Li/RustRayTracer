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

#[derive(Clone, Debug)]
pub struct Reflective
{
    m_phong: Arc<Phong>,
    m_reflective_brdf: Arc<PerfectSpecular>,
}

impl Reflective
{
    pub fn new(phong: Arc<Phong>,
                kr: f32,
                cr: Colorf)
    -> Reflective
    {
        Reflective
        {
            m_phong: phong,
            m_reflective_brdf: Arc::new(PerfectSpecular::new(kr, cr))
        }
    }
}

impl Material for Reflective
{
    fn shade(&self, sr: &mut ShadeRec) -> Colorf
    {
        let mut clr = self.m_phong.shade(sr);
        let mut w_o = -sr.m_ray.m_direction;
        let mut w_i = Vector3::zero();
        let mut dummy = 0.0;
        let f_reflect = self.m_reflective_brdf.sampleFunc(sr, &mut w_i, &mut w_o, &mut dummy);
        let reflected_ray = Ray::new(sr.m_hitpoint, w_i);

        // TODO: Holy crap we are using the tracer of world here!!!
        clr += f_reflect * sr.m_worldptr.as_ref().m_tracer.trace_ray(sr.m_worldptr.clone(), &reflected_ray, sr.m_depth +1)
            * sr.m_normal.dot(w_i);
        clr
    }

    fn area_light_shade(&self, sr: &mut ShadeRec) -> Colorf
    {
        unimplemented!()
    }
}